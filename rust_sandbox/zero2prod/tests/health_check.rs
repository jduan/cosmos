/// This is to make the helpers available as a module.
mod helpers;

use helpers::spawn_app;

/// For each test, actix_rt::test spins up a new runtime and shuts it down.
/// So there's no need to implement any clean up logic to avoid leaking resources between tests.
#[actix_rt::test]
async fn health_check_tests() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
