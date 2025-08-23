use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[repr(C)]
pub struct Vector2F32 {
    pub x: f32,
    pub y: f32,
}
