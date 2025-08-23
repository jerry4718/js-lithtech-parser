use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct Triangle {
    #[br(count = 3)]
    pub vertex_indexes: Vec<u32>,

    pub poly_index: u32,
}
