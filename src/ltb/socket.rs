use crate::common::quaternion_f32::QuaternionF32;
use crate::common::string_s16::StringS16;
use crate::common::vector3_f32::Vector3F32;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct Socket {
    pub node_index: u32,
    pub name: StringS16,
    pub rotation: QuaternionF32,
    pub position: Vector3F32,
    pub scale: Vector3F32,
}
