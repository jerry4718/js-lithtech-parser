use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct DtxSection {
    #[br(count = 15)]
    pub section_type_data: Vec<u8>,
    #[br(count = 10)]
    pub section_name_data: Vec<u8>,

    #[br(calc = String::from_utf8_lossy(&section_type_data).to_string())]
    pub section_type: String,
    #[br(calc = String::from_utf8_lossy(&section_name_data).to_string())]
    pub section_name: String,

    #[br(dbg)]
    pub data_count: u32, // Data length, not including SectionHeader.
    #[br(count = 0)]
    pub data: Vec<u8>,
}
