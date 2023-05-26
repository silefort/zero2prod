//! tests/health_check.rs

use std::net::TcpListener;

// `tokio::test`is the testing equivalent of `tokio::main`.
// It alos spares you from having to specify the `#[test]`attribute.
//
// You can inspect what code gets generated using
// `carg expand --test health_check`(<- name of the test file)
#[actix_web::test]
async fn health_check_works() {
    // Given
    let address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // When
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background - somehow -
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = actix_web::rt::spawn(server);

    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}
