use crate::common::matrix::Matrix;
use crate::common::string_s16::StringS16;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct BoneNode {
    pub name: StringS16,
    pub index: u16,
    pub flags: u8,
    pub bind_matrix: Matrix,
    pub child_count: u32,
    #[br(count = child_count)]
    pub children: Vec<BoneNode>,
}
