use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct Surface {
    pub flags: u32,

    pub texture_index: u16,

    pub texture_flags: u16,
}
