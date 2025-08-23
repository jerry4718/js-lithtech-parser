use crate::dat::lithtech_dat::LithtechDat;
use crate::dtx::lithtech_dtx::LithtechDtx;
use crate::ltb::lithtech_ltb::LithtechLtb;
use binrw::io::Cursor;
use binrw::BinRead;
use napi_derive::napi;
use napi_shadow::{NapiShadow, NapiShadowRoot};
use std::rc::Rc;

pub mod common;
pub mod dat;
pub mod dtx;
pub mod ltb;
mod util;

#[napi]
pub fn parse_dat(buf: &[u8]) -> <LithtechDat as NapiShadow>::ShadowStruct {
    let mut data = Cursor::new(buf);
    let dat = LithtechDat::read(&mut data).unwrap();

    let root_rc = Rc::new(dat);

    <LithtechDat as NapiShadow>::napi_shadow(
        root_rc.clone().as_ref(),
        root_rc as Rc<dyn NapiShadowRoot>,
    )
}

#[napi]
pub fn parse_ltb(buf: &[u8]) -> <LithtechLtb as NapiShadow>::ShadowStruct {
    let mut data = Cursor::new(buf);
    let ltb = LithtechLtb::read(&mut data).unwrap();

    let root_rc = Rc::new(ltb);

    <LithtechLtb as NapiShadow>::napi_shadow(
        root_rc.clone().as_ref(),
        root_rc as Rc<dyn NapiShadowRoot>,
    )
}

#[napi]
pub fn parse_dtx(buf: &[u8]) -> <LithtechDtx as NapiShadow>::ShadowStruct {
    let mut data = Cursor::new(buf);
    let dtx = LithtechDtx::read(&mut data).unwrap();

    let root_rc = Rc::new(dtx);

    <LithtechDtx as NapiShadow>::napi_shadow(
        root_rc.clone().as_ref(),
        root_rc as Rc<dyn NapiShadowRoot>,
    )
}

#[link(name = "zig-dxt")] // 对应 zig-dxt.a
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

/// 提供一个 Rust 包装函数
pub fn plus_from_zig(a: i32, b: i32) -> i32 {
    unsafe { add(a, b) }
}

#[napi]
pub fn plus_100_from_zig(input: i32) -> i32 {
    plus_from_zig(input, 100)
}

#[napi]
pub fn plus_100(input: i32) -> i32 {
    input + 100
}
