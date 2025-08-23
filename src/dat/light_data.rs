use crate::common::vector3_f32::Vector3F32;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct LightData {
    pub lookup_start: Vector3F32,

    pub block_size: Vector3F32,

    #[br(count = 3)]
    pub lookup_size: Vec<u32>,

    pub light_data_grid_count: u32,

    /* RLE compressed data */
    #[br(count = light_data_grid_count)]
    pub light_data_grid: Vec<u8>,
}
