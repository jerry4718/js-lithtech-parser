use crate::common::color_argb::ColorArgb;
use crate::dtx::dtx_header::DtxHeader;
use crate::dtx::image::calc_colour_data;
use crate::dtx::mipmap::Mipmap;
use binrw::helpers::args_iter;
use binrw::{args, BinRead};
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ header: &DtxHeader })]
pub struct Palette8Bit {
    #[br(parse_with = args_iter((0..header.mipmap_count).map(|level| -> <Mipmap as BinRead>::Args<'_> {
        let width = header.width >> level;
        let height = header.height >> level;
        args! {
            width,
            height,
            data_size: usize::from(width) * usize::from(height) * 4
        }
    })))]
    pub mipmaps: Vec<Mipmap>,
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ palette: &Vec<ColorArgb>, data: &Vec<u8> })]
pub struct Colour8bit {
    #[br(calc = calc_colour_data(&palette, &data))]
    pub data: Vec<u8>,
}
