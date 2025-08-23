use crate::common::string_s16::StringS16;
use crate::common::vector3_f32::Vector3F32;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct AnimBinding {
    pub name: StringS16,
    pub extents: Vector3F32,
    pub origin: Vector3F32,
}
