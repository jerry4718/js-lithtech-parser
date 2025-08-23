use crate::common::quaternion_f32::QuaternionF32;
use binrw::BinRead;
use napi_shadow::{NapiShadow, NapiShadowRoot};
use std::convert::Into;
use std::fmt::Debug;
use std::rc::Rc;

const DECOMPRESS_VALUE: f32 = 0x7fffu16 as f32;

#[derive(BinRead, Debug)]
#[repr(C)]
pub struct QuaternionI16Compressed {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}

impl From<QuaternionI16Compressed> for QuaternionF32 {
    fn from(value: QuaternionI16Compressed) -> Self {
        let QuaternionI16Compressed { x, y, z, w } = value;
        Self {
            x: f32::from(x) / DECOMPRESS_VALUE,
            y: f32::from(y) / DECOMPRESS_VALUE,
            z: f32::from(z) / DECOMPRESS_VALUE,
            w: f32::from(w) / DECOMPRESS_VALUE,
        }
    }
}

impl From<&QuaternionI16Compressed> for QuaternionF32 {
    fn from(value: &QuaternionI16Compressed) -> Self {
        let QuaternionI16Compressed { x, y, z, w } = value;
        Self {
            x: f32::from(*x) / DECOMPRESS_VALUE,
            y: f32::from(*y) / DECOMPRESS_VALUE,
            z: f32::from(*z) / DECOMPRESS_VALUE,
            w: f32::from(*w) / DECOMPRESS_VALUE,
        }
    }
}

impl NapiShadow for QuaternionI16Compressed {
    type ShadowStruct = <QuaternionF32 as NapiShadow>::ShadowStruct;

    fn napi_shadow(&self, root: Rc<dyn NapiShadowRoot>) -> Self::ShadowStruct {
        let Self { x, y, z, w } = *self;
        let quaternion_f32 = QuaternionF32 {
            x: f32::from(x) / DECOMPRESS_VALUE,
            y: f32::from(y) / DECOMPRESS_VALUE,
            z: f32::from(z) / DECOMPRESS_VALUE,
            w: f32::from(w) / DECOMPRESS_VALUE,
        };
        <QuaternionF32 as NapiShadow>::napi_shadow(&quaternion_f32, root)
    }
}
