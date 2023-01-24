#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use std::collections::HashMap;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the URL scraping manager"
}

#[post("/scrape", data = "<urls>")]
fn scrape(urls: Json<HashMap<String, String>>) -> String {
    let urls_to_scrape: Vec<String> = urls.into_inner().values().cloned().collect();

    // Perform the web scraping and data processing
    let data = web_scraping(urls_to_scrape);
    let processed_data = data_processing(data);

    // Return the result as a JSON object
    serde_json::to_string(&processed_data).unwrap()
}
