use crate::helpers::common::Infrastructure;
use crate::helpers::database::StorageSize::Resize;
use crate::helpers::utilities::engine_run_test;
use crate::kube::{TestEnvOption, kube_test_env};
use base64::Engine;
use base64::engine::general_purpose;
use function_name::named;
use k8s_openapi::api::core::v1::PersistentVolumeClaim;
use qovery_engine::environment::action::update_pvcs;
use qovery_engine::environment::models::abort::AbortStatus;
use qovery_engine::environment::models::aws::{AwsAppExtraSettings, AwsStorageType};
use qovery_engine::environment::models::container::{Container, get_container_with_invalid_storage_size};
use qovery_engine::environment::models::registry_image_source::RegistryImageSource;
use qovery_engine::environment::models::types::AWS;
use qovery_engine::infrastructure::models::cloud_provider::DeploymentTarget;
use qovery_engine::infrastructure::models::cloud_provider::io::RegistryMirroringMode;
use qovery_engine::infrastructure::models::cloud_provider::service::ServiceType;
use qovery_engine::io_models::context::CloneForTest;
use qovery_engine::io_models::models::{
    EnvironmentVariable, KubernetesCpuResourceUnit, KubernetesMemoryResourceUnit, Storage,
};
use qovery_engine::io_models::variable_utils::VariableInfo;
use qovery_engine::io_models::{Action, MountedFile, QoveryIdentifier};
use qovery_engine::kubers_utils::kube_get_resources_by_selector;
use qovery_engine::runtime::block_on;
use std::collections::{BTreeMap, BTreeSet};
use tracing::{Level, span};

#[cfg(feature = "test-aws-self-hosted")]
#[test]
#[named]
fn should_increase_container_storage_size() {
    let test_name = function_name!();

    engine_run_test(|| {
        let span = span!(Level::INFO, "test", name = test_name);
        let _enter = span.enter();

        let (infra_ctx, environment) = kube_test_env(TestEnvOption::WithContainer);
        let ea = environment.clone();

        assert!(environment.deploy_environment(&ea, &infra_ctx).is_ok());

        let mut resized_env = environment.clone();
        resized_env.containers[0].storages[0].size_in_gib = Resize.size();
        let resized_container = &resized_env.containers[0];

        let resized_context = infra_ctx.context().clone_not_same_execution_id();
        let test_env = resized_env
            .to_environment_domain(
                &resized_context,
                infra_ctx.cloud_provider(),
                infra_ctx.container_registry(),
                infra_ctx.kubernetes(),
            )
            .unwrap();
        let deployment_target = DeploymentTarget::new(&infra_ctx, &test_env, &|| AbortStatus::None).unwrap();
        let test_container = &test_env.containers[0];

        let storages = resized_container
            .storages
            .iter()
            .map(|storage| storage.to_storage())
            .collect::<Vec<Storage>>();

        let envs = resized_container
            .environment_vars_with_infos
            .iter()
            .map(|(k, variable_infos)| EnvironmentVariable {
                key: k.to_string(),
                value: variable_infos.value.to_string(),
                is_secret: variable_infos.is_secret,
            })
            .collect::<Vec<EnvironmentVariable>>();
        let container: Container<AWS> = Container::new(
            &resized_context,
            resized_container.long_id,
            resized_container.name.clone(),
            resized_container.name.clone(),
            *test_container.action(),
            RegistryImageSource {
                registry: resized_container.registry.clone(),
                image: resized_container.image.clone(),
                tag: resized_container.tag.clone(),
                registry_mirroring_mode: RegistryMirroringMode::Service,
            },
            resized_container.command_args.clone(),
            resized_container.entrypoint.clone(),
            KubernetesCpuResourceUnit::MilliCpu(resized_container.cpu_request_in_milli),
            KubernetesCpuResourceUnit::MilliCpu(resized_container.cpu_limit_in_milli),
            KubernetesMemoryResourceUnit::MebiByte(resized_container.ram_request_in_mib),
            KubernetesMemoryResourceUnit::MebiByte(resized_container.ram_limit_in_mib),
            resized_container.min_instances,
            resized_container.max_instances,
            resized_container.public_domain.clone(),
            resized_container.ports.clone(),
            storages,
            envs,
            BTreeSet::default(),
            resized_container.readiness_probe.clone().map(|p| p.to_domain()),
            resized_container.liveness_probe.clone().map(|p| p.to_domain()),
            resized_container.advanced_settings.clone(),
            AwsAppExtraSettings {},
            |transmitter| infra_ctx.context().get_event_details(transmitter),
            vec![],
            vec![],
        )
        .expect("Unable to create container");

        let invalid_statefulset = match get_container_with_invalid_storage_size(
            &container,
            &deployment_target.kube,
            deployment_target.environment.namespace(),
            deployment_target.environment.event_details(),
        ) {
            Ok(result) => match result {
                Some(invalid_storage) => {
                    assert_eq!(invalid_storage.service_type, ServiceType::Container);
                    assert_eq!(invalid_storage.service_id, test_container.long_id().clone());
                    assert_eq!(invalid_storage.invalid_pvcs.len(), 1);
                    assert_eq!(invalid_storage.invalid_pvcs[0].required_disk_size_in_gib, Resize.size());
                    assert!(
                        invalid_storage.invalid_pvcs[0]
                            .pvc_name
                            .starts_with(&resized_env.containers[0].storages[0].long_id.to_string())
                    );
                    invalid_storage
                }
                None => panic!("No invalid storage returned"),
            },
            Err(e) => panic!("No invalid storage returned: {e}"),
        };

        let ret = update_pvcs(
            test_container.as_service(),
            &invalid_statefulset,
            test_env.namespace(),
            test_env.event_details(),
            &deployment_target.kube,
        );
        assert!(ret.is_ok());

        //assert app can be redeployed
        let rea = resized_env.clone();
        assert!(resized_env.deploy_environment(&rea, &infra_ctx).is_ok());

        // assert edited storage have good size
        let pvcs = match block_on(kube_get_resources_by_selector::<PersistentVolumeClaim>(
            &deployment_target.kube,
            deployment_target.environment.namespace(),
            &invalid_statefulset.statefulset_selector,
        )) {
            Ok(result) => result.items,
            Err(_) => panic!("Unable to get pvcs"),
        };

        let pvc = pvcs
            .iter()
            .find(|pvc| match &pvc.metadata.name {
                Some(name) => *name.to_string() == invalid_statefulset.invalid_pvcs[0].pvc_name,
                None => false,
            })
            .expect("Unable to get pvc");

        if let Some(spec) = &pvc.spec {
            if let Some(resources) = &spec.resources {
                if let Some(req) = &resources.requests {
                    assert_eq!(
                        req["storage"].0,
                        format!("{}Gi", invalid_statefulset.invalid_pvcs[0].required_disk_size_in_gib)
                    )
                }
            }
        }

        // clean up
        let mut env_to_delete = environment;
        env_to_delete.action = Action::Delete;
        let ead = env_to_delete.clone();
        assert!(env_to_delete.delete_environment(&ead, &infra_ctx).is_ok());

        test_name.to_string()
    });
}

#[cfg(feature = "test-aws-minimal")]
#[test]
#[named]
fn should_have_mounted_files_as_volume() {
    let test_name = function_name!();

    engine_run_test(|| {
        // setup:
        let span = span!(Level::INFO, "test", name = test_name);
        let _enter = span.enter();

        let (infra_ctx, environment) = kube_test_env(TestEnvOption::WithContainer);
        let mut ea = environment.clone();
        let mut container = environment
            .containers
            .first()
            .expect("there is no container in env")
            .clone();

        // removing useless objects for this test
        ea.applications = vec![];
        ea.databases = vec![];
        ea.jobs = vec![];
        ea.routers = vec![];

        // setup mounted file for this app
        let mounted_file_id = QoveryIdentifier::new_random();
        let mounted_file = MountedFile {
            id: mounted_file_id.short().to_string(),
            long_id: mounted_file_id.to_uuid(),
            mount_path: "/tmp/app.config.json".to_string(),
            file_content_b64: general_purpose::STANDARD.encode(r#"{"name": "config"}"#),
        };
        let mount_file_env_var_key = "APP_CONFIG";
        let mount_file_env_var_value = mounted_file.mount_path.to_string();

        // Use an app crashing in case file doesn't exists
        container.image = "r3m4q3r9/pub-mirror-debian".to_string();
        container.tag = "11.6-ci".to_string();
        container.command_args = vec![
            "/bin/sh".to_string(),
            "-c".to_string(),
            "apt-get update; apt-get install -y netcat-openbsd; echo listening on port $PORT; env ; while test -f $APP_CONFIG; do nc -l 8080; done".to_string(),
        ];
        //container.mounted_files = vec![mounted_file];
        container.environment_vars_with_infos = BTreeMap::from([
            (
                mount_file_env_var_key.to_string(),
                VariableInfo {
                    value: general_purpose::STANDARD.encode(mount_file_env_var_value),
                    is_secret: false,
                },
            ), // <- mounted file PATH
        ]);
        container.mounted_files = vec![mounted_file];

        // Create a statefulset
        let mut statefulset = container.clone();
        let statefulset_id = QoveryIdentifier::new_random();
        statefulset.name = statefulset_id.short().to_string();
        statefulset.kube_name.clone_from(&statefulset.name);
        statefulset.long_id = statefulset_id.to_uuid();
        let storage_id = QoveryIdentifier::new_random();
        statefulset.storages = vec![qovery_engine::io_models::application::Storage {
            id: storage_id.short().to_string(),
            long_id: storage_id.to_uuid(),
            name: storage_id.short().to_string(),
            storage_class: AwsStorageType::GP2.to_k8s_storage_class(),
            size_in_gib: 10,
            mount_point: format!("/tmp/{}", storage_id.short()),
            snapshot_retention_in_days: 1,
        }];

        // attaching application & statefulset to env
        ea.containers = vec![container, statefulset];

        // execute & verify
        let deployment_result = environment.deploy_environment(&ea, &infra_ctx);

        // verify:
        assert!(deployment_result.is_ok());

        // clean up:
        let mut env_to_delete = environment;
        env_to_delete.action = Action::Delete;
        let ead = env_to_delete.clone();
        assert!(env_to_delete.delete_environment(&ead, &infra_ctx).is_ok());

        test_name.to_string()
    });
}
