use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[repr(C)]
pub struct QuaternionF32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
