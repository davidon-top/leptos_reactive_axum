use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;

fn map_fnarg(fnarg: &syn::FnArg) -> Option<TokenStream> {
	match fnarg {
		syn::FnArg::Receiver(_rec) => None,
		syn::FnArg::Typed(pat_typed) => {
			let pat = &pat_typed.pat;
			let ty = match pat_typed.ty.as_ref().to_owned() {
				syn::Type::Infer(_) => None,
				_ => Some(pat_typed.ty.as_ref()),
			};
			let ty = ty.map(|t| quote! {: #t});
			Some(quote! {
				let #pat #ty = ::leptos_reactive_axum::extract().await.unwrap();
			})
		},
	}
}

pub(crate) fn reactive(attr: TokenStream, input: TokenStream) -> TokenStream {
	let syn::ItemFn {
		attrs,
		vis,
		mut sig,
		block,
	} = match syn::parse2(input) {
		Ok(input) => input,
		Err(err) => return err.to_compile_error(),
	};

	let extract_body = match syn::parse2::<syn::LitBool>(attr) {
		Ok(attr) => attr,
		Err(err) => return err.to_compile_error(),
	}
	.value();

	let prepend = quote! {
		let __runtime = leptos_reactive::create_runtime();
		scopeguard::defer!(__runtime.dispose());
		leptos_reactive::provide_context(__parts);
	};
	let stmts = &block.stmts;

	let mut extractors = Vec::new();

	let body = if extract_body {
		if let Some(fnarg) = sig.inputs.pop() {
			Some(fnarg.into_value())
		} else {
			None
		}
	} else {
		None
	};

	sig.inputs.iter().for_each(|fnarg| {
		extractors.push(map_fnarg(fnarg));
	});

	let extractors = extractors.iter().filter(|e| e.is_some());

	sig.inputs = Punctuated::new();
	sig.inputs
		.push(syn::parse2(quote! {__parts: ::http::request::Parts}).unwrap());
	if let Some(body) = body {
		sig.inputs.push(body);
	}

	quote! {
		#(#attrs)* #vis #sig {
			#prepend

			#(#extractors)*

			#(#stmts)*
		}
	}
}
