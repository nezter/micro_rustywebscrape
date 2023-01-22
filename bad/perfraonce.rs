use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn test_performance() -> Result<(), Box<dyn Error>> {
    let mut c = Criterion::default().configure_from_args();

    c.bench_function("message_queue", |b| {
        b.iter(|| {
            // Connect to message queue
            let conn = amqp::Connection::open("amqp://guest:guest@localhost:5672/").unwrap();
            let channel = conn.create_channel().unwrap();
            let queue = channel.queue_declare("scraped_data", amqp::QueueDeclareOptions::default()).unwrap();
            let data = ScrapedData {
                url: "https://example.com".to_string(),
                data: vec![1, 2, 3],
                timestamp: chrono::Utc::now().timestamp(),
            };
            let message = bincode::serialize(&data).unwrap();
            channel.basic_publish("", &queue.name, amqp::BasicPublishOptions::default(), message).unwrap();
            let received = channel.basic_get(&queue.name, amqp::BasicGetOptions::default()).unwrap();
            let message = received.message;
            let _: ScrapedData = bincode::deserialize(&message.data).unwrap();
            black_box(conn);
            black_box(channel);
            black_box(queue);
            black_box(data);
            black_box(message);
        });
    });

    c.bench_function("web_scraping", |b| {
        b.iter(|| {
            let client = reqwest::Client::new();
            let response = client.get("https://example.com").send().unwrap();
            let body = response.text().unwrap();
            black_box(client);
            black_box(response);
            black_box(body);
        });
    });

    c.bench_function("data_processing", |b| {
        b.iter(|| {
            let data = ScrapedData {
                url: "https://example.com".to_string(),
                data: vec![1, 2, 3],
                timestamp: chrono::Utc::now().timestamp(),
            };
            process_scraped_data(&data).unwrap();
            black_box(data);
        });
    });

    c.final_summary();
    Ok(())
}
