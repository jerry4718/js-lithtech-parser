use crate::ltb::vertex_container::VertexGroup;
use binrw::helpers::args_iter;
use binrw::{args, BinRead, BinResult, Endian};
use napi_shadow::NapiShadow;
use std::io::{Read, Seek};

pub const MESH_RIGID: u32 = 4;
pub const MESH_SKELETAL: u32 = 5;
pub const MESH_VERTEX_ANIMATED: u32 = 6;
pub const MESH_NULL: u32 = 7;

#[derive(NapiShadow, BinRead, Debug)]
pub struct LodContainer {
    pub texture_count: u32,
    #[br(count = 4)]
    pub textures: Vec<u32>,
    pub render_style: u32,
    pub render_priority: u8,
    pub mesh_type: u32,
    #[br(args { mesh_type })]
    #[napi_shadow(skip)]
    pub mesh: LodMesh,
    pub used_node_count: u8,
    #[br(count = used_node_count)]
    pub used_nodes: Vec<u8>,
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ mesh_type: u32 })]
pub enum LodMesh {
    #[br(pre_assert(mesh_type == MESH_RIGID))]
    RigidMesh(#[br(args { mesh_type })] RigidMesh),

    #[br(pre_assert(mesh_type == MESH_SKELETAL))]
    SkeletalMesh(#[br(args { mesh_type })] SkeletalMesh),

    #[br(pre_assert(mesh_type == MESH_VERTEX_ANIMATED))]
    VertexAnimatedMesh(VertexAnimatedMesh),

    #[br(pre_assert(mesh_type == MESH_NULL))]
    NullMesh(NullMesh),

    #[br(pre_assert(mesh_type != MESH_RIGID && mesh_type != MESH_SKELETAL && mesh_type != MESH_VERTEX_ANIMATED && mesh_type != MESH_NULL))]
    UnknownMesh(UnknownMesh),
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct LodMeshInfo {
    pub obj_size: u32,
    pub vertex_count: u32,
    pub face_count: u32,
    pub face_max_bone: u32,
    pub vert_max_bone: u32,
}

fn parse_vertex_groups<'a, R: Read + Seek>(
    mesh_type: u32,
    info: &'a LodMeshInfo,
    vertex_type_map: &'a Vec<u32>,
) -> impl FnOnce(&mut R, Endian, ()) -> BinResult<Vec<VertexGroup>> + use<'a, R> {
    args_iter(
        (0..4).map(move |idx| -> <VertexGroup as BinRead>::Args<'_> {
            args! {
                vertex_count: info.vertex_count,
                vertex_type: vertex_type_map[idx],
                mesh_type,
                face_max_bone: info.face_max_bone,
            }
        }),
    )
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ mesh_type: u32 })]
pub struct RigidMesh {
    pub info: LodMeshInfo,
    #[br(count = 4)]
    pub vertex_type_map: Vec<u32>,
    pub bone: u32,
    #[br(parse_with = parse_vertex_groups(mesh_type, &info, &vertex_type_map))]
    pub vertex_groups: Vec<VertexGroup>,
    #[br(count = info.face_count * 3)]
    pub vertex_index: Vec<u16>,
}

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ mesh_type: u32 })]
pub struct SkeletalMesh {
    pub info: LodMeshInfo,
    pub re_indexed_bone: u8,
    #[br(count = 4)]
    pub vertex_type_map: Vec<u32>,
    pub matrix_palette: u8,
    #[br(parse_with = parse_vertex_groups(mesh_type, &info, &vertex_type_map))]
    pub vertex_groups: Vec<VertexGroup>,
    #[br(count = info.face_count * 3)]
    pub vertex_index: Vec<u16>,
    pub bone_set_count: u32,
    #[br(count = bone_set_count)]
    pub bone_set: Vec<BoneSet>,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct BoneSet {
    pub index_start: u16,
    pub index_count: u16,
    #[br(count = 4)]
    pub bone_list: Vec<u8>,
    pub index_buffer_index: u32,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct VertexAnimatedMesh {
    pub info: LodMeshInfo,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct NullMesh {
    pub offset: u8,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct UnknownMesh {
    pub info: LodMeshInfo,
}
