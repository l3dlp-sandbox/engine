use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

use crate::build_platform::Image;
use crate::container_registry::errors::ContainerRegistryError;
use crate::errors::EngineError;
use crate::events::{EventDetails, Stage, Transmitter};
use crate::io_models::context::Context;
use crate::io_models::QoveryIdentifier;

pub mod ecr;
pub mod errors;
pub mod google_artifact_registry;
pub mod scaleway_container_registry;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Repository {
    pub registry_id: String,
    pub id: String,
    pub name: String,
    pub uri: Option<String>,
    pub ttl: Option<Duration>,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DockerImage {
    pub repository_id: String,
    pub name: String,
    pub tag: String,
}

pub trait ContainerRegistry: Send + Sync {
    fn context(&self) -> &Context;
    fn kind(&self) -> Kind;
    fn id(&self) -> &str;
    fn long_id(&self) -> &Uuid;
    fn name(&self) -> &str;
    fn name_with_id(&self) -> String {
        format!("{} ({})", self.name(), self.id())
    }

    // Get info for this registry, url endpoint with login/password, image name convention, ...
    fn registry_info(&self) -> &ContainerRegistryInfo;

    // Some provider require specific action in order to allow container registry
    // For now it is only digital ocean, that require 2 steps to have registries
    fn create_registry(&self) -> Result<(), ContainerRegistryError>;

    // Call to create a specific repository in the registry
    // i.e: docker.io/erebe or docker.io/qovery
    // All providers requires action for that
    // The convention for us is that we create one per application
    fn create_repository(
        &self,
        repository_name: &str,
        image_retention_time_in_seconds: u32,
        resource_ttl: Option<Duration>,
    ) -> Result<(Repository, RepositoryInfo), ContainerRegistryError>;

    fn get_repository(&self, repository_name: &str) -> Result<Repository, ContainerRegistryError>;

    fn delete_repository(&self, repository_name: &str) -> Result<(), ContainerRegistryError>;

    fn delete_image(&self, image_name: &Image) -> Result<(), ContainerRegistryError>;

    // Check on the registry if a specific image already exists
    fn image_exists(&self, image: &Image) -> bool;

    fn get_event_details(&self, stage: Stage) -> EventDetails {
        let context = self.context();
        let ev = EventDetails::new(
            None,
            QoveryIdentifier::new(*context.organization_long_id()),
            QoveryIdentifier::new(*context.cluster_long_id()),
            context.execution_id().to_string(),
            stage,
            Transmitter::ContainerRegistry(*self.long_id(), self.name().to_string()),
        );

        ev
    }
}

pub fn to_engine_error(event_details: EventDetails, err: ContainerRegistryError) -> EngineError {
    EngineError::new_container_registry_error(event_details, err)
}

pub struct ContainerRegistryInfo {
    pub endpoint: Url, // Contains username and password if necessary
    pub registry_name: String,
    pub registry_docker_json_config: Option<String>,
    // give it the name of your image, and it returns the full name with prefix if needed
    // i.e: fo scaleway => image_name/image_name
    // i.e: for AWS => image_name
    pub get_image_name: Box<dyn Fn(&str) -> String + Send + Sync>,

    // Give it the name of your image, and it return the name of the repository that will be used
    pub get_repository_name: Box<dyn Fn(&str) -> String + Send + Sync>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Kind {
    Ecr,
    ScalewayCr,
    GoogleCr,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RepositoryInfo {
    pub created: bool,
}
