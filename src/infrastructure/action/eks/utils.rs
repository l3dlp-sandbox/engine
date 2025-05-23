use crate::environment::models::kubernetes::K8sPod;
use crate::errors::{CommandError, EngineError};
use crate::events::EventDetails;
use crate::infrastructure::action::eks::{
    AWS_EKS_DEFAULT_UPGRADE_TIMEOUT_DURATION, AWS_EKS_MAX_NODE_DRAIN_TIMEOUT_DURATION,
};
use crate::infrastructure::models::cloud_provider::CloudProvider;
use crate::infrastructure::models::cloud_provider::aws::new_rusoto_creds;
use crate::infrastructure::models::kubernetes::Kubernetes;
use crate::io_models::models::KubernetesClusterAction;
use chrono::Duration as ChronoDuration;
use rusoto_core::{Client, HttpClient, Region as RusotoRegion};
use rusoto_eks::EksClient;
use std::str::FromStr;

/// Returns a rusoto eks client using the current configuration.
pub fn get_rusoto_eks_client(
    event_details: EventDetails,
    kubernetes: &dyn Kubernetes,
    cloud_provider: &dyn CloudProvider,
) -> Result<EksClient, Box<EngineError>> {
    let region = match RusotoRegion::from_str(kubernetes.region()) {
        Ok(value) => value,
        Err(error) => {
            return Err(Box::new(EngineError::new_unsupported_region(
                event_details,
                kubernetes.region().to_string(),
                Some(CommandError::new_from_safe_message(error.to_string())),
            )));
        }
    };

    let credentials = new_rusoto_creds(
        cloud_provider
            .downcast_ref()
            .as_aws()
            .ok_or_else(|| Box::new(EngineError::new_bad_cast(event_details.clone(), "Cloudprovider is not AWS")))?
            .aws_credentials(),
    );

    let client = Client::new_with(credentials, HttpClient::new().expect("unable to create new Http client"));
    Ok(EksClient::new_with_client(client, region))
}

pub fn define_cluster_upgrade_timeout(
    pods_list: Vec<K8sPod>,
    kubernetes_action: KubernetesClusterAction,
) -> (ChronoDuration, Option<String>) {
    let mut cluster_upgrade_timeout = AWS_EKS_DEFAULT_UPGRADE_TIMEOUT_DURATION;
    let mut message = None;
    if kubernetes_action != KubernetesClusterAction::Bootstrap {
        // this shouldn't be a blocker in any case
        let mut max_termination_period_found = ChronoDuration::seconds(0);
        let mut pod_names = Vec::new();

        // find the highest termination period among all pods
        for pod in pods_list {
            let current_termination_period = pod
                .metadata
                .termination_grace_period_seconds
                .unwrap_or(ChronoDuration::seconds(0));

            if current_termination_period > max_termination_period_found {
                max_termination_period_found = current_termination_period;
            }

            if current_termination_period > AWS_EKS_MAX_NODE_DRAIN_TIMEOUT_DURATION {
                pod_names.push(format!(
                    "{} [{:?}] ({} seconds)",
                    pod.metadata.name.clone(),
                    pod.status.phase,
                    current_termination_period
                ));
            }
        }

        // update upgrade timeout if required
        let upgrade_time_in_minutes = ChronoDuration::minutes(max_termination_period_found.num_minutes() * 2);
        if !pod_names.is_empty() {
            cluster_upgrade_timeout = upgrade_time_in_minutes;
            message = Some(format!(
                "Kubernetes workers timeout will be adjusted to {} minutes, because some pods have a termination period greater than 15 min. Pods:\n{}",
                cluster_upgrade_timeout.num_minutes(),
                pod_names.join(", ")
            ));
        }
    };
    (cluster_upgrade_timeout, message)
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::define_cluster_upgrade_timeout;
    use crate::environment::models::kubernetes::{K8sMetadata, K8sPod, K8sPodPhase, K8sPodStatus};
    use crate::infrastructure::action::eks::AWS_EKS_DEFAULT_UPGRADE_TIMEOUT_DURATION;
    use crate::io_models::models::KubernetesClusterAction;

    #[test]
    fn test_upgrade_timeout() {
        // bootrap
        assert_eq!(
            define_cluster_upgrade_timeout(Vec::new(), KubernetesClusterAction::Bootstrap).0,
            AWS_EKS_DEFAULT_UPGRADE_TIMEOUT_DURATION
        );
        // update without nodes
        assert_eq!(
            define_cluster_upgrade_timeout(Vec::new(), KubernetesClusterAction::Update(None)).0,
            AWS_EKS_DEFAULT_UPGRADE_TIMEOUT_DURATION
        );
        // update with 1 node above termination_grace_period_seconds
        let res = define_cluster_upgrade_timeout(
            vec![
                K8sPod {
                    metadata: K8sMetadata {
                        name: "x".to_string(),
                        namespace: "x".to_string(),
                        termination_grace_period_seconds: Some(Duration::seconds(40)),
                        labels: None,
                        annotations: None,
                    },
                    status: K8sPodStatus {
                        phase: K8sPodPhase::Running,
                    },
                },
                K8sPod {
                    metadata: K8sMetadata {
                        name: "y".to_string(),
                        namespace: "z".to_string(),
                        termination_grace_period_seconds: Some(Duration::minutes(80)),
                        labels: None,
                        annotations: None,
                    },
                    status: K8sPodStatus {
                        phase: K8sPodPhase::Pending,
                    },
                },
            ],
            KubernetesClusterAction::Update(None),
        );
        assert_eq!(res.0, Duration::minutes(160));
        assert!(res.1.is_some());
        assert!(res.1.unwrap().contains("160 minutes"));
    }
}
