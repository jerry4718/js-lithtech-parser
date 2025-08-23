mod derive_impl;
mod utils;

use proc_macro::TokenStream;

#[proc_macro_derive(NapiShadow, attributes(shadow_root, shadow_getter, napi_shadow))]
pub fn napi_shadow_derive(input: TokenStream) -> TokenStream {
    TokenStream::from(derive_impl::napi_shadow_derive(input))
}
