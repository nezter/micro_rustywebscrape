// Required crates
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use log::{info, error};
use log4rs::Handle;
use serde_json::to_string;

// Data Structures
struct ScrapedData {
    url: String,
    data: String,
}

struct ProcessedData {
    url: String,
    processed_data: String,
}

// Channels
let (tx, rx) = channel();
let (ptx, prx) = channel();

// Thread-safe data container
let shared_data = Arc::new(Mutex::new(vec![]));
let shared_processed_data = Arc::new(Mutex::new(vec![]));

// configure log4rs
log4rs::init_file("path/to/config.yml", Default::default()).unwrap();

// use log4rs as the global logger
let handle = Handle::new(log4rs::config::Root::new("info"));
log::set_boxed_logger(Box::new(handle)).unwrap();

// Start the performance timer
let start_time = Instant::now();

// Create a thread pool
let pool = ThreadPool::new(8);

// Scraping urls
let urls_to_scrape = vec!["https://example1.com", "https://example2.com"];
for url in urls_to_scrape {
    let tx = tx.clone();
    let shared_data = shared_data.clone();
    pool.execute(move || {
        match scrape_website(url) {
            Ok(data) => {
                let scraped_data = ScrapedData { url: url.to_string(), data };
                let mut data = shared_data.lock().unwrap();
                data.push(scraped_data);
                tx.send(scraped_data).unwrap();
            }
            Err(err) => {
                error(&err);
                match auto_fix_error(&err) {
                    Ok(fixed_data) => {
                        let scraped_data = ScrapedData { url: url.to_string(), data: fixed_data };
                        let mut data = shared_data.lock().unwrap();
                        data.push(scraped_data);
                        tx.send(scraped_data).unwrap();
                    },
                    Err(err) => {
                        error(&err);
                    }
                }
            }
        }
    });
}

// Wait for all scraping to finish
for _ in 0..urls_to_scrape.len() {
    let scraped_data = rx.recv().unwrap();
    let ptx = ptx.clone();
    let shared_processed_data = shared_processed_data.clone();
    pool.execute(move || {
        let processed_data = ProcessedData {
            url: scraped_data.url,
            processed_data: process_data(&scraped_data.data),
        };
        let mut processed_data_vec = shared_processed_data.lock().unwrap();
        processed_data_vec.push(processed_data);
        ptx.send(processed_data).unwrap();
    });
}

// Wait for all processing to finish
for _ in 0..urls_to_scrape.len() {
    let processed_data = prx.recv().unwrap();
    info!("Processed Data for URL {}: {}", processed_data.url, processed_data.processed_data);
}

// Stop the performance timer
let duration = start_time.elapsed();
info!("Total time taken: {:?}", duration);

// Print the performance metrics
let metrics = get_performance_metrics();
info!("Performance Metrics: {:?}", metrics);

// Export the processed data to an API
let json_data = to_string(&shared_processed_data).unwrap();
export_to_api(json_data);

