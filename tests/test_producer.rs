use ds_event_stream_rs_sdk::error::ProducerError;
use ds_event_stream_rs_sdk::producer::KafkaProducer;

// Note: These tests require environment variables to be set externally
// Run with: KAFKA_BOOTSTRAP_SERVERS=localhost:9092 cargo test

#[test]
fn test_producer_new() {
    // This test will only pass if KAFKA_BOOTSTRAP_SERVERS is set in environment
    if std::env::var("KAFKA_BOOTSTRAP_SERVERS").is_ok() {
        let _producer = KafkaProducer::default("username", "password").unwrap();
        // Producer created successfully - if we reach here, the test passes
    } else {
        // Skip test if environment not set
        println!("Skipping test - KAFKA_BOOTSTRAP_SERVERS not set");
    }
}

#[test]
fn test_producer_missing_bootstrap_servers() {
    // Temporarily remove env var for this test
    let original = std::env::var("KAFKA_BOOTSTRAP_SERVERS").ok();
    std::env::remove_var("KAFKA_BOOTSTRAP_SERVERS");

    let result = KafkaProducer::default("username", "password");
    assert!(matches!(result, Err(ProducerError::MissingEnvVar { .. })));

    // Restore original value
    if let Some(val) = original {
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", val);
    }
}

#[test]
fn test_producer_empty_credentials() {
    if std::env::var("KAFKA_BOOTSTRAP_SERVERS").is_ok() {
        // Empty credentials should fail because rdkafka validates SASL credentials
        let result = KafkaProducer::default("", "");
        assert!(matches!(result, Err(ProducerError::Kafka(_))));
    } else {
        println!("Skipping test - KAFKA_BOOTSTRAP_SERVERS not set");
    }
}
