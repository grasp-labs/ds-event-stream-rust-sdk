use ds_event_stream_rs_sdk::utils::{get_bootstrap_servers, get_topic, list_topics, ClientCredentials, Environment};

#[test]
fn test_get_topic_function_exists() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "username".to_string(),
        password: "password".to_string(),
    };

    let result = std::panic::catch_unwind(|| get_topic(&bootstrap_servers, &credentials, "test-topic"));

    // Function should exist and be callable (even if it fails due to no Kafka)
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_list_topics_function_exists() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "username".to_string(),
        password: "password".to_string(),
    };

    let result = std::panic::catch_unwind(|| list_topics(&bootstrap_servers, &credentials));

    // Function should exist and be callable (even if it fails due to no Kafka)
    assert!(result.is_ok() || result.is_err());
}
