use binrw::BinRead;
use napi_shadow::{NapiShadow, NapiShadowRoot};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(BinRead)]
#[repr(C)]
pub struct StringS16 {
    pub len: u16,

    #[br(count=len)]
    pub data: Vec<u8>,
}

impl Debug for StringS16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.data))
    }
}

impl StringS16 {
    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }
}

impl NapiShadow for StringS16 {
    type ShadowStruct = String;

    fn napi_shadow(&self, _: Rc<dyn NapiShadowRoot>) -> Self::ShadowStruct {
        self.to_string()
    }
}
