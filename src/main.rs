use reqwest::Client;
use scrape::item_core::language::Language::EN;
use scrape::scrape::Scrape;
use scrape_militariamart::Militariamart;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // This one is good for testing/demo as they only have a few items
    let liverpoolmilitaria = Militariamart {
        base_url: "https://liverpoolmilitaria.com".to_string(),
        shop_dimension: None,
        language: EN,
    };

    let scrapers: Vec<Box<dyn Scrape>> = vec![Box::new(liverpoolmilitaria)];

    let client = Client::new();

    for scraper in scrapers {
        let items = scraper.gather(&client, None).await;
        for item in items.unwrap() {
            let json = serde_json::to_string_pretty(&item).unwrap();
            println!("{}\n", &json);
        }
    }

    Ok(())
}
