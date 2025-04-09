use async_trait::async_trait;
use item::currency::Currency;
use item::item::ItemDiff;
use item::itemstate::ItemState;
use reqwest::Client;
use scrape::scrape::Scrape;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;

pub struct Militariamart {
    pub base_url: String,
    pub shop_dimension: Option<i8>,
    pub currency: Currency,
}

#[async_trait]
impl Scrape for Militariamart {
    async fn gather_page(
        &self,
        page_num: i16,
        client: &Client,
    ) -> Result<Vec<ItemDiff>, Box<dyn Error + Send + Sync>> {
        let html = client
            .get(format!(
                "{}/shop.php?d={}&pg={}",
                &self.base_url,
                &self.shop_dimension.unwrap_or(1),
                page_num
            ))
            .send()
            .await?
            .text()
            .await?;
        let document = Html::parse_document(&html);
        let shop_items = document
            .select(&Selector::parse("div.shopitem > div.inner-wrapper").unwrap())
            .map(|shop_item| {
                let item_id = extract_item_id(shop_item);
                let price = extract_price(shop_item, self.currency);
                ItemDiff {
                    item_id: Some(String::from("TODO")),
                    source_id: Some(String::from("TODO")),
                    event_id: None,
                    created: None,
                    item_type: None,
                    item_state: extract_state(shop_item),
                    category: None,
                    name_en: Some(String::from("TODO")),
                    description_en: Some(String::from("TODO")),
                    name_de: Some(String::from("TODO")),
                    description_de: Some(String::from("TODO")),
                    lower_year: None,
                    upper_year: None,
                    currency: Some(self.currency),
                    lower_price: price,
                    upper_price: price,
                    url: item_id.map(|id| format!("{}/shop.php?code={}", &self.base_url, id)),
                    image_url: extract_image_url(shop_item).map(|relative_image_url| {
                        format!("{}/{}", &self.base_url, relative_image_url)
                    }),
                }
            })
            .collect::<Vec<_>>();

        Ok(shop_items)
    }
}

fn extract_item_id(shop_item: ElementRef) -> Option<String> {
    shop_item
        .select(&Selector::parse("div.block-text > p.itemCode > a").unwrap())
        .next()
        .unwrap()
        .attr("href")
        .map(|href| href.strip_prefix("?code="))
        .flatten()
        .map(String::from)
}

fn extract_name(shop_item: ElementRef) -> Option<String> {
    shop_item
        .select(&Selector::parse("div.block-text > a.shopitemTitle").unwrap())
        .next()
        .unwrap()
        .attr("title")
        .map(String::from)
}

fn extract_description(shop_item: ElementRef) -> Option<String> {
    // TODO: This only gathers the description for the catalog-page.
    //       It may have been shortened. If so, it ends with '...'.
    //       If it does, go the the items page and parse full description there
    shop_item
        .select(&Selector::parse("div.block-text > p.itemDescription").unwrap())
        .next()
        .map(|desc_elem| desc_elem.text().next().map(|text| text.trim().to_string()))
        .flatten()
}

fn extract_price(shop_item: ElementRef, currency: Currency) -> Option<f32> {
    shop_item
        .select(&Selector::parse("div.block-text > div.actioncontainer > p.price").unwrap())
        .next()
        .map(|price_elem| {
            price_elem
                .text()
                .next()
                .map(|price_text| {
                    price_text
                        .replace(&currency.to_string(), "")
                        .trim()
                        .parse::<f32>()
                        .ok()
                })
                .flatten()
        })
        .flatten()
}

fn extract_state(shop_item: ElementRef) -> Option<ItemState> {
    let selectors = [
        "div.block-text > div.actioncontainer > form > button",
        "div.block-text > div.actioncontainer > form > p",
    ];

    selectors
        .iter()
        .filter_map(|selector_str| {
            let selector = Selector::parse(selector_str).ok()?;
            shop_item.select(&selector).next()
        })
        .find_map(|state_elem| {
            state_elem.text().next().map(|state_text| match state_text {
                "SOLD" => ItemState::SOLD,
                "Reserved" => ItemState::RESERVED,
                "Add to basket" => ItemState::AVAILABLE,
                _ => ItemState::LISTED,
            })
        })
        .or(Some(ItemState::LISTED))
}

fn extract_image_url(shop_item: ElementRef) -> Option<String> {
    shop_item
        .select(&Selector::parse("div.block-image > a > img").unwrap())
        .next()
        .unwrap()
        .attr("src")
        .map(String::from)
}
