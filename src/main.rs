mod lib;

use crate::lib::Militariamart;
use item::currency::Currency;
use reqwest::Client;
use scrape::scrape::Scrape;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // This one is good for testing/demo as they only have a few items
    let liverpoolmilitaria = Militariamart {
        base_url: "https://liverpoolmilitaria.com".to_string(),
        shop_dimension: None,
        currency: Currency::GBP,
    };

    let client = Client::new();
    let items = liverpoolmilitaria.gather(&client, None).await?;
    let size = items.len();

    for item in items {
        let json = serde_json::to_string_pretty(&item).unwrap();
        println!("{}\n", &json);
    }

    println!("Size: {}", size);

    Ok(())
}
