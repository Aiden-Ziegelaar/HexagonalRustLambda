use aws_config::{BehaviorVersion, SdkConfig};
use tokio::sync::OnceCell;

pub static AWS_CREDENTIAL_REPOSITORY: OnceCell<SdkCredentialsMetaRepository> =
    OnceCell::const_new();

#[derive(Clone)]
pub struct SdkCredentialsMetaRepository {
    pub sdk_config: SdkConfig,
}

impl SdkCredentialsMetaRepository {
    pub async fn new() -> SdkCredentialsMetaRepository {
        let now_sdkconfig = std::time::SystemTime::now();
        let sdk_config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
        println!(
            "aws_config::load_from_env() took {:?}",
            now_sdkconfig.elapsed().unwrap().as_millis()
        );
        SdkCredentialsMetaRepository { sdk_config }
    }
}
