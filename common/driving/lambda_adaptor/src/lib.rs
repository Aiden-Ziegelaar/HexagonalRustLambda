#[macro_export]
macro_rules! lambda_driving_adaptor {
    ($x:ident, $cond_el:expr) => {
        #[tokio::main]
        async fn main() -> Result<(), Error> {

            let now_sdkconfig = std::time::SystemTime::now();
            let sdk_config = aws_config::from_env().load().await;
            println!("aws_config::load_from_env() took {:?}", now_sdkconfig.elapsed().unwrap().as_millis());
            
            let repository = dynamo_db_repository::DynamoDBSingleTableRepository::new(
                sdk_config
            ).await;

            match models::REPOSITORY.set(repository) {
                Ok(_) => (),
                Err(_) => panic!("Unable to set repository"),
            };

            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::INFO)
                // disable printing the name of the module in every log line.
                .with_target(false)
                // disabling time is handy because CloudWatch will add the ingestion time.
                .without_time()
                .init();

            run(service_fn($x)).await
        }
    };
}
