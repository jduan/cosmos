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

#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=John%20Wick&email=johnwick%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=John%20Wick", "missing the email"),
        ("email=johnwick%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API didn't fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
