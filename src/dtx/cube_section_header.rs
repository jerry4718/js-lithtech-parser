use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct DtxSection {
    #[br(count = 15)]
    #[napi_shadow(skip)]
    pub section_type: Vec<u8>,
    #[br(count = 10)]
    #[napi_shadow(skip)]
    pub section_name: Vec<u8>,

    #[br(calc = String::from_utf8_lossy(&section_type).to_string())]
    pub section_type_string: String,
    #[br(calc = String::from_utf8_lossy(&section_name).to_string())]
    pub section_name_string: String,

    #[br(dbg)]
    pub data_count: u32, // Data length, not including SectionHeader.
    #[br(count = 0)]
    pub data: Vec<u8>,
}
