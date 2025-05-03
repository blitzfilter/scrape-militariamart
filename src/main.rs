use aws_config::BehaviorVersion;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use scrape::scraper_config::ScraperConfig;
use scrape_militariamart::Militariamart;
use scrape_militariamart::handler::function_handler;
use std::env;
use tracing::{error, info};
use tracing_subscriber::fmt::format::FmtSpan;

const ITEM_WRITE_LAMBDA_QUEUE_NAME: &str = "item_write_lambda_queue";

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_current_span(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_ansi(false)
        .without_time()
        .init();

    match dotenvy::from_filename(".env.localstack") {
        Ok(_) => info!("Successfully loaded '.env.localstack'."),
        Err(_) => {}
    }

    let mut aws_config_builder = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await
        .into_builder();

    if let Ok(endpoint_url) = env::var("AWS_ENDPOINT_URL") {
        aws_config_builder.set_endpoint_url(Some(endpoint_url.clone()));
        info!("Using environments custom AWS_ENDPOINT_URL '{endpoint_url}'");
    }

    let aws_conf = &aws_config_builder.build();

    let reqwest_client = &reqwest::Client::new();
    let sqs_client = &aws_sdk_sqs::Client::new(aws_conf);
    let ddb_client = &aws_sdk_dynamodb::Client::new(aws_conf);
    let item_write_lambda_q_url = &sqs_client
        .get_queue_url()
        .queue_name(ITEM_WRITE_LAMBDA_QUEUE_NAME)
        .send()
        .await
        .map_err(|e| {
            error!("Failed retrieving QUEUE_URL for QUEUE '{ITEM_WRITE_LAMBDA_QUEUE_NAME}': {e}");
            e
        })?
        .queue_url
        .ok_or_else(|| {
            let msg = format!(
                "Failed retrieving QUEUE_URL for QUEUE '{ITEM_WRITE_LAMBDA_QUEUE_NAME}': \
                                'queueUrl' is 'None'"
            );
            error!(msg);
            Error::from(msg)
        })?;

    info!("Lambda cold start completed, clients initialized.");

    run(service_fn(
        move |event: LambdaEvent<ScraperConfig>| async move {
            function_handler::<Militariamart>(
                event,
                reqwest_client,
                sqs_client,
                ddb_client,
                item_write_lambda_q_url,
            )
            .await
        },
    ))
    .await
}
