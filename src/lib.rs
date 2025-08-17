#![allow(unused_variables, unused_imports, dead_code, non_camel_case_types)]

use napi_derive::napi;

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
