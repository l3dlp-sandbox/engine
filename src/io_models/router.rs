use crate::environment::models;
use crate::environment::models::aws::AwsRouterExtraSettings;
use crate::environment::models::azure::AzureRouterExtraSettings;
use crate::environment::models::gcp::GcpRouterExtraSettings;
use crate::environment::models::router::{RouterAdvancedSettings, RouterError, RouterService};
use crate::environment::models::scaleway::ScwRouterExtraSettings;
use crate::environment::models::selfmanaged::OnPremiseRouterExtraSettings;
use crate::environment::models::types::{AWS, Azure, GCP, OnPremise, SCW};
use crate::infrastructure::models::cloud_provider::{CloudProvider, Kind as CPKind};
use crate::io_models::Action;
use crate::io_models::annotations_group::AnnotationsGroup;
use crate::io_models::context::Context;
use crate::io_models::labels_group::LabelsGroup;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn default_generate_certificate() -> bool {
    true
}

fn default_use_cdn() -> bool {
    false
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Router {
    pub long_id: Uuid,
    pub name: String,
    pub kube_name: String,
    pub action: Action,
    pub default_domain: String,
    pub public_port: u16,
    pub custom_domains: Vec<CustomDomain>,
    pub routes: Vec<Route>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct CustomDomain {
    pub domain: String,
    pub target_domain: String,
    #[serde(default = "default_generate_certificate")]
    pub generate_certificate: bool,
    #[serde(default = "default_use_cdn")]
    pub use_cdn: bool,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Route {
    pub path: String,
    pub service_long_id: Uuid,
}

impl Router {
    pub fn to_router_domain(
        &self,
        context: &Context,
        advanced_settings: RouterAdvancedSettings,
        cloud_provider: &dyn CloudProvider,
        annotations_groups: Vec<AnnotationsGroup>,
        labels_groups: Vec<LabelsGroup>,
    ) -> Result<Box<dyn RouterService>, RouterError> {
        let custom_domains = self
            .custom_domains
            .iter()
            .map(|it| crate::io_models::models::CustomDomain {
                domain: it.domain.clone(),
                target_domain: it.target_domain.clone(),
                generate_certificate: it.generate_certificate,
                use_cdn: it.use_cdn,
            })
            .collect::<Vec<_>>();

        let routes = self
            .routes
            .iter()
            .map(|x| crate::io_models::models::Route {
                path: x.path.clone(),
                service_long_id: x.service_long_id,
            })
            .collect::<Vec<_>>();

        match cloud_provider.kind() {
            CPKind::Aws => Ok(Box::new(models::router::Router::<AWS>::new(
                context,
                self.long_id,
                self.name.as_str(),
                self.kube_name.to_string(),
                self.action.to_service_action(),
                self.default_domain.as_str(),
                custom_domains,
                routes,
                AwsRouterExtraSettings {},
                advanced_settings,
                |transmitter| context.get_event_details(transmitter),
                annotations_groups,
                labels_groups,
            )?)),
            CPKind::Scw => {
                let router = Box::new(models::router::Router::<SCW>::new(
                    context,
                    self.long_id,
                    self.name.as_str(),
                    self.kube_name.to_string(),
                    self.action.to_service_action(),
                    self.default_domain.as_str(),
                    custom_domains,
                    routes,
                    ScwRouterExtraSettings {},
                    advanced_settings,
                    |transmitter| context.get_event_details(transmitter),
                    annotations_groups,
                    labels_groups,
                )?);
                Ok(router)
            }
            CPKind::Gcp => Ok(Box::new(models::router::Router::<GCP>::new(
                context,
                self.long_id,
                self.name.as_str(),
                self.kube_name.to_string(),
                self.action.to_service_action(),
                self.default_domain.as_str(),
                custom_domains,
                routes,
                GcpRouterExtraSettings {},
                advanced_settings,
                |transmitter| context.get_event_details(transmitter),
                annotations_groups,
                labels_groups,
            )?)),
            CPKind::Azure => Ok(Box::new(models::router::Router::<Azure>::new(
                context,
                self.long_id,
                self.name.as_str(),
                self.kube_name.to_string(),
                self.action.to_service_action(),
                self.default_domain.as_str(),
                custom_domains,
                routes,
                AzureRouterExtraSettings {},
                advanced_settings,
                |transmitter| context.get_event_details(transmitter),
                annotations_groups,
                labels_groups,
            )?)),
            CPKind::OnPremise => {
                let router = Box::new(models::router::Router::<OnPremise>::new(
                    context,
                    self.long_id,
                    self.name.as_str(),
                    self.kube_name.to_string(),
                    self.action.to_service_action(),
                    self.default_domain.as_str(),
                    custom_domains,
                    routes,
                    OnPremiseRouterExtraSettings {},
                    advanced_settings,
                    |transmitter| context.get_event_details(transmitter),
                    annotations_groups,
                    labels_groups,
                )?);
                Ok(router)
            }
        }
    }
}
