use scrape::item_core::item_model::ItemModel;
use scrape::item_core::language::Language::{DE, EN};
use scrape::scrape::Scrape;
use scrape_militariamart::Militariamart;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use aws_config::{BehaviorVersion, Region};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conf = aws_sdk_dynamodb::Config::builder()
        .region(Region::new("eu-central-1"))
        .endpoint_url("http://localhost:8000")
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::for_tests())
        .behavior_version(BehaviorVersion::latest())
        .build();
    let ddb_client = aws_sdk_dynamodb::Client::from_conf(conf);
    
    // This one is good for testing/demo as they only have a few items
    let liverpoolmilitaria = Militariamart {
        base_url: "https://liverpoolmilitaria.com".to_string(),
        shop_dimension: None,
        language: EN,
    };   
    let aandcmilitaria = Militariamart {
        base_url: "https://gorsewayantiques.com".to_string(),
        shop_dimension: None,
        language: DE,
    };

    let reqwest_client = reqwest::Client::new();
    // let scrapers: Vec<Box<dyn Scrape>> = vec![Box::new(liverpoolmilitaria)];

    let items_data = aandcmilitaria
        .gather(&reqwest_client, None)
        .await
        .ok()
        .unwrap();
    
    let items= items_data.iter()
        .map(|item| item.clone().into())
        .collect::<Vec<ItemModel>>();
    
    let mut file = File::create("gorsewayantiques.json");
    file.unwrap().write_all(serde_json::to_string_pretty(&items).unwrap().as_bytes()).ok();

    Ok(())
}
