pub extern crate altaria_macros;
pub extern crate paste;
pub extern crate async_trait;

#[macro_export]
macro_rules! endpoint {
    ($fn:path) => {{
        $crate::router::macros::paste::paste! {
            ([<_AltariaEndpoint $fn:upper>]::get_endpoint(), [<_AltariaEndpoint $fn:upper>]::new())
        }
    }};
}