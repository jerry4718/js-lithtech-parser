use crate::dtx::dtx_header::DtxHeader;
use crate::dtx::image::{
    is_compressed, is_palette_32bit, is_palette_8bit, is_texture_32bit, ImageMeta,
};
use binrw::helpers::until_eof;
use binrw::BinRead;
use napi_shadow::NapiShadow;

// Versioning
pub const DTX_VERSION_LT1: i32 = -2;
pub const DTX_VERSION_LT15: i32 = -3;
pub const DTX_VERSION_LT2: i32 = -5;

pub const SUPPORT_VERSIONS: [i32; 3] = [DTX_VERSION_LT1, DTX_VERSION_LT15, DTX_VERSION_LT2];

pub const DTX_COMMAND_STRING_LENGTH: usize = 128;
pub const COMMAND_STRING_VERSIONS: [i32; 2] = [DTX_VERSION_LT15, DTX_VERSION_LT2];

// Not used in version DTX_VERSION_LT1?
pub const BPP_8P: u8 = 0;
pub const BPP_8: u8 = 1;
pub const BPP_16: u8 = 2;
pub const BPP_32: u8 = 3;
pub const BPP_S3TC_DXT1: u8 = 4;
pub const BPP_S3TC_DXT3: u8 = 5;
pub const BPP_S3TC_DXT5: u8 = 6;
pub const BPP_32P: u8 = 7;
pub const BPP_24: u8 = 8;

// the bytes per pixel for each format type, (0 if not applicable)
pub const G_PIXEL_BYTES: [usize; 9] = [1, 1, 2, 4, 0, 0, 0, 1, 3];

pub const DTX_FULLBRITE: u32 = 1 << 0; // This DTX has fullbrite colors.
pub const DTX_PREFER16BIT: u32 = 1 << 1; // Use 16-bit, even if in 32-bit mode.
pub const DTX_MIPSALLOCED: u32 = 1 << 2; // Used to make some of the tools stuff easier..
                                         // This means each TextureMipData has its texture data allocated.
pub const DTX_SECTIONSFIXED: u32 = 1 << 3; // The sections count was screwed up originally.  This flag is set
                                           // in all the textures from now on when the count is fixed.
pub const DTX_NOSYSCACHE: u32 = 1 << 6; // Not saved: used internally.. tells it to not put the texture
                                        // in the texture cache list.
pub const DTX_PREFER4444: u32 = 1 << 7; // If in 16-bit mode, use a 4444 texture for this.

pub const DTX_PREFER5551: u32 = 1 << 8; // Use 5551 if 16-bit.

pub const DTX_32BITSYSCOPY: u32 = 1 << 9; // If there is a sys copy - don't convert it to device specific format (keep it 32 bit).

pub const DTX_CUBEMAP: u32 = 1 << 10; // Cube environment map.  +x is stored in the normal data area,
                                      // -x,+y,-y,+z,-z are stored in their own sections

pub const DTX_BUMPMAP: u32 = 1 << 11; // Bump mapped texture, this has 8 bit U and V components for the bump normal

pub const DTX_LUMBUMPMAP: u32 = 1 << 12; // Bump mapped texture with luminance, this has 8 bits for luminance, U and V

#[derive(NapiShadow, BinRead, Debug)]
#[napi_shadow(root)]
#[br(little)]
pub struct LithtechDtx {
    pub header: DtxHeader,

    #[br(calc = header.flags & DTX_FULLBRITE == DTX_FULLBRITE)]
    pub dtx_fullbrite: bool,
    #[br(calc = header.flags & DTX_PREFER16BIT == DTX_PREFER16BIT)]
    pub dtx_prefer16bit: bool,
    #[br(calc = header.flags & DTX_MIPSALLOCED == DTX_MIPSALLOCED)]
    pub dtx_mipsalloced: bool,
    #[br(calc = header.flags & DTX_SECTIONSFIXED == DTX_SECTIONSFIXED)]
    pub dtx_sectionsfixed: bool,
    #[br(calc = header.flags & DTX_NOSYSCACHE == DTX_NOSYSCACHE)]
    pub dtx_nosyscache: bool,
    #[br(calc = header.flags & DTX_PREFER4444 == DTX_PREFER4444)]
    pub dtx_prefer4444: bool,
    #[br(calc = header.flags & DTX_PREFER5551 == DTX_PREFER5551)]
    pub dtx_prefer5551: bool,
    #[br(calc = header.flags & DTX_32BITSYSCOPY == DTX_32BITSYSCOPY)]
    pub dtx_32bitsyscopy: bool,
    #[br(calc = header.flags & DTX_CUBEMAP == DTX_CUBEMAP)]
    pub dtx_cubemap: bool,
    #[br(calc = header.flags & DTX_BUMPMAP == DTX_BUMPMAP)]
    pub dtx_bumpmap: bool,
    #[br(calc = header.flags & DTX_LUMBUMPMAP == DTX_LUMBUMPMAP)]
    pub dtx_lumbumpmap: bool,

    #[br(calc = header.bpp_identifier == BPP_8P)]
    pub is_bpp_8p: bool,
    #[br(calc = header.bpp_identifier == BPP_8)]
    pub is_bpp_8: bool,
    #[br(calc = header.bpp_identifier == BPP_16)]
    pub is_bpp_16: bool,
    #[br(calc = header.bpp_identifier == BPP_32)]
    pub is_bpp_32: bool,
    #[br(calc = header.bpp_identifier == BPP_S3TC_DXT1)]
    pub is_bpp_s3tc_dxt1: bool,
    #[br(calc = header.bpp_identifier == BPP_S3TC_DXT3)]
    pub is_bpp_s3tc_dxt3: bool,
    #[br(calc = header.bpp_identifier == BPP_S3TC_DXT5)]
    pub is_bpp_s3tc_dxt5: bool,
    #[br(calc = header.bpp_identifier == BPP_32P)]
    pub is_bpp_32p: bool,
    #[br(calc = header.bpp_identifier == BPP_24)]
    pub is_bpp_24: bool,

    #[br(calc = is_palette_8bit(&header))]
    pub is_palette_8bit: bool,
    #[br(calc = is_compressed(&header))]
    pub is_compressed: bool,
    #[br(calc = is_texture_32bit(&header))]
    pub is_texture_32bit: bool,
    #[br(calc = is_palette_32bit(&header))]
    pub is_palette_32bit: bool,

    #[br(args { header: &header })]
    #[napi_shadow(skip)]
    pub meta: ImageMeta,

    #[br(parse_with = until_eof)]
    pub surplus: Vec<u8>,
}
