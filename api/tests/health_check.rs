mod test_app;
use crate::test_app::TestApp;

#[tokio::test]
async fn get_health_route_should_return_200_ok() {
    let app = TestApp::new();
    let client = reqwest::Client::new();

    let response = client
        .get(app.url("/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}
