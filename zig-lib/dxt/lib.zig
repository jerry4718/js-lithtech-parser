const std = @import("std");

/// 用 C ABI 导出函数
pub export fn add(a: i32, b: i32) i32 {
    return a + b;
}