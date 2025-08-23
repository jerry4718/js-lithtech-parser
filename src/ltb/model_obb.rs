use crate::common::vector3_f32::Vector3F32;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[allow(dead_code)]
const OBB_V23: u32 = 23;
const OBB_V24: u32 = 24;
const OBB_V25: u32 = 25;

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ obb_version: u32 })]
pub struct ModelOBB {
    pub pos: Vector3F32,
    pub size: Vector3F32,
    #[br(count = 3)]
    pub basis: Vec<Vector3F32>,
    pub i_node: u32,
    /*
    if (scope.header.version === ObbVersion.V23) return false;
    if (scope.header.version === ObbVersion.V24) return true;
    if (scope.header.version === ObbVersion.V25) return true;
    */
    #[br(if(obb_version == OBB_V24 || obb_version == OBB_V25))]
    pub radius: Option<f32>,
}
