use crate::napi_shadow_root::NapiShadowRoot;
use std::rc::Rc;

pub trait NapiShadow {
    type ShadowStruct;

    fn napi_shadow(&self, root: Rc<dyn NapiShadowRoot>) -> Self::ShadowStruct;
}

impl<T, S> NapiShadow for Vec<T>
where
    T: NapiShadow<ShadowStruct = S>,
{
    type ShadowStruct = Vec<S>;

    fn napi_shadow(&self, root: Rc<dyn NapiShadowRoot>) -> Self::ShadowStruct {
        self.iter()
            .map(|item| NapiShadow::napi_shadow(item, Rc::clone(&root)))
            .collect()
    }
}

impl<T, S> NapiShadow for Option<T>
where
    T: NapiShadow<ShadowStruct = S>,
{
    type ShadowStruct = Option<S>;

    fn napi_shadow(&self, root: Rc<dyn NapiShadowRoot>) -> Self::ShadowStruct {
        match self {
            None => None,
            Some(value) => Some(NapiShadow::napi_shadow(value, Rc::clone(&root))),
        }
    }
}

macro_rules! impl_with_deref_for {
    ($($ty:ty)*) => {
        $(
        impl NapiShadow for $ty {
            type ShadowStruct = $ty;

            fn napi_shadow(&self, _root: Rc<dyn NapiShadowRoot>) -> Self::ShadowStruct {
                *self
            }
        }
        )*
    };
}

impl_with_deref_for!(u8 i8 u16 i16 u32 i32 u64 i64 usize f32 f64 bool);

impl NapiShadow for String {
    type ShadowStruct = String;

    fn napi_shadow(&self, _root: Rc<dyn NapiShadowRoot>) -> Self::ShadowStruct {
        self.to_string()
    }
}
