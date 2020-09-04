use helpers::spawn_app;

/// This is to make the helpers available as a module.
mod helpers;

/// For each test, actix_rt::test spins up a new runtime and shuts it down.
/// So there's no need to implement any clean up logic to avoid leaking resources between tests.
#[actix_rt::test]
async fn test_home_page_default() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(String::from("Hello World!"), response.text().await.unwrap());
}

#[actix_rt::test]
async fn test_home_page_with_param() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/Rust", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(String::from("Hello Rust!"), response.text().await.unwrap());
}
