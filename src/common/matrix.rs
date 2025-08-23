use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[repr(C)]
pub struct Matrix {
    #[br(count = 4 * 4)]
    data: Vec<f32>, /* [f32; 4 * 4] */
}
