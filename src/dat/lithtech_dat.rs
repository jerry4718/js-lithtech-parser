use crate::dat::blind_data::BlindData;
use crate::dat::dat_header::DatHeader;
use crate::dat::light_data::LightData;
use crate::dat::polygon_data::PolygonData;
use crate::dat::render_data::RenderData;
use crate::dat::world::World;
use crate::dat::world_data::WorldData;
use crate::dat::world_tree::WorldTree;
use binrw::{io::SeekFrom, BinRead};
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
#[br(little)]
#[napi_shadow(root)]
pub struct LithtechDat {
    pub header: DatHeader,

    pub world: World,

    pub world_tree: WorldTree,

    #[br(seek_before = SeekFrom::Start(header.world_data_pos.into()))]
    pub world_data: WorldData,

    #[br(seek_before = SeekFrom::Start(header.blind_data_pos.into()))]
    pub blind_data: BlindData,

    #[br(seek_before = SeekFrom::Start(header.light_data_pos.into()))]
    pub light_data: LightData,

    #[br(seek_before = SeekFrom::Start(header.physics_data_pos.into()))]
    pub physics_data: PolygonData,

    #[br(seek_before = SeekFrom::Start(header.particle_data_pos.into()))]
    pub particle_data: PolygonData,

    #[br(seek_before = SeekFrom::Start(header.render_data_pos.into()))]
    pub render_data: RenderData,
}
