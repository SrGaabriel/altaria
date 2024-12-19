use proc_macro::TokenStream;
use std::collections::HashMap;
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
    let query_index = path.find('?');

    let url = if let Some(index) = query_index { &path[..index] } else { &path };
    let query_part = if let Some(index) = query_index { &path[index + 1..] } else { "" };

    let params = url.split('/')
        .filter(|s| s.starts_with('{') && s.ends_with('}'))
        .map(|s| &s[1..s.len() - 1])
        .collect::<Vec<&str>>();

    let query_params: HashMap<String, String> = query_part.split('&')
        .map(|s| s.split('=').collect::<Vec<&str>>())
        .filter(|s| s.len() == 2)
        .map(|s| (s[0].to_string(), s[1].to_string()))
        .filter(|(key, value)| value.starts_with('{') && value.ends_with('}'))
        .map(|(key, value)| (key, value[1..value.len() - 1].to_string()))
        .map(|(key, value)| (value, key))
        .collect();

    let mut inputs: Vec<_> = function_item.sig.inputs.iter().cloned().collect();
    inputs.sort_by_key(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                let index_of_param = params.iter().position(|param| *param == pat_ident.ident.to_string());
                if let Some(index) = index_of_param {
                    return index;
                }
            }
        }
        params.len()
    });

    let mut accesses = Vec::new();
    let mut idents = Vec::new();
    let mut extractors = Vec::new();
    let mut extractions = Vec::new();

    for (index, arg) in inputs.iter().enumerate() {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Type::Path(type_path) = &*pat_type.ty {
                let variable_ident = Ident::new(&format!("param_{}", index), index.span());
                if let syn::Pat::Ident(ident) = &*pat_type.pat {
                    let name = ident.ident.to_string();
                    if params.contains(&name.as_str()) {
                        let extractor = quote! { altaria::extractor::param::Param::<#type_path> };
                        let access = quote! { #variable_ident.0 };
                        accesses.push(access);
                        idents.push(variable_ident.clone());
                        extractors.push(extractor.clone());
                        extractions.push(quote! {
                            let #variable_ident = #extractor::from_request(#index, &request)?;
                        });
                        continue;
                    } else if query_params.contains_key(&name) {
                        let actual_name = query_params.get(&name).unwrap();

                        let true_type = extract_option_type_param(type_path);
                        let extractor = if let Some(type_path) = true_type {
                            quote! { altaria::extractor::query::OptionalQuery::<#type_path> }
                        } else {
                            quote! { altaria::extractor::query::Query::<#type_path> }
                        };
                        let access = quote! { #variable_ident.0 };
                        accesses.push(access);
                        idents.push(variable_ident.clone());
                        extractors.push(extractor.clone());
                        extractions.push(quote! {
                            let #variable_ident = #extractor::from_request_by_name(#actual_name, &request)?;
                        });
                        continue;
                    }
                }
                let extractor_name = type_path.to_token_stream().to_string().replace("<", "::<").replace(" ", "");
                let extractor: proc_macro2::TokenStream = syn::parse_str(&extractor_name).expect("");
                let access = quote! { #variable_ident };
                accesses.push(access);
                idents.push(variable_ident.clone());
                extractors.push(extractor.clone());
                extractions.push(quote! {
                    let #variable_ident = #extractor::from_request(#index, &request)?;
                });
            } else {
                panic!("Invalid function argument: it's either not a simple identifier or not a type");
            }
        } else {
            panic!("Invalid function argument: it's either not a simple identifier or not a type");
        }
    }

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
                    use altaria::extractor::query::NamedExtractor;
                    #(#extractions)*
                    Ok((#(#idents),*))
                };

                match extract_values() {
                    Ok((#(#idents),*)) => {
                        let response = #function_ident(#(#accesses),*).await;
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

fn extract_option_type_param(type_path: &syn::TypePath) -> Option<syn::Type> {
    if let Some(segment) = type_path.path.segments.last() {
        if segment.ident == "Option" {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(arg) = args.args.first() {
                    if let syn::GenericArgument::Type(ty) = arg {
                        return Some(ty.clone())
                    }
                }
            }
        }
    }
    None
}