use crate::common::color_rgb::ColorRgb;
use crate::common::quaternion_f32::QuaternionF32;
use crate::common::string_s16::StringS16;
use crate::common::vector3_f32::Vector3F32;
use binrw::BinRead;
use napi_shadow::NapiShadow;
use std::clone::Clone;

#[derive(NapiShadow, BinRead, Debug)]
pub struct WorldData {
    pub data_item_count: u32,

    #[br(count = data_item_count)]
    pub data_items: Vec<DataItem>,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct DataItem {
    pub item_size: u16,

    pub item_type: StringS16,

    pub item_property_count: u32,

    #[br(count = item_property_count)]
    pub item_properties: Vec<ItemProperty>,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct ItemProperty {
    pub name: StringS16,

    pub data_type: u8,

    pub flags: u32,

    pub data_length: u16,

    #[br(args{ data_type })]
    #[napi_shadow(skip)]
    pub data: ItemPropertyData,
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ data_type: u8 })]
pub enum ItemPropertyData {
    #[br(pre_assert(data_type == 0u8))]
    String(StringS16),

    #[br(pre_assert(data_type == 1u8))]
    Vector3(Vector3F32),

    #[br(pre_assert(data_type == 2u8))]
    Color(ColorRgb),

    #[br(pre_assert(data_type == 3u8))]
    Float32(f32),

    #[br(pre_assert(data_type == 4u8))]
    Uint32(u32),

    #[br(pre_assert(data_type == 5u8))]
    Uint8(u8),

    #[br(pre_assert(data_type == 6u8))]
    Int32(i32),

    #[br(pre_assert(data_type == 7u8))]
    Quaternion(QuaternionF32),
}
