use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[repr(C)]
pub struct DatHeader {
    #[br(count = 4)]
    pub dat_version: Vec<u8>,

    pub world_data_pos: u32,

    pub blind_data_pos: u32,

    pub light_data_pos: u32,

    pub physics_data_pos: u32,

    pub particle_data_pos: u32,

    pub render_data_pos: u32,

    #[br(count = 8)]
    pub future: Vec<u32>,
}
