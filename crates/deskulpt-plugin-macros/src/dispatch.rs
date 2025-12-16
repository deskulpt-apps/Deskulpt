//! Internals of the `#[dispatch]` macro.

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{FnArg, ItemFn, Pat, PatType, ReturnType, parse_macro_input, parse_quote};

/// Token stream processor for the `#[dispatch]` macro.
///
/// This clones the AST of the original function and performs the following
/// modifications:
///
/// - Get the type of the `input` parameter of the function and replace it with
///   `serde_json::Value`. Not having an `input` parameter panics.
/// - Get the return type of the function and replace it with
///   `anyhow::Result<serde_json::Value>`. Not specifying an explicit return
///   type panics.
/// - Wrap the original function body in a block that deserializes the input,
///   calls the original function, serializes the output, and returns it. Note
///   that the original function must have a return type that the `?` operator
///   can be applied to.
pub fn proc_dispatch(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut meth = parse_macro_input!(item as ItemFn);

    let mut input_type = None;
    for arg in &mut meth.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg
            && let Pat::Ident(ident) = &**pat
            && ident.ident == "input"
        {
            input_type = Some(ty.clone());
            **ty = parse_quote!(::deskulpt_plugin::serde_json::Value);
        }
    }
    let input_type = input_type.expect("Missing `input` parameter");

    let output_type = if let ReturnType::Type(_, ty) = meth.sig.output {
        ty
    } else {
        panic!("Return type must be specified");
    };
    meth.sig.output =
        parse_quote!(-> ::deskulpt_plugin::anyhow::Result<::deskulpt_plugin::serde_json::Value>);

    let original_body = meth.block.clone();
    meth.block = Box::new(parse_quote!({
        let context = format!("Failed to deserialize input: {:?}", input);
        let input: #input_type = ::deskulpt_plugin::anyhow::Context::context(::deskulpt_plugin::serde_json::from_value(input), context)?;
        let result: #output_type = #original_body;
        let result = result?;
        let output = ::deskulpt_plugin::anyhow::Context::context(::deskulpt_plugin::serde_json::to_value(result), "Failed to serialize output")?;
        Ok(output)
    }));

    meth.into_token_stream().into()
}
