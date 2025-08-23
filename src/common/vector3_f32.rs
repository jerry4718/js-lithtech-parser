use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[repr(C)]
pub struct Vector3F32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
