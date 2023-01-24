package main

import (
    "log"

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

    // Process data from the queue
    for msg := range msgs {
        data := string(msg.Body)
        processedData := processData(data)

        // Publish processed data to another queue
        err = ch.Publish(
            "",               // exchange
            "carl-processed-data-queue", // routing key
            false,            // mandatory
            false,            // immediate
            amqp.Publishing{
                ContentType: "text/plain",
                Body:        []byte(processedData),
            })
        if err != nil {
            log.Printf("Failed to publish message: %v", err)
            // Here, Carl may suggest adding code to retry publishing the message or log the error for further analysis
        } else {
            log.Printf("Published message to queue")
        }
    }
}

func processData(data string) string {
    // Code to process the data
}
