use crate::dtx::cube_section_header::DtxSection;
use crate::dtx::dtx_header::DtxHeader;
use crate::dtx::lithtech_dtx::DTX_CUBEMAP;
use crate::dtx::mipmap::Mipmap;
use binrw::helpers::args_iter;
use binrw::{args, BinRead};
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ header: &DtxHeader })]
pub struct Texture32Bit {
    #[br(calc = if header.flags & DTX_CUBEMAP == 0 { 1 } else { 6 })]
    pub row_count: u16,

    #[br(parse_with = args_iter((0..row_count).map(|rdx| -> <Texture32BitTextureRow as BinRead>::Args<'_> {
        args! { width: header.width, height: header.height, mipmap_count: header.mipmap_count, flags: header.flags, rdx }
    })))]
    pub texture_rows: Vec<Texture32BitTextureRow>,
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import { width: u16, height: u16, mipmap_count: u16, flags: u32, rdx: u16 })]
pub struct Texture32BitTextureRow {
    // FORMAT_RGBA8
    #[br(parse_with = args_iter((0..mipmap_count).map(|level| -> <Mipmap as BinRead>::Args<'_> {
        let width = width >> level;
        let height = height >> level;
        args! {
            width,
            height,
            data_size: usize::from(width) * usize::from(height) * 4
        }
    })))]
    pub mipmaps: Vec<Mipmap>,
    #[br(if(flags & DTX_CUBEMAP > 0 && rdx == 0))]
    pub dtx_section: Option<DtxSection>,
}
