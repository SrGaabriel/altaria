use proc_macro::TokenStream;
use std::collections::HashMap;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn handler(args: TokenStream, item: TokenStream) -> TokenStream {
    let function_item = parse_macro_input!(item as syn::ItemFn);

    let mut function_parameters: HashMap<String, syn::Type> = HashMap::new();
    for input in function_item.sig.inputs.iter() {
        if let syn::FnArg::Typed(pat) = input {
            let ident = pat.pat.to_token_stream().to_string();
            let ty = pat.ty.to_token_stream().to_string();
            function_parameters.insert(ident, syn::parse_str(&ty).unwrap());
        }
    }

    let function_block = &function_item.block;
    let new_block = quote! {
        let
        #function_block
    };

    TokenStream::from(quote! {
        #function_item
    })
}