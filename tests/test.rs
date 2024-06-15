use axum::{
	http::{HeaderMap, HeaderValue},
	response::IntoResponse,
	routing::get,
	Json, Router,
};
use axum_test::TestServer;
use http::HeaderName;
use serde_json::json;

async fn demo_expanded_handler(
	__parts: ::http::request::Parts,
	Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
	let __runtime = leptos_reactive::create_runtime();
	scopeguard::defer!(__runtime.dispose());
	{
		leptos_reactive::provide_context(__parts);
	}
	let headers: HeaderMap = ::leptos_reactive_axum::extract().await.unwrap();

	assert_eq!(
		headers.get("macrosareawesome"),
		Some(&HeaderValue::from_static("YehTheyAre"))
	);
	assert_eq!(payload.as_str(), Some("hello there"));

	""
}

#[leptos_reactive_axum::reactive(true)]
async fn handler(headers: HeaderMap, Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
	assert_eq!(
		headers.get("macrosareawesome"),
		Some(&HeaderValue::from_static("YehTheyAre"))
	);
	assert_eq!(payload.as_str(), Some("hello there"));

	""
}

#[tokio::test]
async fn test_handlers() {
	let router = Router::new()
		.route("/demo", get(demo_expanded_handler))
		.route("/", get(handler));

	let server = TestServer::new(router).unwrap();

	let _ = server
		.get("/")
		.add_header(
			HeaderName::from_static("macrosareawesome"),
			HeaderValue::from_static("YehTheyAre"),
		)
		.json(&json!("hello there"));
	let _ = server
		.get("/demo")
		.add_header(
			HeaderName::from_static("macrosareawesome"),
			HeaderValue::from_static("YehTheyAre"),
		)
		.json(&json!("hello there"));
}
