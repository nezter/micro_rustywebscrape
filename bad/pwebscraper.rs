//modularization 
mod web_scraper;
mod error_handling;
mod performance;
mod logging;

//dependency injection
struct WebScraper {
    http_client: reqwest::Client,
}

impl WebScraper {
    fn new(http_client: reqwest::Client) -> Self {
        Self { http_client }
    }

    fn scrape(&self, url: &str) -> Result<Vec<String>, reqwest::Error> {
        // ...
    }
}

fn health_check(web_scraper: &WebScraper) -> HttpResponse {
    let health = "OK";
    let benchmark = benchmark();
    let performance = performance::get_performance();
    let error_testing = error_handling::error_testing(web_scraper);
    HttpResponse::Ok().json((health, benchmark, performance, error_testing))
}

fn main() {
    let http_client = reqwest::Client::new();
    let web_scraper = WebScraper::new(http_client);
    HttpServer::new(|| {
        App::new()
            .route("/scrape", web::get().to(web_scraper::scrape))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();
}
