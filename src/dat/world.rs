use crate::common::string_s32::StringS32;
use crate::common::vector3_f32::Vector3F32;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct World {
    pub properties: StringS32,

    pub extents_min: Vector3F32,

    pub extents_max: Vector3F32,

    pub world_offset: Vector3F32,
}
