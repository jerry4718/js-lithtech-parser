use crate::dat::polygon::Polygon;
use binrw::BinRead;
use napi_shadow::NapiShadow;

#[derive(NapiShadow, BinRead, Debug)]
pub struct PolygonData {
    pub polygon_count: u32,

    #[br(count = polygon_count)]
    pub polygons: Vec<Polygon>,
}
