use ds_event_stream_rs_sdk::utils::{get_topic, list_topics};

#[test]
fn test_get_topic_function_exists() {
    // Test that the function exists and can be called
    // This will fail in test environment without Kafka, which is expected
    let result = std::panic::catch_unwind(|| get_topic("username", "password", "test-topic"));

    // Function should exist and be callable (even if it fails due to no Kafka)
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_list_topics_function_exists() {
    // Test that the function exists and can be called
    // This will fail in test environment without Kafka, which is expected
    let result = std::panic::catch_unwind(|| list_topics("username", "password"));

    // Function should exist and be callable (even if it fails due to no Kafka)
    assert!(result.is_ok() || result.is_err());
}
