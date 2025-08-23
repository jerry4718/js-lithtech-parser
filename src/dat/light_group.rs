use crate::common::color_rgb::ColorRgb;
use crate::common::string_s16::StringS16;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct LightGroup {
    pub name: StringS16,

    pub color: ColorRgb,

    pub intensity_data_len: u32,

    #[br(count = intensity_data_len)]
    pub intensity_data: Vec<u8>,

    pub section_row_count: u32,

    #[br(count = section_row_count)]
    pub section_rows: Vec<LightMapSectionRow>,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct LightMapSectionRow {
    pub section_count: u32,

    #[br(count = section_count)]
    pub sections: Vec<LightMapSection>,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct LightMapSection {
    pub left: u32,

    pub top: u32,

    pub width: u32,

    pub height: u32,

    pub data_len: u32,

    #[br(count = data_len)]
    pub data: Vec<u8>,
}
