use base64::engine::general_purpose;
use base64::Engine;

use crate::build_platform::Image;
use crate::container_registry::errors::ContainerRegistryError;
use crate::container_registry::{ContainerRegistry, ContainerRegistryInfo, Kind, Repository, RepositoryInfo};

use crate::io_models::context::Context;

use crate::cmd::docker::ContainerImage;
use crate::cmd::skopeo::Skopeo;
use url::Url;
use uuid::Uuid;

use super::RegistryTags;

pub struct GenericCr {
    context: Context,
    long_id: Uuid,
    name: String,
    url: Url,
    skip_tls_verification: bool,
    _repository_name: String,
    skopeo: Skopeo,
    cr_info: ContainerRegistryInfo,
    // Only used for the demo mode, which does not support delete operations on its registry.
    // And skopeo does not return the same error with ARM version. On AMD64 it works fine.
    // https://github.com/k3d-io/k3d/issues/1090
    support_delete: bool,
}

impl GenericCr {
    pub fn new(
        context: Context,
        long_id: Uuid,
        name: &str,
        url: Url,
        skip_tls_verification: bool,
        repository_name: String,
        credentials: Option<(String, String)>,
        support_delete: bool,
    ) -> Result<Self, ContainerRegistryError> {
        let mut registry_docker_json_config = None;
        if let Some((user, pass)) = &credentials {
            let mut registry_url = url.clone();
            let _ = registry_url.set_username(user);
            let _ = registry_url.set_password(Some(pass));

            context
                .docker
                .login(&registry_url)
                .map_err(|_err| ContainerRegistryError::InvalidCredentials)?;

            registry_docker_json_config = Some(GenericCr::get_docker_json_config_raw(
                url.host_str().unwrap_or(""),
                url.port_or_known_default().unwrap_or(443),
                user,
                pass,
            ));
        }

        let skopeo = Skopeo::new(credentials).map_err(|err| ContainerRegistryError::CannotInstantiateClient {
            raw_error_message: err.to_string(),
        })?;

        let container_registry_info = ContainerRegistryInfo {
            endpoint: url.clone(),
            registry_name: name.to_string(),
            registry_docker_json_config,
            insecure_registry: skip_tls_verification,
            get_image_name: Box::new({
                let repository = repository_name.clone();
                move |name| format!("{}/{}", repository, name)
            }),
            get_repository_name: Box::new({
                let repository = repository_name.clone();
                move |name| format!("{}/{}", repository, name)
            }),
        };

        let cr = Self {
            context,
            long_id,
            name: name.to_string(),
            skip_tls_verification: if url.scheme() == "http" {
                true
            } else {
                skip_tls_verification
            },
            url,
            _repository_name: repository_name,
            skopeo,
            cr_info: container_registry_info,
            support_delete,
        };

        Ok(cr)
    }

    fn get_docker_json_config_raw(host: &str, port: u16, login: &str, secret_token: &str) -> String {
        let port = if port == 443 {
            "".to_string()
        } else {
            format!(":{}", port)
        };
        general_purpose::STANDARD.encode(
            format!(
                r#"{{"auths":{{"{}{}":{{"auth":"{}"}}}}}}"#,
                host,
                port,
                general_purpose::STANDARD.encode(format!("{login}:{secret_token}").as_bytes())
            )
            .as_bytes(),
        )
    }
}

impl ContainerRegistry for GenericCr {
    fn context(&self) -> &Context {
        &self.context
    }

    fn kind(&self) -> Kind {
        Kind::GenericCr
    }

    fn id(&self) -> &str {
        ""
    }

    fn long_id(&self) -> &Uuid {
        &self.long_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn registry_info(&self) -> &ContainerRegistryInfo {
        &self.cr_info
    }

    fn create_registry(&self) -> Result<(), ContainerRegistryError> {
        // Nothing to do, local registry create automatically new repositories
        Ok(())
    }

    fn create_repository(
        &self,
        name: &str,
        _image_retention_time_in_seconds: u32,
        _registry_tags: RegistryTags,
    ) -> Result<(Repository, RepositoryInfo), ContainerRegistryError> {
        // Nothing to do, local registry create automatically new repositories
        Ok((
            Repository {
                registry_id: name.to_string(),
                name: name.to_string(),
                uri: Some(self.url.join(name).map(|u| u.to_string()).unwrap_or_default()),
                ttl: None,
                labels: None,
            },
            RepositoryInfo { created: false },
        ))
    }

    fn get_repository(&self, repository_name: &str) -> Result<Repository, ContainerRegistryError> {
        Ok(Repository {
            registry_id: repository_name.to_string(),
            name: repository_name.to_string(),
            uri: Some(
                self.url
                    .join(repository_name)
                    .map(|u| u.to_string())
                    .unwrap_or_default(),
            ),
            ttl: None,
            labels: None,
        })
    }

    fn delete_repository(&self, repository_name: &str) -> Result<(), ContainerRegistryError> {
        if !self.support_delete {
            return Ok(());
        }

        let container =
            ContainerImage::new(self.cr_info.endpoint.clone(), repository_name.to_string(), vec!["".to_string()]);
        let tags = self
            .skopeo
            .list_tags(&container, !self.skip_tls_verification)
            .map_err(|err| ContainerRegistryError::CannotDeleteRepository {
                registry_name: self.name.clone(),
                repository_name: repository_name.to_string(),
                raw_error_message: err.to_string(),
            })?;

        for tag in tags {
            let container = ContainerImage::new(self.cr_info.endpoint.clone(), repository_name.to_string(), vec![tag]);
            self.skopeo
                .delete_image(&container, !self.skip_tls_verification)
                .map_err(|err| ContainerRegistryError::CannotDeleteRepository {
                    registry_name: self.name.clone(),
                    repository_name: repository_name.to_string(),
                    raw_error_message: err.to_string(),
                })?;
        }

        Ok(())
    }

    fn delete_image(&self, image: &Image) -> Result<(), ContainerRegistryError> {
        if !self.support_delete {
            return Ok(());
        }

        let container = ContainerImage::new(self.cr_info.endpoint.clone(), image.name.clone(), vec![image.tag.clone()]);
        self.skopeo
            .delete_image(&container, !self.skip_tls_verification)
            .map_err(|err| ContainerRegistryError::CannotDeleteImage {
                registry_name: self.name.clone(),
                repository_name: image.repository_name().to_string(),
                image_name: image.name().to_string(),
                raw_error_message: err.to_string(),
            })?;

        Ok(())
    }

    fn image_exists(&self, image: &Image) -> bool {
        let container = ContainerImage::new(self.cr_info.endpoint.clone(), image.name.clone(), vec![image.tag.clone()]);
        let Ok(tags) = self.skopeo.list_tags(&container, !self.skip_tls_verification) else {
            return false;
        };

        tags.contains(&image.tag)
    }
}
