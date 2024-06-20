use axum::{
    body::{to_bytes, Body},
    http::{header, Method, Request},
};
use fake::{Fake, Faker};
use tower::ServiceExt;

use pie::api::router::get_router;

#[tokio::test]
async fn sut_responds_challenge_request_as_status_200() {
    // Arrange
    let sut = get_router().await;
    let request_body = challenge_request_body();
    let request = request(Method::POST, "/slack/event", &request_body);

    // Act
    let response = sut.oneshot(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn sut_responds_challenge_request_with_requested_challenge_code() {
    // Arrange
    let sut = get_router().await;
    let request_body = challenge_request_body();
    let request = request(Method::POST, "/slack/event", &request_body);

    // Act
    let response = sut.oneshot(request).await.unwrap();
    let response_body = serde_json::from_slice::<serde_json::Value>(
        &to_bytes(response.into_body(), 1000).await.unwrap(),
    )
    .unwrap();

    // Assert
    let expected = request_body["challenge"].as_str().unwrap();
    let actual = response_body.get("challenge").unwrap().as_str().unwrap();
    assert_eq!(actual, expected);
}

fn challenge_request_body() -> serde_json::Value {
    serde_json::json!({"type": "url_verification", "challenge": Faker.fake::<String>()})
}

fn request(method: Method, uri: &str, body: &serde_json::Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}
