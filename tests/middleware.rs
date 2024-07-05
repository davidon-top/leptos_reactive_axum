use axum::{response::IntoResponse, routing::get, Json, Router};
use axum_test::TestServer;
use http::{HeaderMap, HeaderName, HeaderValue};
use leptos_reactive_axum::extract;
use serde_json::{json, Value};

async fn handler(Json(json): Json<Value>) -> impl IntoResponse {
	let headers: HeaderMap = extract().await.unwrap();

	assert_eq!(
		headers.get("macrosareawesome"),
		Some(&HeaderValue::from_static("YehTheyAre"))
	);
	assert_eq!(json.as_str(), Some("hello there"));

	""
}

#[tokio::test]
async fn test_handlers() {
	let router = Router::new()
		.route("/", get(handler))
		.layer(leptos_reactive_axum::middleware::ReactiveLayer);

	let server = TestServer::new(router).unwrap();

	let _ = server
		.get("/")
		.add_header(
			HeaderName::from_static("macrosareawesome"),
			HeaderValue::from_static("YehTheyAre"),
		)
		.json(&json!("hello there"));
}
