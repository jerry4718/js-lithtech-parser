#![allow(unused_variables, unused_imports, dead_code, non_camel_case_types)]
type BlockFn = fn() -> ();

#[non_exhaustive]
enum DxtType {
    Dxt1 = 0,
    Dxt3 = 1,
    Dxt5 = 2,
}

impl From<u8> for DxtType {
    fn from(value: u8) -> Self {
        match value {
            0 => DxtType::Dxt1,
            1 => DxtType::Dxt3,
            2 => DxtType::Dxt5,
            _ => unreachable!(),
        }
    }
}

impl Into<u8> for DxtType {
    fn into(self) -> u8 {
        match self {
            DxtType::Dxt1 => 0,
            DxtType::Dxt3 => 1,
            DxtType::Dxt5 => 2,
        }
    }
}

fn exp_565_r(packed: u16) -> u8 {
    ((packed >> 8) as u8 & 0xf8u8) | ((packed >> 13) as u8 & 0x7u8)
}
fn exp_565_g(packed: u16) -> u8 {
    ((packed >> 3) as u8 & 0xfcu8) | ((packed >> 9) as u8 & 0x3u8)
}
fn exp_565_b(packed: u16) -> u8 {
    ((packed << 3) as u8 & 0xf8u8) | ((packed >> 2) as u8 & 0x7u8)
}
fn exp4to8(col: u8) -> u8 {
    col | (col << 4)
}

type TupleRgb = (u8, u8, u8);

/**
 * @param dxt_type 0|1|2
 * @param code 0|1|2|3
 * @param color0 packed color0
 * @param color1 packed color1
 * @param get
 */
fn get_unit<F>(dxt_type: &DxtType, code: u8, color0: u16, color1: u16, get: F) -> u8
where
    F: Fn(u16) -> u8,
{
    let v0 = get(color0);
    if code == 0 {
        return v0;
    }

    let v1 = get(color1);
    if code == 1 {
        return v1;
    }

    if (matches!(dxt_type, DxtType::Dxt1 | DxtType::Dxt3)) || (color0 > color1) {
        return ((4 - code) * v0 + (code - 1) * v1) / 3;
    }

    if code == 2 {
        return (v0 + v1) / 2;
    }
    if code == 3 {
        return 0;
    }
    unimplemented!("unimplemented code {}", code);
}

fn unpack_block(dxt_type: &DxtType, code: u8, color0: u16, color1: u16) -> TupleRgb {
    (
        /* r */ get_unit(dxt_type, code, color0, color1, exp_565_r),
        /* g */ get_unit(dxt_type, code, color0, color1, exp_565_g),
        /* b */ get_unit(dxt_type, code, color0, color1, exp_565_b),
    )
}

struct EachPos {
    out_width: usize,
    block_left: usize,
    block_top: usize,
    each_i: usize,
    each_j: usize,
}

impl EachPos {
    pub fn new(out_width: usize, block_left: usize, block_top: usize) -> Self {
        Self {
            out_width,
            block_left,
            block_top,
            each_i: 0,
            each_j: 0,
        }
    }
}

mod dxt_mod {
    use binrw::BinRead;
    use std::io::Cursor;
    use std::ops::Index;

    #[derive(binrw::BinRead, Debug)]
    pub struct Dxt1Chunk {
        color_data: (u16, u16),
        color_masks: [u8; 4],
    }
    #[derive(binrw::BinRead, Debug)]
    pub struct Dxt3Chunk {
        alpha_masks: [u8; 8],
        color_data: (u16, u16),
        color_masks: [u8; 4],
    }
    #[derive(binrw::BinRead, Debug)]
    pub struct Dxt5Chunk {
        alpha_data: (u8, u8),
        alpha_masks: [u8; 6],
        color_data: (u16, u16),
        color_masks: [u8; 4],
    }

    #[test]
    fn test_0001() {
        let mut data = Cursor::new(&[0xff; 16]);
        let x = Dxt5Chunk::read_be(&mut data).unwrap();

        dbg!(x);
    }
}
