use crate::common::string_s16::StringS16;
use crate::common::vector3_f32::Vector3F32;
use crate::dat::light_group::LightGroup;
use crate::dat::shader_poly::ShaderPoly;
use crate::dat::sky_portal::SkyPortal;
use crate::dat::triangle::Triangle;
use crate::dat::vertex::DatVertex;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct RenderData {
    pub render_block_count: u32,

    #[br(count = render_block_count)]
    pub render_blocks: Vec<RenderBlock>,

    pub world_model_render_block_count: u32,

    #[br(count = world_model_render_block_count)]
    pub world_model_render_blocks: Vec<WorldModelRenderBlock>,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct RenderBlock {
    pub center: Vector3F32,

    pub half_dims: Vector3F32,

    pub section_count: u32,

    #[br(count = section_count)]
    pub sections: Vec<RenderSection>,

    pub vertex_count: u32,

    #[br(count = vertex_count)]
    pub vertexes: Vec<DatVertex>,

    pub triangle_count: u32,

    #[br(count = triangle_count)]
    pub triangles: Vec<Triangle>,

    pub sky_portal_count: u32,

    #[br(count = sky_portal_count)]
    pub sky_portals: Vec<SkyPortal>,

    pub shader_count: u32,

    #[br(count = shader_count)]
    pub shaders: Vec<ShaderPoly>,

    pub light_group_count: u32,

    #[br(count = light_group_count)]
    pub light_groups: Vec<LightGroup>,

    pub child_flags: u8,

    #[br(count = 2)]
    pub child_node_indices: Vec<u32>,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct RenderSection {
    #[br(count = 2)]
    pub textures: Vec<StringS16>,

    pub shader_code: u8,

    pub num_triangles: u32,

    pub texture_effect: StringS16,

    pub light_map_width: u32,

    pub light_map_height: u32,

    pub light_map_len: u32,

    #[br(count = light_map_len)]
    pub light_map: Vec<u8>,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct WorldModelRenderBlock {
    pub name: StringS16,

    pub render_block_count: u32,

    #[br(count = render_block_count)]
    pub render_blocks: Vec<RenderBlock>,

    pub no_child_flag: u32,
}
