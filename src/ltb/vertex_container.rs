use crate::common::color_rgba::ColorRgba;
use crate::common::vector2_f32::Vector2F32;
use crate::common::vector3_f32::Vector3F32;
use crate::ltb::lod_container::MESH_SKELETAL;
use binrw::{args, BinRead};
use napi_shadow::NapiShadow;
use std::convert::Into;

const MASK_POSITION: u32 = 1u32 << 0;
const MASK_NORMAL: u32 = 1u32 << 1;
const MASK_COLOR: u32 = 1u32 << 2;
const MASK_UV1: u32 = 1u32 << 4;
const MASK_UV2: u32 = 1u32 << 5;
const MASK_UV3: u32 = 1u32 << 6;
const MASK_UV4: u32 = 1u32 << 7;
const MASK_BASIS_VECTOR: u32 = 1u32 << 8;

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ vertex_count: u32, vertex_type: u32, mesh_type: u32, face_max_bone: u32 })]
pub struct VertexGroup {
    #[br(
        if(vertex_type > 0),
        count = vertex_count,
        args { inner: args! { vertex_type, mesh_type, face_max_bone } }
    )]
    pub vertexes: Option<Vec<LtbVertex>>,
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ vertex_type: u32, mesh_type: u32, face_max_bone: u32})]
pub struct LtbVertex {
    #[br(if(vertex_type & MASK_POSITION > 0))]
    pub position: Option<Vector3F32>,
    #[br(if(vertex_type & MASK_POSITION > 0 && mesh_type == MESH_SKELETAL), count = face_max_bone - 1)]
    pub weight_blend: Option<Vec<f32>>,
    #[br(if(vertex_type & MASK_NORMAL > 0))]
    pub normal: Option<Vector3F32>,
    #[br(if(vertex_type & MASK_COLOR > 0))]
    pub color: Option<ColorRgba>,
    #[br(if(vertex_type & MASK_UV1 > 0))]
    pub uv1: Option<Vector2F32>,
    #[br(if(vertex_type & MASK_UV2 > 0))]
    pub uv2: Option<Vector2F32>,
    #[br(if(vertex_type & MASK_UV3 > 0))]
    pub uv3: Option<Vector2F32>,
    #[br(if(vertex_type & MASK_UV4 > 0))]
    pub uv4: Option<Vector2F32>,
    #[br(if(vertex_type & MASK_BASIS_VECTOR > 0))]
    pub s: Option<Vector3F32>,
    #[br(if(vertex_type & MASK_BASIS_VECTOR > 0))]
    pub t: Option<Vector3F32>,
}
