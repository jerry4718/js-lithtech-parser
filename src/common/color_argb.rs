use crate::common::t::Rgba;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[repr(C)]
pub struct ColorArgb {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgba<u8> for ColorArgb {
    fn rgba(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}
