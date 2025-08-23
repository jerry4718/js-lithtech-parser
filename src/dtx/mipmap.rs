use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ width: u16, height: u16, data_size: usize })]
pub struct Mipmap {
    #[br(calc = width)]
    pub width: u16,
    #[br(calc = height)]
    pub height: u16,
    #[br(count = data_size)]
    pub data: Vec<u8>,
}
