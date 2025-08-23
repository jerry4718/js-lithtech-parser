use crate::common::vector3_f32::Vector3F32;
use binrw::BinRead;
use napi_shadow::{NapiShadow, NapiShadowRoot};
use std::rc::Rc;

const DECOMPRESS_VALUE: f32 = 16.0f32;

#[derive(BinRead, Debug)]
#[repr(C)]
pub struct Vector3I16Compressed {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl From<Vector3I16Compressed> for Vector3F32 {
    fn from(value: Vector3I16Compressed) -> Self {
        let Vector3I16Compressed { x, y, z } = value;
        Self {
            x: f32::from(x) / DECOMPRESS_VALUE,
            y: f32::from(y) / DECOMPRESS_VALUE,
            z: f32::from(z) / DECOMPRESS_VALUE,
        }
    }
}

impl From<&Vector3I16Compressed> for Vector3F32 {
    fn from(value: &Vector3I16Compressed) -> Self {
        let Vector3I16Compressed { x, y, z } = value;
        Self {
            x: f32::from(*x) / DECOMPRESS_VALUE,
            y: f32::from(*y) / DECOMPRESS_VALUE,
            z: f32::from(*z) / DECOMPRESS_VALUE,
        }
    }
}

impl NapiShadow for Vector3I16Compressed {
    type ShadowStruct = <Vector3F32 as NapiShadow>::ShadowStruct;

    fn napi_shadow(&self, root: Rc<dyn NapiShadowRoot>) -> Self::ShadowStruct {
        let Self { x, y, z } = *self;
        let vector_f32 = Vector3F32 {
            x: f32::from(x) / DECOMPRESS_VALUE,
            y: f32::from(y) / DECOMPRESS_VALUE,
            z: f32::from(z) / DECOMPRESS_VALUE,
        };
        <Vector3F32 as NapiShadow>::napi_shadow(&vector_f32, root)
    }
}
