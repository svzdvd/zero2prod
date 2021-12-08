use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client.get(format!("{}/health_check", address)).send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
fn spawn_app() -> String {
    let ip_address = "127.0.0.1";
    let listener = TcpListener::bind(format!("{}:0", ip_address)).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    let _ = tokio::spawn(server);

    format!("http://{}:{}", ip_address, port)
}
