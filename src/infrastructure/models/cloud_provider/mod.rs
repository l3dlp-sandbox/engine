use std::any::Any;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use aws_types::SdkConfig;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::cmd::docker::Docker;
use crate::cmd::helm::{to_engine_error, Helm};
use crate::environment::models::abort::Abort;
use crate::environment::models::environment::Environment;
use crate::environment::report::logger::EnvLogger;
use crate::errors::EngineError;
use crate::events::{EnvironmentStep, EventDetails, Stage, Transmitter};
use crate::infrastructure::infrastructure_context::InfrastructureContext;
use crate::infrastructure::models::cloud_provider::service::Service;
use crate::infrastructure::models::container_registry::ContainerRegistry;
use crate::infrastructure::models::dns_provider::DnsProvider;
use crate::infrastructure::models::kubernetes;
use crate::infrastructure::models::kubernetes::Kubernetes;
use crate::io_models::context::Context;
use crate::logger::Logger;
use crate::metrics_registry::MetricsRegistry;
use crate::services::kube_client::QubeClient;

pub mod aws;
pub mod gcp;
pub mod io;
pub mod scaleway;
pub mod self_managed;
pub mod service;

pub trait CloudProvider: Send + Sync {
    fn context(&self) -> &Context;
    fn kind(&self) -> Kind;
    fn kubernetes_kind(&self) -> kubernetes::Kind;
    fn id(&self) -> &str;
    fn organization_id(&self) -> &str;
    fn organization_long_id(&self) -> uuid::Uuid;
    fn name(&self) -> &str;
    fn name_with_id(&self) -> String {
        format!("{} ({})", self.name(), self.id())
    }
    fn access_key_id(&self) -> String;
    fn secret_access_key(&self) -> String;
    fn region(&self) -> String;
    // TODO(benjaminch): Remove client from here
    fn aws_sdk_client(&self) -> Option<SdkConfig>;
    fn is_valid(&self) -> Result<(), Box<EngineError>>;
    fn zones(&self) -> Vec<String>;
    /// environment variables containing credentials
    fn credentials_environment_variables(&self) -> Vec<(&str, &str)>;
    /// environment variables to inject to generate Terraform files from templates
    fn tera_context_environment_variables(&self) -> Vec<(&str, &str)>;
    fn terraform_state_credentials(&self) -> Option<&TerraformStateCredentials>;
    fn as_any(&self) -> &dyn Any;
    fn get_event_details(&self, stage: Stage) -> EventDetails;
    fn to_transmitter(&self) -> Transmitter;
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Kind {
    Aws,
    Scw,
    Gcp,
    OnPremise,
}

impl FromStr for Kind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "aws" | "amazon" => Ok(Kind::Aws),
            "scw" | "scaleway" => Ok(Kind::Scw),
            "gcp" | "google" => Ok(Kind::Gcp),
            "on-premise" | "onpremise" => Ok(Kind::OnPremise),
            _ => Err(()),
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Kind::Aws => "AWS",
            Kind::Scw => "Scaleway",
            Kind::Gcp => "GCP",
            Kind::OnPremise => "OnPremise",
        })
    }
}

pub trait CloudProviderZones {}

#[derive(Default)]
pub struct TerraformStateCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub region: String,
    pub s3_bucket: String,
    pub dynamodb_table: String,
}

impl TerraformStateCredentials {
    pub fn new(
        access_key_id: &str,
        secret_access_key: &str,
        region: &str,
        s3_bucket: &str,
        dynamodb_table: &str,
    ) -> Self {
        TerraformStateCredentials {
            access_key_id: access_key_id.to_string(),
            secret_access_key: secret_access_key.to_string(),
            region: region.to_string(),
            s3_bucket: s3_bucket.to_string(),
            dynamodb_table: dynamodb_table.to_string(),
        }
    }
}

pub struct DeploymentTarget<'a> {
    pub kubernetes: &'a dyn Kubernetes,
    pub container_registry: &'a dyn ContainerRegistry,
    pub cloud_provider: &'a dyn CloudProvider,
    pub dns_provider: &'a dyn DnsProvider,
    pub environment: &'a Environment,
    pub docker: &'a Docker,
    pub kube: kube::Client,
    pub helm: Helm,
    pub abort: &'a dyn Abort,
    logger: Arc<Box<dyn Logger>>,
    pub metrics_registry: Arc<dyn MetricsRegistry>,
    pub is_dry_run_deploy: bool,
    pub is_test_cluster: bool,
}

impl<'a> DeploymentTarget<'a> {
    pub fn new(
        infra_ctx: &'a InfrastructureContext,
        environment: &'a Environment,
        abort: &'a dyn Abort,
    ) -> Result<DeploymentTarget<'a>, Box<EngineError>> {
        let event_details = environment.event_details();
        let kubernetes = infra_ctx.kubernetes();
        let kubeconfig_path = {
            let kubeconfig_path = kubernetes.kubeconfig_local_file_path();
            if kubeconfig_path.exists() {
                Some(kubeconfig_path)
            } else {
                None
            }
        };

        let helm = if let Some(kubeconfig_path) = &kubeconfig_path {
            Helm::new(
                Some(kubeconfig_path),
                &infra_ctx.cloud_provider().credentials_environment_variables(),
            )
            .map_err(|e| to_engine_error(event_details, e))?
        } else {
            Helm::new(Option::<&Path>::None, &[]).map_err(|e| to_engine_error(event_details, e))?
        };

        Ok(DeploymentTarget {
            kubernetes,
            container_registry: infra_ctx.container_registry(),
            cloud_provider: infra_ctx.cloud_provider(),
            dns_provider: infra_ctx.dns_provider(),
            environment,
            docker: &infra_ctx.context().docker,
            kube: infra_ctx.mk_kube_client()?.client().clone(),
            helm,
            abort,
            logger: Arc::new(infra_ctx.kubernetes().logger().clone_dyn()),
            is_dry_run_deploy: kubernetes.context().is_dry_run_deploy(),
            is_test_cluster: kubernetes.context().is_test_cluster(),
            metrics_registry: Arc::from(infra_ctx.metrics_registry().clone_dyn()),
        })
    }

    pub fn env_logger(&self, service: &impl Service, step: EnvironmentStep) -> EnvLogger {
        EnvLogger::new(service, step, self.logger.clone())
    }

    pub fn qube_client(&self, event_details: EventDetails) -> Result<QubeClient, Box<EngineError>> {
        QubeClient::new(
            event_details,
            Some(self.kubernetes.kubeconfig_local_file_path()),
            self.cloud_provider
                .credentials_environment_variables()
                .iter()
                .map(|(x, y)| (x.to_string(), y.to_string()))
                .collect_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::models::cloud_provider::Kind;

    #[test]
    fn test_provider_kind_from_str() {
        // setup:
        let test_cases = vec![
            ("", Err(())),
            (" ", Err(())),
            ("aws", Ok(Kind::Aws)),
            ("amazon", Ok(Kind::Aws)),
            (" aws ", Ok(Kind::Aws)),
            (" amazon ", Ok(Kind::Aws)),
            ("AWS ", Ok(Kind::Aws)),
            ("amaZon", Ok(Kind::Aws)),
            ("amazon_blabla", Err(())),
            ("scw", Ok(Kind::Scw)),
            ("scaleway", Ok(Kind::Scw)),
            (" scw ", Ok(Kind::Scw)),
            (" scaleway ", Ok(Kind::Scw)),
            ("SCW ", Ok(Kind::Scw)),
            ("Scw", Ok(Kind::Scw)),
            ("scw_blabla", Err(())),
            ("gcp", Ok(Kind::Gcp)),
            ("google", Ok(Kind::Gcp)),
            (" gcp ", Ok(Kind::Gcp)),
            (" google ", Ok(Kind::Gcp)),
            ("GCP ", Ok(Kind::Gcp)),
            ("Gcp", Ok(Kind::Gcp)),
            ("gcp_blabla", Err(())),
        ];

        for tc in test_cases {
            // execute:
            let result: Result<Kind, ()> = tc.0.parse();

            // verify:
            assert_eq!(tc.1, result);
        }
    }
}
