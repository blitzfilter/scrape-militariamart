use lambda_runtime::LambdaEvent;
use scrape::scraper::Scraper;
use scrape::scraper_config::ScraperConfig;
use scrape::{ScrapePushError, scrape_and_push};
use tracing::info;

#[tracing::instrument(skip(event), fields(req_id = %event.context.request_id))]
pub async fn function_handler<T>(
    event: LambdaEvent<ScraperConfig>,
    reqwest_client: &reqwest::Client,
    sqs_client: &aws_sdk_sqs::Client,
    dynamodb_client: &aws_sdk_dynamodb::Client,
    item_write_lambda_q_url: &str,
) -> Result<(), ScrapePushError>
where
    T: From<ScraperConfig> + Scraper,
{
    let scraper_cfg = event.payload;
    let scraper: T = scraper_cfg.clone().into();
    info!("Handling new scrape job for: '{:?}'", scraper_cfg);

    scrape_and_push(
        &scraper,
        &scraper_cfg,
        reqwest_client,
        sqs_client,
        dynamodb_client,
        item_write_lambda_q_url,
    )
    .await
}
