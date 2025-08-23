use crate::common::string_s16::StringS16;
use crate::ltb::anim_binding::AnimBinding;
use crate::ltb::animation::Animation;
use crate::ltb::bone_node::BoneNode;
use crate::ltb::ltb_header::LtbHeader;
use crate::ltb::model_obb::ModelOBB;
use crate::ltb::piece::Piece;
use crate::ltb::socket::Socket;
use crate::ltb::weight_set::WeightSet;
use binrw::{args, BinRead};
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[br(little)]
#[napi_shadow(root)]
pub struct LithtechLtb {
    pub header: LtbHeader,

    pub model_obb_count: u32,
    #[br(count = model_obb_count, args { inner: args! { obb_version: header.obb_version } })]
    pub model_obb_list: Vec<ModelOBB>,

    pub piece_count: u32,
    #[br(count = piece_count)]
    pub pieces: Vec<Piece>,

    pub bone_tree: BoneNode,

    pub weight_set_count: u32,
    #[br(count = weight_set_count)]
    pub weight_sets: Vec<WeightSet>,

    pub child_model_count: u32,
    #[br(count = child_model_count - 1)]
    pub child_models: Vec<StringS16>,

    pub animation_count: u32,
    #[br(count = animation_count, args { inner: args! { node_count: header.node_count } })]
    pub animations: Vec<Animation>,

    pub socket_count: u32,
    #[br(count = socket_count)]
    pub sockets: Vec<Socket>,

    pub anim_binding_count: u32,
    #[br(count = anim_binding_count)]
    pub anim_bindings: Vec<AnimBinding>,
}
