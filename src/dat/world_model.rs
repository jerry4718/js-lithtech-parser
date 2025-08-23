use crate::common::string_s16::StringS16;
use crate::common::string_until0::StringUntil0;
use crate::common::vector3_f32::Vector3F32;
use crate::dat::plane::Plane;
use crate::dat::surface::Surface;
use binrw::helpers::args_iter;
use binrw::{args, BinRead};
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct WorldModel {
    pub reserved: u32,

    pub world_info_flag: u32,

    pub world_name: StringS16,

    pub num_points: u32,

    pub num_planes: u32,

    pub num_surfaces: u32,

    pub reserved1: u32,

    pub num_polygons: u32,

    pub reserved2: u32,

    pub num_polygon_vertex_indexes: u32,

    pub reserved3: u32,

    pub reserved4: u32,

    pub num_nodes: u32,

    pub box_min: Vector3F32,

    pub box_max: Vector3F32,

    pub world_translation: Vector3F32,

    pub texture_name_size: u32,

    pub num_texture_names: u32,

    #[br(count = num_texture_names)]
    pub texture_names: Vec<StringUntil0>,

    #[br(count = num_polygons)]
    pub vertex_count_list: Vec<u8>,

    #[br(count = num_planes)]
    pub planes: Vec<Plane>,

    #[br(count = num_surfaces)]
    pub surfaces: Vec<Surface>,

    #[br(
        parse_with = args_iter(
            (0..num_polygons).map(|idx| -> <WorldModelPolygon as BinRead>::Args<'_> {
                args! { vertex_count: vertex_count_list[idx as usize] }
            })
        )
    )]
    pub polygons: Vec<WorldModelPolygon>,

    #[br(count = num_nodes)]
    pub nodes: Vec<WorldModelNode>,

    #[br(count = num_points)]
    pub points: Vec<Vector3F32>,

    pub root_node_index: i32,

    pub sections: u32,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct WorldModelNode {
    pub poly_index: u32,

    pub reserved: u16,

    #[br(count = 2)]
    pub node_sides_indices: Vec<i32>,
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ vertex_count: u8 })]
pub struct WorldModelPolygon {
    pub surface_index: u32,

    pub plane_index: u32,

    #[br(count=vertex_count)]
    pub vertex_indexes: Vec<u32>,
}
