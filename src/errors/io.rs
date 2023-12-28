use crate::errors;
use crate::events::EventDetails;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct CommandError {
    message: String,
    full_details: String,
}

impl From<errors::CommandError> for CommandError {
    fn from(error: errors::CommandError) -> Self {
        CommandError {
            message: error.message_safe,
            full_details: error.full_details.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Tag {
    AwsCloudwatchRetentionConfigurationError,
    AwsSdkDetachEC2Volumes,
    AwsSdkGetClient,
    AwsSdkListDocDbClusters,
    AwsSdkListEC2Instances,
    AwsSdkListEC2Volumes,
    AwsSdkListElasticacheClusters,
    AwsSdkListRdsInstances,
    Base64DecodeIssue,
    BuilderBuildpackCannotBuildContainerImage,
    BuilderBuildpackInvalidLanguageFormat,
    BuilderCloningRepositoryError,
    BuilderDockerCannotBuildContainerImage,
    BuilderDockerCannotExtractEnvVarsFromDockerfile,
    BuilderDockerCannotFindAnyDockerfile,
    BuilderDockerCannotListImages,
    BuilderDockerCannotReadDockerfile,
    BuilderError,
    BuilderGetBuildError,
    CannotConnectK8sCluster,
    CannotCopyFilesFromDirectoryToDirectory,
    CannotCreateFile,
    CannotDeleteNodeGroup,
    CannotDetermineK8sKubeProxyVersion,
    CannotDetermineK8sKubeletWorkerVersion,
    CannotDetermineK8sMasterVersion,
    CannotDetermineK8sRequestedUpgradeVersion,
    CannotExecuteK8sApiCustomMetrics,
    CannotExecuteK8sVersion,
    CannotFetchScalewayPrivateNetworks,
    CannotFindRequiredBinary,
    CannotGetAnyAvailableVPC,
    CannotGetCluster,
    CannotGetClusterNodes,
    CannotGetNodeGroupInfo,
    CannotGetNodeGroupList,
    CannotGetOrCreateIamRole,
    CannotGetSupportedVersions,
    CannotGetWorkspaceDirectory,
    CannotListClusters,
    CannotParseString,
    CannotPauseClusterTasksAreRunning,
    CannotPauseManagedDatabase,
    CannotReadFile,
    CannotRestartService,
    CannotRetrieveClusterConfigFile,
    CannotUninstallHelmChart,
    CannotWriteToFile,
    ClientServiceFailedToDeployBeforeStart,
    ClientServiceFailedToStart,
    CloudProviderApiMissingInfo,
    CloudProviderClientInvalidCredentials,
    CloudProviderDeleteLoadBalancer,
    CloudProviderGetLoadBalancer,
    CloudProviderGetLoadBalancerTags,
    CloudProviderInformationError,
    ClusterHasNoWorkerNodes,
    ClusterSecretsManipulationError,
    ClusterWorkerNodeNotFound,
    CompressionError,
    ContainerRegistryCannotInstantiateClient,
    ContainerRegistryCannotCreateRegistry,
    ContainerRegistryCannotCreateRepository,
    ContainerRegistryCannotGetRepository,
    ContainerRegistryCannotDeleteImage,
    ContainerRegistryCannotDeleteRegistry,
    ContainerRegistryCannotDeleteRepository,
    ContainerRegistryCannotGetCredentials,
    ContainerRegistryInvalidRegistryUrl,
    ContainerRegistryCannotLinkRegistryToCluster,
    ContainerRegistryCannotSetRepositoryLifecycleError,
    ContainerRegistryCannotSetRepositoryTags,
    ContainerRegistryImageDoesntExist,
    ContainerRegistryImageUnreachableAfterPush,
    ContainerRegistryInvalidCredentials,
    ContainerRegistryInvalidInformation,
    ContainerRegistryRegistryDoesntExist,
    ContainerRegistryRepositoryDoesntExistInRegistry,
    ContainerRegistryRepositoryNameInvalid,
    ContainerRegistryUnknownError,
    DatabaseError,
    DatabaseFailedToStartAfterSeveralRetries,
    DeleteLocalKubeconfigFileError,
    DnsProviderInformationError,
    DnsProviderInvalidApiUrl,
    DnsProviderInvalidCredentials,
    DoNotRespectCloudProviderBestPractices,
    DockerError,
    DockerPullImageError,
    DockerPushImageError,
    HelmChartUninstallError,
    HelmChartsDeployError,
    HelmChartsSetupError,
    HelmChartsUpgradeError,
    HelmDeployTimeout,
    HelmHistoryError,
    HelmReleaseDataNotFound,
    HelmSecretNotFound,
    InvalidEngineApiInputCannotBeDeserialized,
    InvalidEnginePayload,
    InvalidJobOutputCannotBeSerialized,
    JobFailure,
    JsonDeserializationError,
    JsonSerializationError,
    K8sAddonVersionNotSupported,
    K8sCannotApplyFromFile,
    K8sCannotBoundPVC,
    K8sCannotCreateNamespace,
    K8sCannotDeleteCompletedJobs,
    K8sCannotDeletePod,
    K8sCannotDeletePvc,
    K8sCannotGetCrashLoopingPods,
    K8sCannotGetPVCs,
    K8sCannotGetPods,
    K8sCannotGetServices,
    K8sCannotGetStatefulset,
    K8sCannotOrphanDelete,
    K8sCannotPVCEdit,
    K8sCannotReachToApi,
    K8sCannotRolloutRestartStatefulset,
    K8sDeleteDeploymentError,
    K8sDeleteStatefulsetError,
    K8sDescribe,
    K8sErrorCopySecret,
    K8sGetDeploymentError,
    K8sGetEvents,
    K8sGetLogs,
    K8sGetPodError,
    K8sGetSecretError,
    K8sGetStatefulsetError,
    K8sHistory,
    K8sLoadBalancerConfigurationIssue,
    K8sNodeIsNotReady,
    K8sNodeIsNotReadyWithTheRequestedVersion,
    K8sPatchSecretError,
    K8sPodDisruptionBudgetInInvalidState,
    K8sPodIsNotReady,
    K8sPodsDisruptionBudgetCannotBeRetrieved,
    K8sScaleReplicas,
    K8sServiceError,
    K8sUpgradeDeployedVsRequestedVersionsInconsistency,
    K8sValidateRequiredCPUandBurstableError,
    KubeconfigFileDoNotPermitToConnectToK8sCluster,
    KubeconfigSecurityCheckError,
    MissingRequiredEnvVariable,
    NoClusterFound,
    NotAllowedInstanceType,
    NotEnoughNodesAvailableToDeployEnvironment,
    NotEnoughResourcesToDeployEnvironment,
    NotImplementedError,
    NumberOfRequestedMaxNodesIsBelowThanCurrentUsage,
    ObjectStorageCannotInstantiateClient,
    ObjectStorageCannotActivateBucketVersioning,
    ObjectStorageCannotCreateBucket,
    ObjectStorageCannotDeleteBucket,
    ObjectStorageCannotGetBucket,
    ObjectStorageCannotDeleteFileIntoBucket,
    ObjectStorageCannotEmptyBucket,
    ObjectStorageCannotGetObjectFile,
    ObjectStorageCannotPutFileIntoBucket,
    ObjectStorageCannotTagBucket,
    ObjectStorageInvalidBucketName,
    ObjectStorageQuotaExceeded,
    OnlyOneClusterExpected,
    RouterFailedToDeploy,
    SubnetsCountShouldBeEven,
    TaskCancelled,
    TerraformAccountBlockedByProvider,
    TerraformAlreadyExistingResource,
    TerraformApplyError,
    TerraformCannotDeleteLockFile,
    TerraformCannotImportResource,
    TerraformCannotRemoveEntryOut,
    TerraformCloudProviderActivationRequired,
    TerraformCloudProviderQuotasReached,
    TerraformClusterUnsupportedVersionUpdate,
    TerraformConfigFileInvalidContent,
    TerraformContextUnsupportedParameterValue,
    TerraformDestroyError,
    TerraformErrorWhileExecutingDestroyPipeline,
    TerraformErrorWhileExecutingPipeline,
    TerraformInitError,
    TerraformInstanceTypeDoesntExist,
    TerraformInstanceVolumeCannotBeReduced,
    TerraformInvalidCIDRBlock,
    TerraformInvalidCredentials,
    TerraformManagedDatabaseError,
    TerraformMultipleInterruptsReceived,
    TerraformNotEnoughPermissions,
    TerraformPlanError,
    TerraformQoveryConfigMismatch,
    TerraformResourceDependencyViolation,
    TerraformS3BucketCreationErrorAlreadyOwnedByYou,
    TerraformServiceNotActivatedOptInRequired,
    TerraformStateLocked,
    TerraformStatelistError,
    TerraformUnknownError,
    TerraformValidateError,
    TerraformWaitingTimeoutResource,
    TerraformWrongState,
    UncompressError,
    Unknown,
    UnsupportedClusterKind,
    UnsupportedInstanceType,
    UnsupportedRegion,
    UnsupportedVersion,
    UnsupportedZone,
    VaultConnectionError,
    VaultSecretCouldNotBeCreatedOrUpdated,
    VaultSecretCouldNotBeDeleted,
    VaultSecretCouldNotBeRetrieved,
    VersionNumberParsingError,
    RouterInvalidConfiguration,
    RouterBasicAuthEnvVarCannotDecodeBase64Error,
    RouterBasicAuthEnvVarNotFound,
}

impl From<errors::Tag> for Tag {
    fn from(tag: errors::Tag) -> Self {
        match tag {
            errors::Tag::Unknown => Tag::Unknown,
            errors::Tag::TerraformAccountBlockedByProvider => Tag::TerraformAccountBlockedByProvider,
            errors::Tag::InvalidEngineApiInputCannotBeDeserialized => Tag::InvalidEngineApiInputCannotBeDeserialized,
            errors::Tag::UnsupportedInstanceType => Tag::UnsupportedInstanceType,
            errors::Tag::CannotRetrieveClusterConfigFile => Tag::CannotRetrieveClusterConfigFile,
            errors::Tag::CannotCreateFile => Tag::CannotCreateFile,
            errors::Tag::CannotGetClusterNodes => Tag::CannotGetClusterNodes,
            errors::Tag::NotEnoughNodesAvailableToDeployEnvironment => Tag::NotEnoughNodesAvailableToDeployEnvironment,
            errors::Tag::NotEnoughResourcesToDeployEnvironment => Tag::NotEnoughResourcesToDeployEnvironment,
            errors::Tag::MissingRequiredEnvVariable => Tag::MissingRequiredEnvVariable,
            errors::Tag::ClusterHasNoWorkerNodes => Tag::ClusterHasNoWorkerNodes,
            errors::Tag::ClusterWorkerNodeNotFound => Tag::ClusterWorkerNodeNotFound,
            errors::Tag::CannotGetWorkspaceDirectory => Tag::CannotGetWorkspaceDirectory,
            errors::Tag::CannotUninstallHelmChart => Tag::CannotUninstallHelmChart,
            errors::Tag::HelmReleaseDataNotFound => Tag::HelmReleaseDataNotFound,
            errors::Tag::HelmSecretNotFound => Tag::HelmSecretNotFound,
            errors::Tag::CannotExecuteK8sVersion => Tag::CannotExecuteK8sVersion,
            errors::Tag::CannotDetermineK8sMasterVersion => Tag::CannotDetermineK8sMasterVersion,
            errors::Tag::CannotDetermineK8sRequestedUpgradeVersion => Tag::CannotDetermineK8sRequestedUpgradeVersion,
            errors::Tag::CannotDetermineK8sKubeletWorkerVersion => Tag::CannotDetermineK8sKubeletWorkerVersion,
            errors::Tag::CannotDetermineK8sKubeProxyVersion => Tag::CannotDetermineK8sKubeProxyVersion,
            errors::Tag::CannotExecuteK8sApiCustomMetrics => Tag::CannotExecuteK8sApiCustomMetrics,
            errors::Tag::K8sPodDisruptionBudgetInInvalidState => Tag::K8sPodDisruptionBudgetInInvalidState,
            errors::Tag::K8sPodsDisruptionBudgetCannotBeRetrieved => Tag::K8sPodsDisruptionBudgetCannotBeRetrieved,
            errors::Tag::K8sCannotDeletePod => Tag::K8sCannotDeletePod,
            errors::Tag::K8sCannotGetCrashLoopingPods => Tag::K8sCannotGetCrashLoopingPods,
            errors::Tag::K8sCannotDeleteCompletedJobs => Tag::K8sCannotDeleteCompletedJobs,
            errors::Tag::K8sCannotGetPods => Tag::K8sCannotGetPods,
            errors::Tag::K8sUpgradeDeployedVsRequestedVersionsInconsistency => {
                Tag::K8sUpgradeDeployedVsRequestedVersionsInconsistency
            }
            errors::Tag::K8sScaleReplicas => Tag::K8sScaleReplicas,
            errors::Tag::K8sLoadBalancerConfigurationIssue => Tag::K8sLoadBalancerConfigurationIssue,
            errors::Tag::K8sServiceError => Tag::K8sServiceError,
            errors::Tag::K8sGetLogs => Tag::K8sGetLogs,
            errors::Tag::K8sGetEvents => Tag::K8sGetEvents,
            errors::Tag::K8sDescribe => Tag::K8sDescribe,
            errors::Tag::K8sHistory => Tag::K8sHistory,
            errors::Tag::K8sCannotCreateNamespace => Tag::K8sCannotCreateNamespace,
            errors::Tag::K8sPodIsNotReady => Tag::K8sPodIsNotReady,
            errors::Tag::K8sGetPodError => Tag::K8sGetPodError,
            errors::Tag::K8sGetDeploymentError => Tag::K8sGetDeploymentError,
            errors::Tag::K8sDeleteDeploymentError => Tag::K8sDeleteDeploymentError,
            errors::Tag::K8sGetStatefulsetError => Tag::K8sGetStatefulsetError,
            errors::Tag::K8sDeleteStatefulsetError => Tag::K8sDeleteStatefulsetError,
            errors::Tag::K8sGetSecretError => Tag::K8sGetSecretError,
            errors::Tag::CannotFindRequiredBinary => Tag::CannotFindRequiredBinary,
            errors::Tag::SubnetsCountShouldBeEven => Tag::SubnetsCountShouldBeEven,
            errors::Tag::CannotGetOrCreateIamRole => Tag::CannotGetOrCreateIamRole,
            errors::Tag::CannotCopyFilesFromDirectoryToDirectory => Tag::CannotCopyFilesFromDirectoryToDirectory,
            errors::Tag::CannotPauseClusterTasksAreRunning => Tag::CannotPauseClusterTasksAreRunning,
            errors::Tag::TerraformCannotRemoveEntryOut => Tag::TerraformCannotRemoveEntryOut,
            errors::Tag::TerraformErrorWhileExecutingPipeline => Tag::TerraformErrorWhileExecutingPipeline,
            errors::Tag::TerraformErrorWhileExecutingDestroyPipeline => {
                Tag::TerraformErrorWhileExecutingDestroyPipeline
            }
            errors::Tag::TerraformResourceDependencyViolation => Tag::TerraformResourceDependencyViolation,
            errors::Tag::TerraformClusterUnsupportedVersionUpdate => Tag::TerraformClusterUnsupportedVersionUpdate,
            errors::Tag::HelmChartsSetupError => Tag::HelmChartsSetupError,
            errors::Tag::HelmChartsDeployError => Tag::HelmChartsDeployError,
            errors::Tag::HelmChartsUpgradeError => Tag::HelmChartsUpgradeError,
            errors::Tag::HelmChartUninstallError => Tag::HelmChartUninstallError,
            errors::Tag::HelmHistoryError => Tag::HelmHistoryError,
            errors::Tag::CannotGetAnyAvailableVPC => Tag::CannotGetAnyAvailableVPC,
            errors::Tag::UnsupportedVersion => Tag::UnsupportedVersion,
            errors::Tag::CannotGetSupportedVersions => Tag::CannotGetSupportedVersions,
            errors::Tag::CannotGetCluster => Tag::CannotGetCluster,
            errors::Tag::ObjectStorageCannotCreateBucket => Tag::ObjectStorageCannotCreateBucket,
            errors::Tag::ObjectStorageCannotPutFileIntoBucket => Tag::ObjectStorageCannotPutFileIntoBucket,
            errors::Tag::UnsupportedRegion => Tag::UnsupportedRegion,
            errors::Tag::UnsupportedZone => Tag::UnsupportedZone,
            errors::Tag::K8sNodeIsNotReadyWithTheRequestedVersion => Tag::K8sNodeIsNotReadyWithTheRequestedVersion,
            errors::Tag::K8sNodeIsNotReady => Tag::K8sNodeIsNotReady,
            errors::Tag::NoClusterFound => Tag::NoClusterFound,
            errors::Tag::OnlyOneClusterExpected => Tag::OnlyOneClusterExpected,
            errors::Tag::CloudProviderApiMissingInfo => Tag::CloudProviderApiMissingInfo,
            errors::Tag::K8sValidateRequiredCPUandBurstableError => Tag::K8sValidateRequiredCPUandBurstableError,
            errors::Tag::TerraformContextUnsupportedParameterValue => Tag::TerraformContextUnsupportedParameterValue,
            errors::Tag::ClientServiceFailedToStart => Tag::ClientServiceFailedToStart,
            errors::Tag::ClientServiceFailedToDeployBeforeStart => Tag::ClientServiceFailedToDeployBeforeStart,
            errors::Tag::DatabaseFailedToStartAfterSeveralRetries => Tag::DatabaseFailedToStartAfterSeveralRetries,
            errors::Tag::RouterFailedToDeploy => Tag::RouterFailedToDeploy,
            errors::Tag::CloudProviderClientInvalidCredentials => Tag::CloudProviderClientInvalidCredentials,
            errors::Tag::VersionNumberParsingError => Tag::VersionNumberParsingError,
            errors::Tag::NotImplementedError => Tag::NotImplementedError,
            errors::Tag::TaskCancellationRequested => Tag::TaskCancelled,
            errors::Tag::BuilderDockerCannotFindAnyDockerfile => Tag::BuilderDockerCannotFindAnyDockerfile,
            errors::Tag::BuilderDockerCannotReadDockerfile => Tag::BuilderDockerCannotReadDockerfile,
            errors::Tag::BuilderDockerCannotExtractEnvVarsFromDockerfile => {
                Tag::BuilderDockerCannotExtractEnvVarsFromDockerfile
            }
            errors::Tag::BuilderDockerCannotBuildContainerImage => Tag::BuilderDockerCannotBuildContainerImage,
            errors::Tag::BuilderBuildpackInvalidLanguageFormat => Tag::BuilderBuildpackInvalidLanguageFormat,
            errors::Tag::BuilderBuildpackCannotBuildContainerImage => Tag::BuilderBuildpackCannotBuildContainerImage,
            errors::Tag::BuilderGetBuildError => Tag::BuilderGetBuildError,
            errors::Tag::BuilderCloningRepositoryError => Tag::BuilderCloningRepositoryError,
            errors::Tag::DockerPushImageError => Tag::DockerPushImageError,
            errors::Tag::DockerPullImageError => Tag::DockerPullImageError,
            errors::Tag::ContainerRegistryCannotInstantiateClient => Tag::ContainerRegistryCannotInstantiateClient,
            errors::Tag::ContainerRegistryCannotCreateRepository => Tag::ContainerRegistryCannotCreateRepository,
            errors::Tag::ContainerRegistryCannotGetRepository => Tag::ContainerRegistryCannotGetRepository,
            errors::Tag::ContainerRegistryCannotSetRepositoryLifecycle => {
                Tag::ContainerRegistryCannotSetRepositoryLifecycleError
            }
            errors::Tag::ContainerRegistryCannotGetCredentials => Tag::ContainerRegistryCannotGetCredentials,
            errors::Tag::ContainerRegistryInvalidRegistryUrl => Tag::ContainerRegistryInvalidRegistryUrl,
            errors::Tag::ContainerRegistryCannotDeleteImage => Tag::ContainerRegistryCannotDeleteImage,
            errors::Tag::ContainerRegistryImageDoesntExist => Tag::ContainerRegistryImageDoesntExist,
            errors::Tag::ContainerRegistryImageUnreachableAfterPush => Tag::ContainerRegistryImageUnreachableAfterPush,
            errors::Tag::ContainerRegistryRepositoryDoesntExistInRegistry => {
                Tag::ContainerRegistryRepositoryDoesntExistInRegistry
            }
            errors::Tag::ContainerRegistryCannotDeleteRepository => Tag::ContainerRegistryCannotDeleteRepository,
            errors::Tag::ContainerRegistryInvalidInformation => Tag::ContainerRegistryInvalidInformation,
            errors::Tag::ContainerRegistryRegistryDoesntExist => Tag::ContainerRegistryRegistryDoesntExist,
            errors::Tag::ContainerRegistryInvalidCredentials => Tag::ContainerRegistryInvalidCredentials,
            errors::Tag::ContainerRegistryCannotLinkRegistryToCluster => {
                Tag::ContainerRegistryCannotLinkRegistryToCluster
            }
            errors::Tag::ContainerRegistryCannotDeleteRegistry => Tag::ContainerRegistryCannotDeleteRegistry,
            errors::Tag::ContainerRegistryCannotSetRepositoryTags => Tag::ContainerRegistryCannotSetRepositoryTags,
            errors::Tag::ContainerRegistryUnknownError => Tag::ContainerRegistryUnknownError,
            errors::Tag::ContainerRegistryRepositoryNameInvalid => Tag::ContainerRegistryRepositoryNameInvalid,
            errors::Tag::BuilderDockerCannotListImages => Tag::BuilderDockerCannotListImages,
            errors::Tag::DockerError => Tag::DockerError,
            errors::Tag::ObjectStorageCannotInstantiateClient => Tag::ObjectStorageCannotInstantiateClient,
            errors::Tag::ObjectStorageInvalidBucketName => Tag::ObjectStorageInvalidBucketName,
            errors::Tag::ObjectStorageCannotEmptyBucket => Tag::ObjectStorageCannotEmptyBucket,
            errors::Tag::ObjectStorageCannotTagBucket => Tag::ObjectStorageCannotTagBucket,
            errors::Tag::ObjectStorageCannotActivateBucketVersioning => {
                Tag::ObjectStorageCannotActivateBucketVersioning
            }
            errors::Tag::BuilderError => Tag::BuilderError,
            errors::Tag::ContainerRegistryCannotCreateRegistry => Tag::ContainerRegistryCannotCreateRegistry,
            errors::Tag::UnsupportedClusterKind => Tag::UnsupportedClusterKind,
            errors::Tag::NotAllowedInstanceType => Tag::NotAllowedInstanceType,
            errors::Tag::TerraformConfigFileNotFound => Tag::TerraformQoveryConfigMismatch,
            errors::Tag::KubeconfigFileDoNotPermitToConnectToK8sCluster => {
                Tag::KubeconfigFileDoNotPermitToConnectToK8sCluster
            }
            errors::Tag::KubeconfigSecurityCheckError => Tag::KubeconfigSecurityCheckError,
            errors::Tag::DeleteLocalKubeconfigFileError => Tag::DeleteLocalKubeconfigFileError,
            errors::Tag::VaultConnectionError => Tag::VaultConnectionError,
            errors::Tag::VaultSecretCouldNotBeRetrieved => Tag::VaultSecretCouldNotBeRetrieved,
            errors::Tag::VaultSecretCouldNotBeCreatedOrUpdated => Tag::VaultSecretCouldNotBeCreatedOrUpdated,
            errors::Tag::JsonDeserializationError => Tag::JsonDeserializationError,
            errors::Tag::ClusterSecretsManipulationError => Tag::ClusterSecretsManipulationError,
            errors::Tag::VaultSecretCouldNotBeDeleted => Tag::VaultSecretCouldNotBeDeleted,
            errors::Tag::ObjectStorageCannotDeleteFileIntoBucket => Tag::ObjectStorageCannotDeleteFileIntoBucket,
            errors::Tag::CannotGetNodeGroupList => Tag::CannotGetNodeGroupList,
            errors::Tag::CannotGetNodeGroupInfo => Tag::CannotGetNodeGroupInfo,
            errors::Tag::NumberOfRequestedMaxNodesIsBelowThanCurrentUsage => {
                Tag::NumberOfRequestedMaxNodesIsBelowThanCurrentUsage
            }
            errors::Tag::CannotConnectK8sCluster => Tag::CannotConnectK8sCluster,
            errors::Tag::DnsProviderInformationError => Tag::DnsProviderInformationError,
            errors::Tag::CloudProviderInformationError => Tag::CloudProviderInformationError,
            errors::Tag::DnsProviderInvalidCredentials => Tag::DnsProviderInvalidCredentials,
            errors::Tag::DnsProviderInvalidApiUrl => Tag::DnsProviderInvalidApiUrl,
            errors::Tag::K8sErrorCopySecret => Tag::K8sErrorCopySecret,
            errors::Tag::K8sCannotReachToApi => Tag::K8sCannotReachToApi,
            errors::Tag::TerraformUnknownError => Tag::TerraformUnknownError,
            errors::Tag::TerraformConfigFileInvalidContent => Tag::TerraformConfigFileInvalidContent,
            errors::Tag::TerraformCannotDeleteLockFile => Tag::TerraformCannotDeleteLockFile,
            errors::Tag::TerraformInitError => Tag::TerraformInitError,
            errors::Tag::TerraformValidateError => Tag::TerraformValidateError,
            errors::Tag::TerraformPlanError => Tag::TerraformPlanError,
            errors::Tag::TerraformApplyError => Tag::TerraformApplyError,
            errors::Tag::TerraformDestroyError => Tag::TerraformDestroyError,
            errors::Tag::TerraformCloudProviderQuotasReached => Tag::TerraformCloudProviderQuotasReached,
            errors::Tag::TerraformCloudProviderActivationRequired => Tag::TerraformCloudProviderActivationRequired,
            errors::Tag::TerraformInvalidCredentials => Tag::TerraformInvalidCredentials,
            errors::Tag::TerraformServiceNotActivatedOptInRequired => Tag::TerraformServiceNotActivatedOptInRequired,
            errors::Tag::TerraformWaitingTimeoutResource => Tag::TerraformWaitingTimeoutResource,
            errors::Tag::TerraformAlreadyExistingResource => Tag::TerraformAlreadyExistingResource,
            errors::Tag::TerraformNotEnoughPermissions => Tag::TerraformNotEnoughPermissions,
            errors::Tag::TerraformWrongState => Tag::TerraformWrongState,
            errors::Tag::TerraformInstanceTypeDoesntExist => Tag::TerraformInstanceTypeDoesntExist,
            errors::Tag::TerraformMultipleInterruptsReceived => Tag::TerraformMultipleInterruptsReceived,
            errors::Tag::TerraformInstanceVolumeCannotBeReduced => Tag::TerraformInstanceVolumeCannotBeReduced,
            errors::Tag::TerraformS3BucketCreationErrorAlreadyOwnedByYou => {
                Tag::TerraformS3BucketCreationErrorAlreadyOwnedByYou
            }
            errors::Tag::TerraformCannotImportResource => Tag::TerraformCannotImportResource,
            errors::Tag::TerraformManagedDatabaseError => Tag::TerraformManagedDatabaseError,
            errors::Tag::HelmDeployTimeout => Tag::HelmDeployTimeout,
            errors::Tag::CannotPauseManagedDatabase => Tag::CannotPauseManagedDatabase,
            errors::Tag::ObjectStorageCannotDeleteBucket => Tag::ObjectStorageCannotDeleteBucket,
            errors::Tag::ObjectStorageCannotGetBucket => Tag::ObjectStorageCannotGetBucket,
            errors::Tag::ObjectStorageQuotaExceeded => Tag::ObjectStorageQuotaExceeded,
            errors::Tag::ObjectStorageCannotGetObjectFile => Tag::ObjectStorageCannotGetObjectFile,
            errors::Tag::CloudProviderGetLoadBalancer => Tag::CloudProviderGetLoadBalancer,
            errors::Tag::CloudProviderGetLoadBalancerTags => Tag::CloudProviderGetLoadBalancerTags,
            errors::Tag::K8sCannotDeletePvc => Tag::K8sCannotDeletePvc,
            errors::Tag::CloudProviderDeleteLoadBalancer => Tag::CloudProviderDeleteLoadBalancer,
            errors::Tag::InvalidEnginePayload => Tag::InvalidEnginePayload,
            errors::Tag::JobFailure => Tag::JobFailure,
            errors::Tag::TerraformInvalidCIDRBlock => Tag::TerraformInvalidCIDRBlock,
            errors::Tag::DoNotRespectCloudProviderBestPractices => Tag::DoNotRespectCloudProviderBestPractices,
            errors::Tag::TerraformStateLocked => Tag::TerraformStateLocked,
            errors::Tag::K8sCannotGetPVCs => Tag::K8sCannotGetPVCs,
            errors::Tag::K8sCannotGetServices => Tag::K8sCannotGetServices,
            errors::Tag::K8sCannotBoundPVC => Tag::K8sCannotBoundPVC,
            errors::Tag::K8sCannotOrphanDelete => Tag::K8sCannotOrphanDelete,
            errors::Tag::K8sCannotPVCEdit => Tag::K8sCannotPVCEdit,
            errors::Tag::K8sCannotGetStatefulset => Tag::K8sCannotGetStatefulset,
            errors::Tag::K8sCannotRolloutRestartStatefulset => Tag::K8sCannotRolloutRestartStatefulset,
            errors::Tag::K8sCannotApplyFromFile => Tag::K8sCannotApplyFromFile,
            errors::Tag::K8sAddonVersionNotSupported => Tag::K8sAddonVersionNotSupported,
            errors::Tag::CannotListClusters => Tag::CannotListClusters,
            errors::Tag::CannotParseString => Tag::CannotParseString,
            errors::Tag::CannotDeleteNodeGroup => Tag::CannotDeleteNodeGroup,
            errors::Tag::CannotRestartService => Tag::CannotRestartService,
            errors::Tag::AwsSdkGetClient => Tag::AwsSdkGetClient,
            errors::Tag::AwsSdkListRdsInstances => Tag::AwsSdkListRdsInstances,
            errors::Tag::AwsSdkListElasticacheClusters => Tag::AwsSdkListElasticacheClusters,
            errors::Tag::AwsSdkListDocDbClusters => Tag::AwsSdkListDocDbClusters,
            errors::Tag::AwsCloudwatchRetentionConfigurationError => Tag::AwsCloudwatchRetentionConfigurationError,
            errors::Tag::AwsSdkListEC2Volumes => Tag::AwsSdkListEC2Volumes,
            errors::Tag::AwsSdkDetachEC2Volumes => Tag::AwsSdkDetachEC2Volumes,
            errors::Tag::AwsSdkListEC2Instances => Tag::AwsSdkListEC2Instances,
            errors::Tag::Base64DecodeIssue => Tag::Base64DecodeIssue,
            errors::Tag::CannotReadFile => Tag::CannotReadFile,
            errors::Tag::InvalidJobOutputCannotBeSerialized => Tag::InvalidJobOutputCannotBeSerialized,
            errors::Tag::DatabaseError => Tag::DatabaseError,
            errors::Tag::K8sPatchSecretError => Tag::K8sPatchSecretError,
            errors::Tag::CompressionError => Tag::CompressionError,
            errors::Tag::UncompressError => Tag::UncompressError,
            errors::Tag::JsonSerializationError => Tag::JsonSerializationError,
            errors::Tag::RouterInvalidConfiguration => Tag::RouterInvalidConfiguration,
            errors::Tag::RouterBasicAuthEnvVarCannotDecodeBase64Error => {
                Tag::RouterBasicAuthEnvVarCannotDecodeBase64Error
            }
            errors::Tag::RouterBasicAuthEnvVarNotFound => Tag::RouterBasicAuthEnvVarNotFound,
            errors::Tag::CannotFetchScalewayPrivateNetworks => Tag::CannotFetchScalewayPrivateNetworks,
            errors::Tag::CannotWriteToFile => Tag::CannotWriteToFile,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct EngineError {
    tag: Tag,
    user_log_message: String,
    underlying_error: Option<CommandError>,
    link: Option<String>,
    hint_message: Option<String>,
}

impl EngineError {
    pub fn from(error: errors::EngineError) -> (Self, EventDetails) {
        (
            EngineError {
                tag: Tag::from(error.tag),
                user_log_message: error.user_log_message,
                underlying_error: error.underlying_error.map(CommandError::from),
                link: error.link.map(|url| url.to_string()),
                hint_message: error.hint_message,
            },
            error.event_details,
        )
    }
}
