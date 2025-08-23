use crate::utils::{append_to_tokens, get_meta_by_name, get_metas_by_attr_name, replace_type_path};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use regex::Captures;
use syn::parse::{Parse, ParseStream};
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, ExprClosure, Field,
    Fields, FieldsNamed, FieldsUnnamed, Ident, LitStr, Meta, MetaList, Token, Type, TypePath,
    Variant, Visibility,
};

macro_rules! map_meta_to_local {
    ($from:expr => { $($name:expr => $local:ident),* $(,)? }) => {
        let metas = ($from);
        $(let $local = $crate::utils::get_meta_by_name(metas, $name);)*
    };
}

const ATTR_NAPI_SHADOW: &str = "napi_shadow";

// const ATTR_INCLUDES: &[&str] = &[ATTR_NAPI_SHADOW];

const META_SHADOW_ROOT: &str = "root";
// const META_SHADOW_GETTER: &str = "getter";
const META_SHADOW_SKIP: &str = "skip";

struct ShadowGetter {
    name: Ident,
    // token_colon: Token![:],
    output: Type,
    // token_eq: Token![=],
    tpl: ExprClosure,
}

impl Parse for ShadowGetter {
    #[rustfmt::skip]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let _token_colon = input.parse::<Token![:]>()?;
        let output = input.parse::<Type>()?;
        let _token_eq = input.parse::<Token![=]>()?;
        let tpl = input.parse::<ExprClosure>()?;
        Ok(Self { name, output, tpl })
    }
}

fn ident_wrap_to_lit(ident: &Ident) -> LitStr {
    LitStr::new(&ident.to_string(), ident.span())
}

struct NapiShadowInput {
    pub(crate) ident: Ident,
    pub(crate) data: NapiShadowData,
    pub(crate) shadow_root: bool,
    attrs: Vec<Attribute>,
}

enum NapiShadowData {
    Struct(DataStruct),
    Enum(DataEnum),
}

impl From<Data> for NapiShadowData {
    fn from(value: Data) -> Self {
        match value {
            Data::Struct(data_struct) => NapiShadowData::Struct(data_struct),
            Data::Enum(data_enum) => NapiShadowData::Enum(data_enum),
            _ => unimplemented!("unexpected derive input for NapiShadow"),
        }
    }
}

fn split_rep(caps: &Captures) -> String {
    let chs = caps.get(0).unwrap().as_str();
    format!("{}_{}", chs.get(0..=0).unwrap(), chs.get(1..=1).unwrap())
}

fn self_snake_case(input: &str) -> String {
    let step0 = regex::Regex::new("[0-9][A-Za-z]")
        .unwrap()
        .replace_all(input, split_rep)
        .to_string();
    regex::Regex::new("[a-zA-Z][0-9A-Z]")
        .unwrap()
        .replace_all(step0.as_str(), split_rep)
        .to_string()
}

fn self_snake_case_ident(input: &Ident) -> Ident {
    format_ident!(
        "{}",
        self_snake_case(input.to_string().as_str()).to_lowercase()
    )
}

impl ToTokens for NapiShadowInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        #[rustfmt::skip]
        let Self { attrs, ident: origin_ident, data, shadow_root, .. } = self;

        let lit_ident = ident_wrap_to_lit(&origin_ident);
        let snake_ident = self_snake_case_ident(origin_ident);
        let __scope_ident = format_ident!("__shadow_{}", snake_ident);
        let __alias_ident = format_ident!("__Origin_{}", origin_ident);

        if *shadow_root {
            #[rustfmt::skip]
            append_to_tokens(tokens, quote! { impl napi_shadow::NapiShadowRoot for #origin_ident {} });
        }

        let impl_block = match data {
            NapiShadowData::Struct(DataStruct {
                fields: Fields::Named(FieldsNamed { named, .. }),
                ..
            }) => {
                let getters: Vec<_> = named
                    .iter()
                    .filter(|f| matches!(&f.vis, Visibility::Public(_)))
                    .filter(|f| matches!(&f.ident, Some(_)))
                    .filter(
                        |f| matches!(&f.ty, Type::Path(TypePath { qself, .. }) if qself.is_none()),
                    )
                    .filter(|field| {
                        get_meta_by_name(
                            &get_metas_by_attr_name(&field.attrs, ATTR_NAPI_SHADOW),
                            META_SHADOW_SKIP,
                        )
                        .is_none()
                    })
                    .map(|field| {
                        #[rustfmt::skip]
                        let Field { ident: field_ident, ty, .. } = field;

                        let Some(getter_ident) = field_ident else {
                            unreachable!("ident must be used with name")
                        };

                        let lit_getter_ident = ident_wrap_to_lit(&getter_ident);

                        let origin = replace_type_path(
                            &ty,
                            quote! { #origin_ident },
                            quote! { #__alias_ident },
                        );

                        let shadow = quote! {<#origin as napi_shadow::NapiShadow>};

                        quote! {
                            #[napi_derive::napi(getter, js_name = #lit_getter_ident)]
                            pub fn #getter_ident(&self) -> #shadow::ShadowStruct {
                                let Self { itself, root } = self;
                                #shadow::napi_shadow(
                                    &unsafe { itself.as_ref() }.#getter_ident,
                                    std::rc::Rc::clone(root),
                                )
                            }
                        }
                    })
                    .collect();

                let shadow_getters: Vec<_> = attrs
                    .iter()
                    .filter(|attr| matches!(attr.path().get_ident(), Some(ident) if ident == "shadow_getter"))
                    .filter(|attr| matches!(attr.meta, Meta::List(_)))
                    .map(|attr| {
                        let Meta::List(MetaList { tokens, .. }) = &attr.meta else {
                            unreachable!()
                        };

                        let getter = syn::parse::<ShadowGetter>(TokenStream::from(tokens.clone()))
                            .expect("cannot parse meta in shadow_getter");

                        let ShadowGetter {
                            name,
                            output,
                            tpl: ExprClosure { body, .. },
                            ..
                        } = getter;

                        let lit_name = ident_wrap_to_lit(&name);

                        let origin = replace_type_path(&output, quote! { #origin_ident }, quote! { #__alias_ident });

                        let shadow = quote!(<#origin as napi_shadow::NapiShadow>);

                        quote! {
                            #[napi_derive::napi(getter, js_name = #lit_name)]
                            pub fn #name(&self) -> #shadow::ShadowStruct {
                                let Self { itself, root } = self;
                                #body
                            }
                        }
                    })
                    .collect();
                quote! {
                    #[napi_derive::napi]
                    impl #origin_ident {
                        #(#getters)*
                        #(#shadow_getters)*
                    }
                }
            }
            NapiShadowData::Enum(DataEnum { variants, .. }) => {
                let matched_variants: Vec<_> = variants
                    .iter()
                    .filter(|v| matches!(&v.discriminant, None))
                    .filter(|v| matches!(&v.fields, Fields::Unnamed( FieldsUnnamed { unnamed , ..} ) if unnamed.len() == 1))
                    .filter(|variant| {
                        get_meta_by_name(
                            &get_metas_by_attr_name(&variant.attrs, ATTR_NAPI_SHADOW),
                            META_SHADOW_SKIP,
                        )
                            .is_none()
                    }).collect();
                let match_arms: Vec<_> = matched_variants.iter().map(|variant| {
                    let Variant { ident: variant_ident, .. } = variant;
                    quote! {
                        #__alias_ident::#variant_ident(_) => String::from(stringify!(#variant_ident)),
                    }
                }).collect();
                let getters: Vec<_> = matched_variants.iter().map(|variant| {
                        #[rustfmt::skip]
                        let Variant { ident: variant_ident, fields: Fields::Unnamed(FieldsUnnamed { unnamed , ..} ), .. } = variant else {
                            unreachable!("variant is not implemented yet");
                        };

                        let snake_ident = self_snake_case_ident(variant_ident);
                        let getter_ident = format_ident!("get_{}", snake_ident);

                        let lit_getter_ident = ident_wrap_to_lit(&variant_ident);

                        let Some(Field { ty, .. }) = unnamed.first() else { unreachable!() };

                        let origin = replace_type_path(ty, quote! { #origin_ident }, quote! { #__alias_ident });

                        let shadow = quote! {<#origin as napi_shadow::NapiShadow>};

                        quote! {
                            #[napi_derive::napi(getter, js_name = #lit_getter_ident)]
                            pub fn #getter_ident(&self) -> napi::Result<#shadow::ShadowStruct> {
                                let Self { itself, root } = self;
                                match unsafe { itself.as_ref() } {
                                    #__alias_ident::#variant_ident(payload) => Ok(
                                        #shadow::napi_shadow(payload, std::rc::Rc::clone(root))
                                    ),
                                    _ => Err(napi::Error::from_reason(format!("cannot get as {} [type nomatch]", stringify!(#variant_ident)))),
                                }
                            }
                        }
                    })
                    .collect();

                quote! {
                    #[napi_derive::napi]
                    impl #origin_ident {
                        #[napi_derive::napi(getter, js_name = "type")]
                        pub fn get_type(&self) -> String {
                            let Self { itself, .. } = self;

                            match unsafe { itself.as_ref() } {
                                #(#match_arms)*
                            }
                        }

                        #(#getters)*
                    }
                }
            }
            _ => unimplemented!("not implemented for NapiShadow input"),
        };
        // #![allow(unused_variables, unused_imports, dead_code, non_camel_case_types)]

        append_to_tokens(
            tokens,
            quote! {
                mod #__scope_ident {
                    use super::*;
                    #[allow(non_camel_case_types)]
                    type #__alias_ident = super::#origin_ident;

                    #[napi_derive::napi(js_name = #lit_ident)]
                    pub struct #origin_ident {
                        pub(crate) root: std::rc::Rc<dyn napi_shadow::NapiShadowRoot>,
                        pub(crate) itself: std::ptr::NonNull<super::#origin_ident>,
                    }

                    #impl_block
                }
            },
        );

        append_to_tokens(
            tokens,
            quote! {
                impl napi_shadow::NapiShadow for #origin_ident {
                    type ShadowStruct = #__scope_ident::#origin_ident;

                    fn napi_shadow(&self, root: std::rc::Rc<dyn napi_shadow::NapiShadowRoot>) -> Self::ShadowStruct {
                        #__scope_ident::#origin_ident {
                            root: std::rc::Rc::clone(&root),
                            itself: std::ptr::NonNull::from(self),
                        }
                    }
                }
            },
        );
    }
}

pub fn napi_shadow_derive(input: TokenStream) -> TokenStream {
    #[rustfmt::skip]
    let DeriveInput { attrs, ident, data, .. } = parse_macro_input!(input as DeriveInput);

    // let lit_ident = ident_wrap_to_lit(&ident);

    map_meta_to_local!(&get_metas_by_attr_name(&attrs, ATTR_NAPI_SHADOW) => {
        META_SHADOW_ROOT => shadow_root,
    });

    let structured = NapiShadowInput {
        attrs: attrs.clone(),
        ident: ident.clone(),
        data: NapiShadowData::from(data.clone()),
        shadow_root: shadow_root.is_some(),
    };

    proc_macro::TokenStream::from(quote! { #structured })
}
