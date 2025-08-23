use crate::common::color_bgra::ColorBgra;
use crate::dtx::dtx_header::DtxHeader;
use crate::dtx::image::calc_colour_data;
use crate::dtx::mipmap::Mipmap;
use binrw::helpers::args_iter;
use binrw::{args, BinRead};
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ header: &DtxHeader })]
#[br(assert(
    header.section_count == 1,
    "Section count is not 1, even though we're a 32bit palette texture! Count: {}",
    header.section_count
))]
pub struct Palette32Bit {
    // FORMAT_RGBA8
    #[br(count = usize::from(header.width) * usize::from(header.height))]
    pub data: Vec<u8>,

    // TODO: Actually use this
    // We need to skip past the mipmaps!
    #[br(parse_with = args_iter((0..header.mipmap_count).map(|level| -> <Mipmap as BinRead>::Args<'_> {
        let width = header.width >> level;
        let height = header.height >> level;
        args! {
            width,
            height,
            data_size: usize::from(width) * usize::from(height)
        }
    })))]
    pub mipmaps: Vec<Mipmap>,

    // Useless bits!
    #[br(count = 16)]
    pub section_type: Vec<u8>,
    #[br(count = 10)]
    pub section_unk: Vec<u8>,
    #[br(count = 2)]
    pub section_filler: Vec<u8>, // skip 2 filler bytes!
    pub section_length: u32,

    // Handle the palette
    #[br(count = 256)]
    pub palette: Vec<ColorBgra>,

    #[br(args{ palette: &palette, data: &data })]
    pub colour_data: Colour32bit,
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ palette: &Vec<ColorBgra>, data: &Vec<u8> })]
pub struct Colour32bit {
    #[br(calc = calc_colour_data(&palette, &data))]
    pub data: Vec<u8>,
}
