use crate::infrastructure::models::cloud_provider::service::Action;

use crate::events::{EnvironmentStep, EventDetails, Stage, Transmitter};
use crate::io_models::context::Context;

use crate::environment::models::application::ApplicationService;
use crate::environment::models::container::ContainerService;
use crate::environment::models::database::DatabaseService;
use crate::environment::models::helm_chart::HelmChartService;
use crate::environment::models::job::JobService;
use crate::environment::models::router::RouterService;
use crate::environment::models::terraform_service::TerraformServiceTrait;
use crate::utilities::to_short_id;
use uuid::Uuid;

pub struct Environment {
    namespace: String,
    event_details: EventDetails,
    pub id: String,
    pub long_id: Uuid,
    pub project_id: String,
    pub project_long_id: Uuid,
    pub owner_id: String,
    pub organization_id: String,
    pub organization_long_id: Uuid,
    pub action: Action,
    pub max_parallel_build: u32,
    pub max_parallel_deploy: u32,
    pub applications: Vec<Box<dyn ApplicationService>>,
    pub containers: Vec<Box<dyn ContainerService>>,
    pub routers: Vec<Box<dyn RouterService>>,
    pub databases: Vec<Box<dyn DatabaseService>>,
    pub jobs: Vec<Box<dyn JobService>>,
    pub helm_charts: Vec<Box<dyn HelmChartService>>,
    pub terraform_services: Vec<Box<dyn TerraformServiceTrait>>,
}

impl Environment {
    pub fn new(
        long_id: Uuid,
        name: String,
        kube_name: String,
        project_long_id: Uuid,
        organization_long_id: Uuid,
        action: Action,
        context: &Context,
        max_parallel_build: u32,
        max_parallel_deploy: u32,
        applications: Vec<Box<dyn ApplicationService>>,
        containers: Vec<Box<dyn ContainerService>>,
        routers: Vec<Box<dyn RouterService>>,
        databases: Vec<Box<dyn DatabaseService>>,
        jobs: Vec<Box<dyn JobService>>,
        helm_charts: Vec<Box<dyn HelmChartService>>,
        terraform_services: Vec<Box<dyn TerraformServiceTrait>>,
    ) -> Self {
        let project_id = to_short_id(&project_long_id);
        let env_id = to_short_id(&long_id);

        let event_details = context.get_event_details(Transmitter::Environment(long_id, name));
        let event_details =
            EventDetails::clone_changing_stage(event_details, Stage::Environment(action.to_environment_step()));

        Environment {
            event_details,
            namespace: kube_name,
            id: env_id,
            long_id,
            project_id,
            project_long_id,
            owner_id: "FAKE".to_string(),
            organization_id: to_short_id(&organization_long_id),
            organization_long_id,
            action,
            max_parallel_build,
            max_parallel_deploy,
            applications,
            containers,
            routers,
            databases,
            jobs,
            helm_charts,
            terraform_services,
        }
    }

    pub fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    pub fn event_details_with_step(&self, step: EnvironmentStep) -> EventDetails {
        EventDetails::clone_changing_stage(self.event_details.clone(), Stage::Environment(step))
    }

    pub fn event_details(&self) -> &EventDetails {
        &self.event_details
    }
}
