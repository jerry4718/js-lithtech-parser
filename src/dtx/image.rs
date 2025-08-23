use crate::common::t::Rgba;
use crate::dtx::dtx_header::DtxHeader;
use crate::dtx::image_compressed::Compressed;
use crate::dtx::image_palette_32bit::Palette32Bit;
use crate::dtx::image_palette_8bit::Palette8Bit;
use crate::dtx::image_texture_32bit::Texture32Bit;
use crate::dtx::lithtech_dtx::DTX_VERSION_LT15;
use crate::dtx::lithtech_dtx::{BPP_32, BPP_32P, BPP_8P, BPP_S3TC_DXT3, BPP_S3TC_DXT5};
use crate::dtx::lithtech_dtx::{BPP_S3TC_DXT1, DTX_VERSION_LT1};
use binrw::BinRead;
use napi_shadow::NapiShadow;

pub fn is_palette_8bit(header: &DtxHeader) -> bool {
    header.nice_version == DTX_VERSION_LT1
        || header.nice_version == DTX_VERSION_LT15
        || header.bpp_identifier == BPP_8P
}

pub fn is_compressed(header: &DtxHeader) -> bool {
    match header.bpp_identifier {
        BPP_S3TC_DXT1 => true,
        BPP_S3TC_DXT3 => true,
        BPP_S3TC_DXT5 => true,
        _ => false,
    }
}

pub fn is_texture_32bit(header: &DtxHeader) -> bool {
    header.bpp_identifier == BPP_32
}
pub fn is_palette_32bit(header: &DtxHeader) -> bool {
    header.bpp_identifier == BPP_32P
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ header: &DtxHeader })]
pub enum ImageMeta {
    #[br(pre_assert(is_palette_8bit(header)))]
    Palette8Bit(#[br(args{ header })] Palette8Bit),
    #[br(pre_assert(is_compressed(header)))]
    Compressed(#[br(args{ header })] Compressed),
    #[br(pre_assert(is_texture_32bit(header)))]
    Texture32Bit(#[br(args{ header })] Texture32Bit),
    #[br(pre_assert(is_palette_32bit(header)))]
    Palette32Bit(#[br(args{ header })] Palette32Bit),
}

pub fn calc_colour_data<T>(palette: &Vec<T>, data: &Vec<u8>) -> Vec<u8>
where
    T: Rgba<u8>,
{
    let mut colour_data: Vec<u8> = Vec::new();
    for idx in data {
        let (r, g, b, a) = Rgba::<u8>::rgba(&palette[*idx as usize]);
        colour_data.push(r);
        colour_data.push(g);
        colour_data.push(b);
        colour_data.push(a);
    }
    colour_data
}
