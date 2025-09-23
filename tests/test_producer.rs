// Note: These tests verify producer creation behavior
// Producer creation is lazy - it doesn't connect to Kafka until you send a message

use ds_event_stream_rs_sdk::error::SDKError;
use ds_event_stream_rs_sdk::producer::KafkaProducer;
use ds_event_stream_rs_sdk::utils::ClientCredentials;
use ds_event_stream_rs_sdk::utils::{get_bootstrap_servers, Environment};

#[test]
fn test_producer_new() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "username".to_string(),
        password: "password".to_string(),
    };
    let producer = KafkaProducer::default(&bootstrap_servers, &credentials);
    // Producer creation should succeed because it's lazy - no actual connection is made
    assert!(producer.is_ok());
}

#[test]
fn test_producer_empty_credentials() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "".to_string(),
        password: "".to_string(),
    };
    let result = KafkaProducer::default(&bootstrap_servers, &credentials);
    // Empty credentials should fail because Kafka validates SASL credentials during construction
    assert!(result.is_err());
    assert!(matches!(result, Err(SDKError::Producer(_))));
}
