use axum::{body::Body, http::Request};
use tower::ServiceExt;



mod setup;

#[tokio::test]
async fn test_dns_add() {
    let app = setup::get_app_router();

    let request = Request::builder()
        .method("POST")
        .uri("/api/dns")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{ "type": "A", "hostname": "example.com", "value": "192.168.1.1" }"#)).unwrap();


    let response = app.oneshot(request).await.unwrap();

    let data = response.body().to_owned();
    

}
