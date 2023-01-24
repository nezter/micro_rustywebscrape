package main

import (
    "fmt"
    "log"

    "github.com/PuerkitoBio/goquery"
    "github.com/streadway/amqp"
)

func main() {
    // Connect to RabbitMQ server
    conn, err := amqp.Dial("amqp://guest:guest@rabbitmq:5672/")
    if err != nil {
        log.Fatalf("Failed to connect to RabbitMQ: %v", err)
    }
    defer conn.Close()

    ch, err := conn.Channel()
    if err != nil {
        log.Fatalf("Failed to open a channel: %v", err)
    }
    defer ch.Close()

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

    // Scrape data from carl.me every 5 seconds
    for {
        doc, err := goquery.NewDocument("http://carl.me")
        if err != nil {
            log.Printf("Error loading URL: %v", err)
        } else {
            // Extract data from the page using goquery
            data := extractData(doc)

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
                // Here, Carl may suggest adding code to retry publishing the message or log the error for further analysis
            } else {
                log.Printf("Published message to queue")
            }
        }
        time.Sleep(5 * time.Second)
    }
}

func extractData(doc *goquery.Document) string {
    // Code to extract data from the page using goquery
}
