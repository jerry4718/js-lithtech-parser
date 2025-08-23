use crate::common::t::Rgba;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[repr(C)]
pub struct ColorRgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgba<f32> for ColorRgb {
    fn rgba(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, 1.0)
    }
}
