package main

import (
    "log"
    "os"
    "time"

    "github.com/streadway/amqp"
)

func main() {
    // Read connection string from configMap
    connString := os.Getenv("RABBITMQ_CONN_STRING")

    // Connect to RabbitMQ server
    conn, err := amqp.Dial(connString)
    if err != nil {
        log.Fatalf("Failed to connect to RabbitMQ: %v", err)
    }
    defer conn.Close()

    ch, err := conn.Channel()
    if err != nil {
        log.Fatalf("Failed to open a channel: %v", err)
    }
    defer ch.Close()

    // Declare a queue
    q, err := ch.QueueDeclare(
        "carl-data-queue", // name
        true,              // durable
        false,             // delete when unused
        false,             // exclusive
        false,             // no-wait
        nil,               // arguments
    )
    if err != nil {
        log.Fatalf("Failed to declare a queue: %v", err)
    }

    // Start scraping data from carl.me
    go func() {
        for {
            // Code to scrape data from carl.me

            // Publish data to queue
            err = ch.Publish(
                "",     // exchange
                q.Name, // routing key
                false,  // mandatory
                false,  // immediate
                amqp.Publishing{
                    ContentType: "text/plain",
                    Body:        []byte(data),
                })
            if err != nil {
                log.Printf("Failed to publish message: %v", err)
            } else {
                log.Printf("Published message to queue")
            }
            time.Sleep(5 * time.Second)
        }
    }()

    // Start consuming messages from queue
    msgs, err := ch.Consume(
        q.Name, // queue
        "",     // consumer
        true,   // auto-ack
        false,  // exclusive
        false,  // no-local
        false,  // no-wait
        nil,    // args
    )
    if err != nil {
        log.Fatalf("Failed to register a consumer: %v", err)
    }

    go func() {
        for d := range msgs {
            log.Printf("Received a message: %s", d.Body)
        }
    }()

    // Start benchmarking and performance monitoring
    go func() {
        // Code to benchmark and monitor performance
    }()

    log.Printf(" [*] Waiting for messages. To exit press CTRL+C")
    forever := make(chan bool)
    <-forever
}