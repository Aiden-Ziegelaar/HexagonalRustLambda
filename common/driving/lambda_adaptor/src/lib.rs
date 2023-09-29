#[macro_export]
macro_rules! common_lambda_adaptor {
    () => {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            // disable printing the name of the module in every log line.
            .with_target(false)
            // disabling time is handy because CloudWatch will add the ingestion time.
            .without_time()
            .init();

        println!("Starting lambda_driving_adaptor");
    };
}
