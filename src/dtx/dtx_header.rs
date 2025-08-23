use crate::dtx::lithtech_dtx::COMMAND_STRING_VERSIONS;
use crate::dtx::lithtech_dtx::DTX_COMMAND_STRING_LENGTH;
use crate::dtx::lithtech_dtx::SUPPORT_VERSIONS;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[br(assert(
    SUPPORT_VERSIONS.contains(&nice_version),
    "Unsupported file version: (resource_type: 0x{:08x}, version: 0x{:08x}, nice_version: {})",
    resource_type, version, nice_version
))]
#[shadow_getter(command_string: Option<String> = || get_command_string(unsafe { itself.as_ref() }))]
pub struct DtxHeader {
    pub resource_type: u32,
    #[br(if(resource_type == 0, resource_type))]
    pub version: u32,
    #[br(calc = u32::cast_signed(version))]
    pub nice_version: i32,

    pub width: u16,
    pub height: u16,
    pub mipmap_count: u16,
    pub section_count: u16,
    pub flags: u32,
    pub user_flags: u32,
    // Extra data - this may be not be entirely correct for DTX_VERSION_LT1
    pub texture_group: u8,
    pub mipmaps_to_use: u8,
    pub bpp_identifier: u8,
    pub mipmap_offset: u8,
    pub mipmap_texcoord_offset: u8,
    pub texture_priority: u8,
    pub detail_texture_scale: f32,
    pub detail_texture_angle: u16,

    #[br(if(COMMAND_STRING_VERSIONS.contains(&nice_version)), count = DTX_COMMAND_STRING_LENGTH)]
    pub command_data: Option<Vec<u8>>,
}

fn get_command_string(header: &DtxHeader) -> Option<String> {
    if let Some(data) = &header.command_data {
        Some(String::from_utf8_lossy(data).to_string())
    } else {
        None
    }
}
