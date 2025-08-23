use crate::common::quaternion_f32::QuaternionF32;
use crate::common::quaternion_i16_compressed::QuaternionI16Compressed;
use crate::common::string_s16::StringS16;
use crate::common::vector3_f32::Vector3F32;
use crate::common::vector3_i16_compressed::Vector3I16Compressed;
use binrw::{args, BinRead, BinResult, Endian, NamedArgs};
use napi_shadow::NapiShadow;
use std::io::{Read, Seek};

pub const COMPRESSED_NONE: u32 = 0;
pub const COMPRESSED_RELEVANT: u32 = 1;
pub const COMPRESSED_RELEVANT16: u32 = 2;
pub const COMPRESSED_RELEVANT_ROT16: u32 = 3;

#[derive(NapiShadow, BinRead, Debug)]
#[br(import{ node_count: u32 })]
pub struct Animation {
    pub extents: Vector3F32,
    pub name: StringS16,
    pub compression_type: u32,
    pub interpolation_time: u32,
    pub keyframe_count: u32,
    #[br(count = keyframe_count)]
    pub keyframes: Vec<Keyframe>,
    #[br(calc = if compression_type == COMPRESSED_NONE { 1 } else { node_count })]
    pub transform_count: u32,
    #[br(count = transform_count, args { inner: args! { keyframe_count, compression_type } })]
    pub transforms: Vec<NormalTransform>,
}

#[derive(NapiShadow, BinRead, Debug)]
pub struct Keyframe {
    pub time: u32,
    pub string: StringS16,
}

#[derive(BinRead, Debug)]
#[br(import{ keyframe_count: u32, compression_type: u32 })]
pub enum TransformCompressed {
    #[br(pre_assert(compression_type == COMPRESSED_NONE))]
    NoneCmp(#[br(args { keyframe_count })] TransformNoneCmp),

    #[br(pre_assert(compression_type == COMPRESSED_RELEVANT))]
    RelevantCmp(TransformRelevantCmp),

    #[br(pre_assert(compression_type == COMPRESSED_RELEVANT16))]
    Relevant16Cmp(TransformRelevant16Cmp),

    #[br(pre_assert(compression_type == COMPRESSED_RELEVANT_ROT16))]
    RelevantRot16Cmp(TransformRelevantRot16Cmp),
}

#[derive(NapiShadow, Debug)]
pub struct NormalTransform {
    pub compression_type: u32,
    pub is_vertex_animation: Option<i8>,
    pub position_count: u32,
    pub positions: Vec<Vector3F32>,
    pub rotation_count: u32,
    pub rotations: Vec<QuaternionF32>,
}

#[derive(NamedArgs, Clone)]
pub struct TransformArgs {
    pub keyframe_count: u32,
    pub compression_type: u32,
}

impl BinRead for NormalTransform {
    type Args<'a> = TransformArgs;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let TransformArgs {
            keyframe_count,
            compression_type,
        } = args;
        let transform = <TransformCompressed as BinRead>::read_options(
            reader,
            endian,
            args! { keyframe_count, compression_type },
        );

        Ok(match transform? {
            TransformCompressed::NoneCmp(t) => NormalTransform::from(t),
            TransformCompressed::RelevantCmp(t) => NormalTransform::from(t),
            TransformCompressed::Relevant16Cmp(t) => NormalTransform::from(t),
            TransformCompressed::RelevantRot16Cmp(t) => NormalTransform::from(t),
        })
    }
}

#[derive(BinRead, Debug)]
#[br(import{ keyframe_count: u32 })]
pub struct TransformNoneCmp {
    pub is_vertex_animation: i8,
    #[br(count = keyframe_count)]
    pub positions: Vec<Vector3F32>,
    #[br(count = keyframe_count)]
    pub rotations: Vec<QuaternionF32>,
}

impl From<TransformNoneCmp> for NormalTransform {
    fn from(value: TransformNoneCmp) -> Self {
        #[rustfmt::skip]
        let TransformNoneCmp { is_vertex_animation, positions, rotations } = value;

        Self {
            compression_type: COMPRESSED_NONE,
            is_vertex_animation: Some(is_vertex_animation),
            position_count: positions.len() as u32,
            positions,
            rotation_count: rotations.len() as u32,
            rotations,
        }
    }
}

#[derive(BinRead, Debug)]
pub struct TransformRelevantCmp {
    pub position_count: u32,
    #[br(count = position_count)]
    pub positions: Vec<Vector3F32>,
    pub rotation_count: u32,
    #[br(count = rotation_count)]
    pub rotations: Vec<QuaternionF32>,
}

impl From<TransformRelevantCmp> for NormalTransform {
    fn from(value: TransformRelevantCmp) -> Self {
        #[rustfmt::skip]
        let TransformRelevantCmp { position_count, positions, rotation_count, rotations } = value;

        Self {
            compression_type: COMPRESSED_RELEVANT,
            is_vertex_animation: None,
            position_count,
            positions,
            rotation_count,
            rotations,
        }
    }
}

#[derive(BinRead, Debug)]
pub struct TransformRelevant16Cmp {
    pub position_count: u32,
    #[br(count = position_count)]
    pub positions: Vec<Vector3I16Compressed>,
    pub rotation_count: u32,
    #[br(count = rotation_count)]
    pub rotations: Vec<QuaternionI16Compressed>,
}

impl From<TransformRelevant16Cmp> for NormalTransform {
    fn from(value: TransformRelevant16Cmp) -> Self {
        #[rustfmt::skip]
        let TransformRelevant16Cmp { position_count, positions, rotation_count, rotations } = value;

        Self {
            compression_type: COMPRESSED_RELEVANT16,
            is_vertex_animation: None,
            position_count,
            positions: positions.iter().map(Vector3F32::from).collect(),
            rotation_count,
            rotations: rotations.iter().map(QuaternionF32::from).collect(),
        }
    }
}

#[derive(BinRead, Debug)]
pub struct TransformRelevantRot16Cmp {
    pub position_count: u32,
    #[br(count = position_count)]
    pub positions: Vec<Vector3F32>,
    pub rotation_count: u32,
    #[br(count = rotation_count)]
    pub rotations: Vec<QuaternionI16Compressed>,
}

impl From<TransformRelevantRot16Cmp> for NormalTransform {
    fn from(value: TransformRelevantRot16Cmp) -> Self {
        #[rustfmt::skip]
        let TransformRelevantRot16Cmp { position_count, positions, rotation_count, rotations } = value;

        Self {
            compression_type: COMPRESSED_RELEVANT_ROT16,
            is_vertex_animation: None,
            position_count,
            positions,
            rotation_count,
            rotations: rotations.iter().map(QuaternionF32::from).collect(),
        }
    }
}
