use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub conn_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    // Port 0 will make the OS choose a random port.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let config = get_configuration().expect("Failed to read configuration");
    let conn_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let server = run(listener, conn_pool.clone()).expect("Failed to bind to address");
    let _ = tokio::spawn(server);

    TestApp { address, conn_pool }
}
