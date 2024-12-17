use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, LitStr};
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn post(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item, Some(Ident::new("POST", proc_macro2::Span::call_site())))
}

#[proc_macro_attribute]
pub fn get(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item, Some(Ident::new("GET", proc_macro2::Span::call_site())))
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item, Some(Ident::new("PUT", proc_macro2::Span::call_site())))
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item, Some(Ident::new("DELETE", proc_macro2::Span::call_site())))
}

#[proc_macro_attribute]
pub fn patch(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item, Some(Ident::new("PATCH", proc_macro2::Span::call_site())))
}

#[proc_macro_attribute]
pub fn head(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item, Some(Ident::new("HEAD", proc_macro2::Span::call_site())))
}

#[proc_macro_attribute]
pub fn options(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item, Some(Ident::new("OPTIONS", proc_macro2::Span::call_site())))
}

#[proc_macro_attribute]
pub fn trace(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item, Some(Ident::new("TRACE", proc_macro2::Span::call_site())))
}

#[proc_macro_attribute]
pub fn connect(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item, Some(Ident::new("CONNECT", proc_macro2::Span::call_site())))
}

#[proc_macro_attribute]
pub fn handler(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item, None)
}

fn expand(
    args: TokenStream,
    item: TokenStream,
    method: Option<Ident>,
) -> TokenStream {
    let mut function_item = parse_macro_input!(item as syn::ItemFn);
    let function_ident = function_item.sig.ident.clone();

    let arg = parse_macro_input!(args as LitStr);
    let const_name = format!("_AltariaEndpoint{}", function_ident.to_string().to_uppercase());
    let const_ident = Ident::new(&const_name, function_ident.span());

    let path = arg.value();
    let params: Vec<_> = path.split('/')
        .filter(|s| s.starts_with('{') && s.ends_with('}'))
        .map(|s| &s[1..s.len() - 1])
        .collect();

    let mut custom_indexes = Vec::new();
    let mut inputs: Vec<_> = function_item.sig.inputs.iter().cloned().collect();

    inputs.sort_by_key(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                if params.contains(&&pat_ident.ident.to_string()[..]) {
                    return 0
                }
            }
        }
        1
    });

    let extractors: Vec<_> = inputs.iter().enumerate().map(|(index, arg)| {
        if let syn::FnArg::Typed(pat_type) = arg {
            let type_path = &pat_type.ty;
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                if params.contains(&&pat_ident.ident.to_string()[..]) {
                    custom_indexes.push(index);
                    return Some(quote! {
                        altaria::extractor::param::Param::<#type_path>
                    });
                }
            }

            let readjusted = type_path.to_token_stream().to_string().replace("<", "::<").replace(" ", "");
            let parsed_tokens: proc_macro2::TokenStream = syn::parse_str(&readjusted).expect("");
            Some(quote! {
                #parsed_tokens
            })
        } else {
            None
        }
    }).collect();

    let (variable_idents, extractions): (Vec<_>, Vec<_>) = extractors.iter().enumerate()
        .map(|(index, type_path)| {
            let ident = Ident::new(&format!("param_{}", index), type_path.span());
            (
                ident.clone(),
                quote! {
                    let #ident = #type_path::from_request(#index, &request)?;
                }
            )
        })
        .unzip();

    let values: Vec<_> = variable_idents.iter().enumerate()
        .map(|(index, ident)| {
            if custom_indexes.contains(&index) {
                quote! { #ident.0 }
            } else {
                quote! { #ident }
            }
        })
        .collect();
    let method = match method {
        Some(method) => quote! { Some(altaria::request::HttpMethod::#method) },
        None => quote! { None }
    };

    function_item.sig.inputs = syn::punctuated::Punctuated::from_iter(inputs);
    TokenStream::from(quote! {
        pub(crate) struct #const_ident;

        impl #const_ident {
            #[inline(always)]
            pub const fn new() -> Self {
                Self
            }

            #[inline(always)]
            pub const fn get_endpoint() -> &'static str {
                #path
            }
        }

        #[altaria::async_trait::async_trait]
        impl altaria::router::func::FunctionRouteHandler<(#(#extractors),*)> for #const_ident {
            fn get_method(&self) -> Option<altaria::request::HttpMethod> {
                #method
            }

            async fn handle_request(&self, request: altaria::request::HttpRequest) -> altaria::response::HttpResponse {
                let extract_values = || -> Result<(#(#extractors),*), altaria::extractor::ExtractorError> {
                    use altaria::extractor::FromRequest;
                    #(#extractions)*
                    Ok((#(#variable_idents),*))
                };

                match extract_values() {
                    Ok((#(#variable_idents),*)) => {
                        let response = #function_ident(#(#values),*).await;
                        response.into_response()
                    },
                    Err(err) => altaria::router::func::handle_function_failure(err)
                }
            }
        }

        #function_item
    })
}

#[proc_macro_attribute]
pub fn deprecated_handler_fix(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut function_item = parse_macro_input!(item as syn::ItemFn);
    sort_parameters(&mut function_item);

    TokenStream::from(quote!(#function_item))
}

fn is_target_type(type_path: &syn::TypePath, target: &str) -> bool {
    type_path.path.segments.last().map_or(false, |segment| {
        segment.ident == target
    })
}

fn get_type_priority(arg: &syn::FnArg) -> u8 {
    match arg {
        syn::FnArg::Typed(pat_type) => {
            if let syn::Type::Path(type_path) = &*pat_type.ty {
                if is_target_type(type_path, "Param") {
                    return 0
                }
            }
        },
        _ => {}
    }
    1
}

fn sort_parameters(function_item: &mut syn::ItemFn) {
    let mut inputs: Vec<_> = function_item.sig.inputs.iter().cloned().collect();
    inputs.sort_by_key(|arg| get_type_priority(arg));

    function_item.sig.inputs = syn::punctuated::Punctuated::from_iter(inputs);
}