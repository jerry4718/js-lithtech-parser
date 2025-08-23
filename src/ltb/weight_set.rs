use crate::common::string_s16::StringS16;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct WeightSet {
    pub name: StringS16,
    pub weight_count: u32,
    #[br(count = weight_count)]
    pub weights: Vec<f32>,
}
