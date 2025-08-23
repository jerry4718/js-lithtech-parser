use crate::common::string_s16::StringS16;
use crate::ltb::lod_container::LodContainer;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct Piece {
    pub name: StringS16,
    pub lod_count: u32,
    #[br(count = lod_count)]
    pub lod_distances: Vec<f32>,
    pub lod_min: u32,
    pub lod_max: u32,
    #[br(count = lod_count)]
    pub lod_containers: Vec<LodContainer>,
}
