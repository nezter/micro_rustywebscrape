use reqwest::{Client, RedirectPolicy};
use reqwest::header::COOKIE;
use select::document::Document;
use select::predicate::Name;
use log::{info, error};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Instant, Duration};
use std::collections::HashMap;
use serde::Serialize;

// Initialize the logger
fn init_logger() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(log::LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

// Function to scrape the website
pub fn scrape_website(url: &str, cookies: &str) -> Result<String, String> {

    let client = Client::builder()
    .redirect(RedirectPolicy::none())
    .default_headers(
        headers! {
            COOKIE => cookies.to_string()
        }
    )
    .build()
    .unwrap();
    let start_time = Instant::now();

    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(err) => {
            error!("Error while sending request: {}", err);
            return Err(format!("Error while sending request: {}", err));
        },
    };

    let body = match response.text() {
        Ok(body) => body,
        Err(err) => {
            error!("Error while parsing response: {}", err);
            return Err(format!("Error while parsing response: {}", err));
        },
    };

    let document = Document::from(body.as_str());
    let mut data = String::new();
   
    for node in document.find(Name("a")) {
        let link = node.attr("href").unwrap();
        data.push_str(&format!("Link: {}\n", link));
    }

    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    info!("Scraped {} links in {:?}", data.lines().count(), duration);

    Ok(data)
}

// Function to check website health
pub fn website_health(url: &str) -> Result<String, String> {
    let client = Client::new();
    let start_time = Instant::now();
    let response = match client.get(url).send() {
        Ok(response) => response,
        Err(err) => {
            error!("Error while sending request: {}", err);
            return Err(format!("Error while sending request: {}", err));
        },
    };
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    let status = response.status();
    let mut data = String::new();
    data.push_str(&format!("Status: {}\n", status));
    data.push_str(&format!("Time: {:?}\n", duration));
    info!("Website health check: {}", data);
    Ok(data)
}

// Function to check the performance of the microservice
pub fn performance_test(url: &str, cookies: &str) -> Result<String, String> {
    let client = Client::new();
    let start_time = Instant::now();
    let response = match client.get(url).send() {
        Ok(response) => response,
        Err(err) => {
            error!("Error while sending request: {}", err);
            return Err(format!("Error while sending request: {}", err));
        },
    };
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    let status = response.status();
    let bytes = response.bytes().len();
    let mut data = String::new();
    data.push_str(&format!("Status: {}\n", status));
    data.push_str(&format!("Bytes: {}\n", bytes));
    data.push_str(&format!("Time: {:?}\n", duration));
    info!("Performance test: {}", data);
    Ok(data)
}

// Function to check the error rate of the microservice
pub fn error_rate_test(url: &str, cookies: &str, n: u32) -> Result<String, String> {
    let client = Client::new();
    let mut error_count = 0;
    for _ in 0..n {
        let response = match client.get(url).send() {
            Ok(response) => response,
            Err(_) => {
                error_count += 1;
                continue;
            },
        };
        if !response.status().is_success() {
            error_count += 1;
        }
    }
    let error_rate = (error_count as f32) / (n as f32);
    let mut data = String::new();
    data.push_str(&format!("Error rate: {:.2}%\n", error_rate * 100.0));
    info!("Error rate test: {}", data);
    Ok(data)
}

// Function to gather all the logs and output into an API
fn gather_logs() {
    // code to gather logs
}

// main function
fn main() {
    init_logger();
    let url = "https://www.example.com";
    let cookies = "session=1234; user=admin";
    let _scraped_data = scrape_website(url, cookies);
    let _health_data = website_health(url);
    let _performance_data = performance_test(url, cookies);
    let _error_data = error_rate_test(url, cookies, 100);
    gather_logs();
}
