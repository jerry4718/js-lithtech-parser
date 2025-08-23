use crate::common::string_s16::StringS16;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct LtbHeader {
    pub file_type: u16,
    pub file_version: u16,
    #[br(count = 4)]
    pub space_unknown: Vec<u32>,
    pub obb_version: u32,
    pub keyframe_count: u32,
    pub animation_count: u32,
    pub node_count: u32,
    pub pieces_no_use_count: u32,
    pub child_model_count: u32,
    pub face_count: u32,
    pub vertex_count: u32,
    pub vertex_weight_count: u32,
    pub lod_count: u32,
    pub socket_count: u32,
    pub weight_set_count: u32,
    pub string_count: u32,
    pub string_length: u32,
    pub vertex_animation_data_size: u32,
    pub animation_data_size: u32,
    pub command_string: StringS16,
    pub internal_radius: f32,
}
