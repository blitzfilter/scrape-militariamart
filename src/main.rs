use futures::TryStreamExt;
use scrape::item_core::item_data::ItemData;
use scrape::item_core::item_model::ItemModel;
use scrape::item_core::language::Language::{EN};
use scrape::scraper::Scraper;
use scrape_militariamart::Militariamart;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // This one is good for testing/demo as they only have a few items
    let liverpoolmilitaria = Militariamart {
        base_url: "https://liverpoolmilitaria.com".to_string(),
        shop_dimension: None,
        language: EN,
    };

    let reqwest_client = reqwest::Client::new();

    let items_data = liverpoolmilitaria
        .scrape(&reqwest_client, None)
        .try_collect::<Vec<ItemData>>()
        .await
        .unwrap();

    let items = items_data
        .into_iter()
        .map(|item| item.into())
        .collect::<Vec<ItemModel>>();

    let file = File::create("gorsewayantiques.json");
    file.unwrap()
        .write_all(serde_json::to_string_pretty(&items).unwrap().as_bytes())
        .ok();

    Ok(())
}
