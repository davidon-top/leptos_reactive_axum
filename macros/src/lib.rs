mod reactive;

/// macro that when applied on an axum handler will provide leptos_reactive runtime and will allow you to use `leptos_reactive_axum::extract` by providing a context holding request parts
/// WARNING: inserts .unwrap() into handlers to make types the same as in function arguments, use middleware to avoid this
#[proc_macro_attribute]
pub fn reactive(
	attr: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	reactive::reactive(attr.into(), input.into()).into()
}
