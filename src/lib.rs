pub mod error;
#[cfg(feature = "middleware")]
pub mod middleware;

use std::fmt::Debug;

use axum::{extract::FromRequestParts, http::request::Parts};
use error::ExtractionError;
#[cfg(feature = "macros")]
pub use leptos_reactive_axum_macros::reactive;

/// used to extract request parts from handlers, should be used in conjunction with `leptos_reactive_axum_macros::reactive macro`
pub async fn extract<T>() -> Result<T, ExtractionError>
where
	T: FromRequestParts<()>,
	T::Rejection: Debug,
{
	extract_with_state::<T, ()>(&()).await
}

/// used to extract request parts from handlers with state, should be used in conjunction with `leptos_reactive_axum_macros::reactive macro`
pub async fn extract_with_state<T, S>(state: &S) -> Result<T, ExtractionError>
where
	T: FromRequestParts<S>,
	T::Rejection: Debug,
{
	let mut parts = leptos_reactive::use_context::<Parts>().ok_or_else(|| {
		ExtractionError::LeptosError("failed to extract Parts from leptos_reactive's Runtime defined by leptos_reactive_axum".to_string())
	})?;
	T::from_request_parts(&mut parts, state)
		.await
		.map_err(|e| ExtractionError::AxumError(format!("{e:?}")))
}
