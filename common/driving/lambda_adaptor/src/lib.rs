

#[macro_export]
macro_rules! lambda_driving_adaptor {
    ($x:ident) => {

        use sdk_credential_meta_repository::{ AWS_CREDENTIAL_REPOSITORY, SdkCredentialsMetaRepository };

        #[tokio::main]
        async fn main() -> Result<(), Error> {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::INFO)
                // disable printing the name of the module in every log line.
                .with_target(false)
                // disabling time is handy because CloudWatch will add the ingestion time.
                .without_time()
                .init();


            println!("Starting lambda_driving_adaptor");

            // For some reason this takes more than half a second if done anywhere but here :(
            let credentials = SdkCredentialsMetaRepository::new().await;

            match AWS_CREDENTIAL_REPOSITORY.set(credentials) {
                Ok(_) => println!("AWS_CREDENTIAL_REPOSITORY set"),
                Err(_) => println!("AWS_CREDENTIAL_REPOSITORY already set before init"),
            };

            run(service_fn($x)).await
        }
    };
}
