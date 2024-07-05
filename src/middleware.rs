use std::{future::Future, pin::Pin};

use axum::{extract::Request, response::Response};
use tower::{Layer, Service};

/// ```rust
/// axum::Router::new().layer(ReactiveLayer);
/// ```
#[derive(Clone)]
pub struct ReactiveLayer;
impl<S> Layer<S> for ReactiveLayer {
	type Service = ReactiveMiddleware<S>;

	fn layer(&self, inner: S) -> Self::Service {
	    ReactiveMiddleware {inner}
	}
}

#[derive(Clone)]
pub struct ReactiveMiddleware<S> {
	inner: S,
}

impl<S> Service<Request> for ReactiveMiddleware<S>
where
	S: Service<Request, Response = Response> + Send + 'static,
	S::Future: Send + 'static,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

	fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx)
	}

	fn call(&mut self, req: Request) -> Self::Future {
		let __runtime = leptos_reactive::create_runtime();
		let (parts, body) = req.into_parts();
		leptos_reactive::provide_context(parts.clone());
		let req = Request::from_parts(parts, body);

		let future = self.inner.call(req);
		Box::pin(async move {
			let response: Response = future.await?;

			__runtime.dispose();

			Ok(response)
		})
	}
}
