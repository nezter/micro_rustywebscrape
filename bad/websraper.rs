use actix_web::{web, App, HttpResponse, HttpServer};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::process::Command;
use std::time::Duration;

fn web_scraper(url: &str) -> Result<Vec<String>, reqwest::Error> {
    // ...
}

fn health_check() -> HttpResponse {
    let health = "OK";
    let benchmark = benchmark();
    let performance = performance();
    let error_testing = error_testing();
    HttpResponse::Ok().json((health, benchmark, performance, error_testing))
}

fn benchmark() -> Vec<(String,f64)> {
    //...
}

    fn performance() -> HashMap<String, f64> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("ps -o %cpu=,%mem= -p $(pgrep rust); df -h /; netstat -i; vmstat -s")
            .output()
            .expect("Failed to execute process");
    
        let output = String::from_utf8_lossy(&output.stdout);
        let mut performance = HashMap::new();
        for metric in output.split("\n") {
            if metric.contains("%") {
                let parts: Vec<&str> = metric.split(" ").collect();
                let metric_name = parts[0].trim();
                let metric_val = parts[1].trim();
                performance.insert(metric_name.to_string(), metric_val.parse::<f64>().unwrap());
            }
            if metric.contains("/") {
                let parts: Vec<&str> = metric.split(" ").collect();
                let metric_name = parts[0].trim();
                let metric_val = parts[4].trim();
                performance.insert(metric_name.to_string(), metric_val.parse::<f64>().unwrap());
            }
            if metric.contains("bytes") {
                let parts: Vec<&str> = metric.split(" ").collect();
                let metric_name = parts[1].trim();
                let metric_val = parts[0].trim();
                performance.insert(metric_name.to_string(), metric_val.parse::<f64>().unwrap());
            }
        }
        return performance;
    }
    

fn error_testing() -> String {
    let mut error = String::new();
    std::thread::spawn(move || {
        loop {
            match web_scraper("https://this-website-does-not-exist.com") {
                Ok(_) => error = "Error: Successful request to non-existing website",
                Err(e) => error = format!("Error: {}", e),
            }
            std::thread::sleep(Duration::from_secs(60));
        }
    });
    return error;
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/scrape", web::get().to(web_scraper))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();
}
