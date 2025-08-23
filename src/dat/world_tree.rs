use crate::common::vector3_f32::Vector3F32;
use crate::dat::world_model::WorldModel;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct WorldTree {
    pub box_min: Vector3F32,

    pub box_max: Vector3F32,

    pub child_num_nodes: u32,

    pub dummy_terrain_depth: u32,

    #[br(count = child_num_nodes / 8 + 1)]
    pub world_layout: Vec<u8>,

    pub world_model_count: u32,

    #[br(count = world_model_count)]
    pub world_models: Vec<WorldModel>,
}
