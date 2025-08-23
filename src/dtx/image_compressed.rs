use crate::dtx::cube_section_header::DtxSection;
use crate::dtx::dtx_header::DtxHeader;
use crate::dtx::lithtech_dtx::DTX_CUBEMAP;
use crate::dtx::lithtech_dtx::{BPP_S3TC_DXT3, BPP_S3TC_DXT5};
use crate::dtx::mipmap::Mipmap;
use binrw::helpers::args_iter;
use binrw::{args, BinRead};
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ header: &DtxHeader })]
pub struct Compressed {
    #[br(calc = if header.flags & DTX_CUBEMAP == 0 { 1 } else { 6 })]
    pub row_count: u16,

    #[br(parse_with = args_iter((0..row_count).map(|rdx| -> <CompressedTextureRow as BinRead>::Args<'_> {
        args! {
            width: header.width, height: header.height,
            mipmap_count: header.mipmap_count,
            flags: header.flags,
            bpp: header.bpp_identifier,
            rdx
        }
    })))]
    pub texture_rows: Vec<CompressedTextureRow>,
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ width: u16, height: u16, mipmap_count: u16, flags: u32, bpp: u8, rdx: u16 })]
pub struct CompressedTextureRow {
    // FORMAT_RGBA8
    #[br(parse_with = args_iter((0..mipmap_count).map(|level| -> <Mipmap as BinRead>::Args<'_> {
        let width = width >> level;
        let height = height >> level;
        args! {
            width,
            height,
            data_size: calc_compressed_size(width, height, bpp)
        }
    })))]
    pub mipmaps: Vec<Mipmap>,
    #[br(if(flags & DTX_CUBEMAP > 0 && rdx == 0))]
    pub dtx_section: Option<DtxSection>,
}

fn calc_compressed_size(width: u16, height: u16, bpp: u8) -> usize {
    let width = (width + 3) / 4;
    let height = (height + 3) / 4;
    let pixel_count = usize::from(width) * usize::from(height);
    match bpp {
        BPP_S3TC_DXT3 => pixel_count * 16,
        BPP_S3TC_DXT5 => pixel_count * 16,
        // DXT1 - Defaults
        _ => pixel_count * 8, // Extra bytes needed in the decoding process
    }
}
