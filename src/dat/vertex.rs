#![allow(dead_code)]

use crate::common::color_rgba::ColorRgba;
use crate::common::vector2_f32::Vector2F32;
use crate::common::vector3_f32::Vector3F32;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct DatVertex {
    pos: Vector3F32,
    uv1: Vector2F32,
    uv2: Vector2F32,
    color: ColorRgba,
    normal: Vector3F32,
}
