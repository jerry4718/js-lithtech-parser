use binrw::helpers::until;
use binrw::BinRead;
use napi_shadow::{NapiShadow, NapiShadowRoot};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(BinRead)]
#[repr(C)]
pub struct StringUntil0 {
    #[br(parse_with = until(|&byte| byte == 0x00))]
    pub data: Vec<u8>,
}

impl Debug for StringUntil0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.data))
    }
}

impl StringUntil0 {
    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }
}

impl NapiShadow for StringUntil0 {
    type ShadowStruct = String;

    fn napi_shadow(&self, _: Rc<dyn NapiShadowRoot>) -> Self::ShadowStruct {
        self.to_string()
    }
}
