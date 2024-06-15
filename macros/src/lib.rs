mod reactive;

#[proc_macro_attribute]
pub fn reactive(
	attr: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	reactive::reactive(attr.into(), input.into()).into()
}
