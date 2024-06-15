use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractionError {
	#[error("axum extracting information failed")]
	AxumError(String),
	#[error("leptos_reactive error occured")]
	LeptosError(String),
	#[error("the body of the request has been extracted more then once this isn't allowed")]
	MultipleBodyExtractor,
}
