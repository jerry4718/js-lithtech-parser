use crate::common::vector3_f32::Vector3F32;
use crate::dat::plane::Plane;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct Polygon {
    pub plane: Plane,

    pub vertex_count: u32,

    #[br(count = vertex_count)]
    pub vertexes: Vec<Vector3F32>,
}
