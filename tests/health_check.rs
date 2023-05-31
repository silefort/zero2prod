//! tests/health_check.rs

use std::net::TcpListener;
use urlencoding::encode;

// Launch our application in the background - somehow -
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind app_address");
    let _ = actix_web::rt::spawn(server);

    // We return the application app_address to the caller!
    format!("http://127.0.0.1:{}", port)
}

// `tokio::test`is the testing equivalent of `tokio::main`.
// It alos spares you from having to specify the `#[test]`attribute.
// Here I use actix_web::test who seems to inherit from tokio
//
// You can inspect what code gets generated using
// `carg expand --test health_check`(<- name of the test file)
#[actix_web::test]
async fn health_check_works() {
    // Given
    let app_address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // When
    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_web::test]
async fn subscribe_endpoint_should_return_200_when_mail_and_name_correctly_entered() {
    // Given
    let app_address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();
    let user_email_address = "paul.bismuth@yopmail.com";
    let user_name = "paul";

    // When
    let body = encode(&(format!("name={}&email={}", user_name, user_email_address))).into_owned();
    println!("{}",body);
    //let body = "name%3Dpaul%40email%3Dpaul.bismuth%40yopmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type","application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    assert_eq!(response.status(), actix_web::http::StatusCode::OK);
}

#[actix_web::test]
async fn subscribe_endpoint_should_return_400_when_name_is_missing() {
    // Given
    let app_address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();
    let user_email_address = "paul.bismuth@yopmail.com";
    let user_name = "";

    // When
    let body = encode(&(format!("name={}&email={}", user_name, user_email_address))).into_owned();
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type","application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST, "The API did not fail with 400 Bad Request when name is missing");
}
