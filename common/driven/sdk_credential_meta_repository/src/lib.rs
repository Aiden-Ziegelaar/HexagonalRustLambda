use aws_config::SdkConfig;

#[derive(Clone)]
pub struct SdkCredentialsMetaRepository {
    pub sdk_config: SdkConfig,
}

impl SdkCredentialsMetaRepository {
    pub async fn new() -> SdkCredentialsMetaRepository {
        let now_sdkconfig = std::time::SystemTime::now();
        let sdk_config = aws_config::from_env().load().await;
        println!(
            "aws_config::load_from_env() took {:?}",
            now_sdkconfig.elapsed().unwrap().as_millis()
        );
        SdkCredentialsMetaRepository { sdk_config }
    }
}
