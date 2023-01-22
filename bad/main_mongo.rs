use std::time::Instant;
use log4rs::Handle;
use web_scraping::scrape_website;
use data_processing::process_data;
use errors::{Error, auto_fix_error};
use performance::start_timer;
use benchmark::end_timer;
use load_testing::run_load_test;
use health_check::check_health;
use logging::init_logger;
use mongodb::{Client, options::ClientOptions};

// Define the structs for the data and processed data
struct Data {
    // Struct fields
}

struct ProcessedData {
    // Struct fields
}

fn main() {
    // Initialize the logger using log4rs with the provided config file
    init_logger("path/to/config.yml");

    // Start the performance timer
    let start_time = start_timer();

    // Create a MongoDB client
    let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    let client = Client::with_options(client_options).unwrap();

    // Scrape the website and get the data
    let data = match scrape_website("https://example.com") {
        Ok(data) => data,
        Err(err) => {
            // Log the error
            log::error(&err);

            // Attempt to fix the error automatically
            match auto_fix_error(&err) {
                Ok(fixed_data) => fixed_data,
                Err(err) => {
                    // Log the error
                    log::error(&err);

                    // Send an alert to the administrator
                    send_alert("Error fixing scrape website error");

                    return err;
                }
            }
        }
    };

    // Process the data
    let processed_data = data_processing(data);

    // Insert the data and processed data into MongoDB
    match insert_into_mongodb(client, data, processed_data) {
        Ok(_) => (),
        Err(err) => {
            // Log the error
            log::error(&err);

            // Send an alert to the administrator
            send_alert("Error inserting data into MongoDB");
        }
    }

    // End the performance timer and log the total time taken
    end_timer(start_time);

    // Run load tests on the microservice
    run_load_test();

    // Check the health of the microservice
    check_health();
}

    fn insert_into_mongodb(client: Client, data: Data, processed_data: ProcessedData) -> Result<(), Error> {
        let db = client.database("web_scraping");
        let coll = db.collection("data");
    
        // Convert the structs to BSON
        let data_bson = bson::to_bson(&data).map_err(|err| Error::SerializationError(err.to_string()))?;
        let processed_data_bson = bson::to_bson(&processed_data).map_err(|err| Error::SerializationError(err.to_string()))?;
    
        if let bson::Bson::Document(data_doc) = data_bson {
            if let bson::Bson::Document(processed_data_doc) = processed_data_bson {
                // Insert the documents into MongoDB
                match coll.insert_one(doc! {
                    "data": data_doc,
                    "processed_data": processed_data_doc
                }, None) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Error::MongoDBError(err.to_string()))
                }
            } else {
                Err(Error::SerializationError("Failed to convert processed_data to BSON document".to_string()))
            }
        } else {
            Err(Error::SerializationError("Failed to convert data to BSON document".to_string()))
        }
    }
    
//this needs to be continued