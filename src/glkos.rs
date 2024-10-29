// Raw bindings to GLdc's glkos.h header
use crate::gl::*;

// Dreamcast specific compressed + twiddled formats.
// We use constants from the range= 0xEEE0 onwards
// to avoid trampling any real GL constants (this is in the middle of the
// any_vendor_future_use range defined in the GL enum.spec file.
pub const GL_UNSIGNED_SHORT_5_6_5_TWID_KOS: u32             = 0xEEE0;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV_TWID_KOS: u32       = 0xEEE2;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV_TWID_KOS: u32       = 0xEEE3;

pub const GL_COMPRESSED_RGB_565_VQ_KOS: u32                 = 0xEEE4;
pub const GL_COMPRESSED_ARGB_1555_VQ_KOS: u32               = 0xEEE6;
pub const GL_COMPRESSED_ARGB_4444_VQ_KOS: u32               = 0xEEE7;

pub const GL_COMPRESSED_RGB_565_VQ_TWID_KOS: u32            = 0xEEE8;
pub const GL_COMPRESSED_ARGB_1555_VQ_TWID_KOS: u32          = 0xEEEA;
pub const GL_COMPRESSED_ARGB_4444_VQ_TWID_KOS: u32          = 0xEEEB;

pub const GL_COMPRESSED_RGB_565_VQ_MIPMAP_KOS: u32          = 0xEEEC;
pub const GL_COMPRESSED_ARGB_1555_VQ_MIPMAP_KOS: u32        = 0xEEED;
pub const GL_COMPRESSED_ARGB_4444_VQ_MIPMAP_KOS: u32        = 0xEEEE;

pub const GL_COMPRESSED_RGB_565_VQ_MIPMAP_TWID_KOS: u32     = 0xEEEF;
pub const GL_COMPRESSED_ARGB_1555_VQ_MIPMAP_TWID_KOS: u32   = 0xEEF0;
pub const GL_COMPRESSED_ARGB_4444_VQ_MIPMAP_TWID_KOS: u32   = 0xEEF1;

pub const GL_NEARZ_CLIPPING_KOS: u32                        = 0xEEFA;

extern "C" {
    // Initialize the GL pipeline. GL will initialize the PVR.
    pub fn glKosInit();
}

#[repr(C)]
pub struct GLdcConfig {
    // If GL_TRUE, enables pvr autosorting, this *will* break glDepthFunc/glDepthTest

    autosort_enabled:               GLboolean,

    // If GL_TRUE, enables the PVR FSAA
    fsaa_enabled:                   GLboolean,

    // The internal format for paletted textures, must be GL_RGBA4 (default) or GL_RGBA8
    internal_palette_format:        GLenum,

    // Initial capacity of each of the OP, TR and PT lists in vertices
    initial_op_capacity:            GLuint,
    initial_tr_capacity:            GLuint,
    initial_pt_capacity:            GLuint,
    initial_immediate_capacity:     GLuint,

    // Default: True
    // Whether glTexImage should automatically twiddle textures
    // if the internal format is a generic format (e.g. GL_RGB).
    // this is the same as calling glEnable(GL_TEXTURE_TWIDDLE_KOS)
    // on boot
    texture_twiddle:                GLboolean,
}

#[repr(C)]
pub struct GLVertexKOS {
    padding0:   GLuint,
    x:          GLfloat,
    y:          GLfloat,
    z:          GLfloat,
    u:          GLfloat,
    v:          GLfloat,
    bgra:       [GLubyte; 4],
    padding1:   GLuint,
}

extern "C" {
    pub fn glVertexPackColor3fKOS(vertex: *mut GLVertexKOS,
                                  r: c_float, g: c_float, b: c_float);
    pub fn glVertexPackColor4fKOS(vertex: *mut GLVertexKOS,
                                  r: c_float, g: c_float, b: c_float, a: c_float);

    pub fn glKosInitConfig(config: *mut GLdcConfig);

    // Usage in C:
    // GLdcConfig config;
    // glKosInitConfig(&config);
    // config.autosort_enabled = GL_TRUE;
    // glKosInitEx(&config);
    pub fn glKosInitEx(config: *mut GLdcConfig);
    pub fn glKosSwapBuffers();
    pub fn glKosShutdown();
}

// CUSTOM EXTENSION multiple_shared_palette_KOS
//
// This extension allows using up to 4 different shared palettes
// with ColorTableEXT. The following constants are provided
// to use as targets for ColorTableExt:
//
// - SHARED_TEXTURE_PALETTE_0_KOS
// - SHARED_TEXTURE_PALETTE_1_KOS
// - SHARED_TEXTURE_PALETTE_2_KOS
// - SHARED_TEXTURE_PALETTE_3_KOS
//
// In this use case SHARED_TEXTURE_PALETTE_0_KOS is interchangable with SHARED_TEXTURE_PALETTE_EXT
// (both refer to the first shared palette).
//
// To select which palette a texture uses, a new pname is accepted by TexParameteri: SHARED_TEXTURE_BANK_KOS
// by default textures use shared palette 0.

pub const GL_SHARED_TEXTURE_PALETTE_0_KOS: u32              = 0xEEFC;
pub const GL_SHARED_TEXTURE_PALETTE_1_KOS: u32              = 0xEEFD;
pub const GL_SHARED_TEXTURE_PALETTE_2_KOS: u32              = 0xEEFE;
pub const GL_SHARED_TEXTURE_PALETTE_3_KOS: u32              = 0xEEFF;
pub const GL_SHARED_TEXTURE_PALETTE_4_KOS: u32              = 0xEF00;
pub const GL_SHARED_TEXTURE_PALETTE_5_KOS: u32              = 0xEF01;
pub const GL_SHARED_TEXTURE_PALETTE_6_KOS: u32              = 0xEF02;
pub const GL_SHARED_TEXTURE_PALETTE_7_KOS: u32              = 0xEF03;
pub const GL_SHARED_TEXTURE_PALETTE_8_KOS: u32              = 0xEF04;
pub const GL_SHARED_TEXTURE_PALETTE_9_KOS: u32              = 0xEF05;

pub const GL_SHARED_TEXTURE_PALETTE_10_KOS: u32             = 0xEF06;
pub const GL_SHARED_TEXTURE_PALETTE_11_KOS: u32             = 0xEF07;
pub const GL_SHARED_TEXTURE_PALETTE_12_KOS: u32             = 0xEF08;
pub const GL_SHARED_TEXTURE_PALETTE_13_KOS: u32             = 0xEF09;
pub const GL_SHARED_TEXTURE_PALETTE_14_KOS: u32             = 0xEF0A;
pub const GL_SHARED_TEXTURE_PALETTE_15_KOS: u32             = 0xEF0B;
pub const GL_SHARED_TEXTURE_PALETTE_16_KOS: u32             = 0xEF0C;
pub const GL_SHARED_TEXTURE_PALETTE_17_KOS: u32             = 0xEF0D;
pub const GL_SHARED_TEXTURE_PALETTE_18_KOS: u32             = 0xEF0E;
pub const GL_SHARED_TEXTURE_PALETTE_19_KOS: u32             = 0xEF0F;

pub const GL_SHARED_TEXTURE_PALETTE_20_KOS: u32             = 0xEF10;
pub const GL_SHARED_TEXTURE_PALETTE_21_KOS: u32             = 0xEF11;
pub const GL_SHARED_TEXTURE_PALETTE_22_KOS: u32             = 0xEF12;
pub const GL_SHARED_TEXTURE_PALETTE_23_KOS: u32             = 0xEF13;
pub const GL_SHARED_TEXTURE_PALETTE_24_KOS: u32             = 0xEF14;
pub const GL_SHARED_TEXTURE_PALETTE_25_KOS: u32             = 0xEF15;
pub const GL_SHARED_TEXTURE_PALETTE_26_KOS: u32             = 0xEF16;
pub const GL_SHARED_TEXTURE_PALETTE_27_KOS: u32             = 0xEF17;
pub const GL_SHARED_TEXTURE_PALETTE_28_KOS: u32             = 0xEF18;
pub const GL_SHARED_TEXTURE_PALETTE_29_KOS: u32             = 0xEF19;

pub const GL_SHARED_TEXTURE_PALETTE_30_KOS: u32             = 0xEF1A;
pub const GL_SHARED_TEXTURE_PALETTE_31_KOS: u32             = 0xEF1B;
pub const GL_SHARED_TEXTURE_PALETTE_32_KOS: u32             = 0xEF1C;
pub const GL_SHARED_TEXTURE_PALETTE_33_KOS: u32             = 0xEF1D;
pub const GL_SHARED_TEXTURE_PALETTE_34_KOS: u32             = 0xEF1E;
pub const GL_SHARED_TEXTURE_PALETTE_35_KOS: u32             = 0xEF1F;
pub const GL_SHARED_TEXTURE_PALETTE_36_KOS: u32             = 0xEF20;
pub const GL_SHARED_TEXTURE_PALETTE_37_KOS: u32             = 0xEF21;
pub const GL_SHARED_TEXTURE_PALETTE_38_KOS: u32             = 0xEF22;
pub const GL_SHARED_TEXTURE_PALETTE_39_KOS: u32             = 0xEF23;

pub const GL_SHARED_TEXTURE_PALETTE_40_KOS: u32             = 0xEF24;
pub const GL_SHARED_TEXTURE_PALETTE_41_KOS: u32             = 0xEF25;
pub const GL_SHARED_TEXTURE_PALETTE_42_KOS: u32             = 0xEF26;
pub const GL_SHARED_TEXTURE_PALETTE_43_KOS: u32             = 0xEF27;
pub const GL_SHARED_TEXTURE_PALETTE_44_KOS: u32             = 0xEF28;
pub const GL_SHARED_TEXTURE_PALETTE_45_KOS: u32             = 0xEF29;
pub const GL_SHARED_TEXTURE_PALETTE_46_KOS: u32             = 0xEF2A;
pub const GL_SHARED_TEXTURE_PALETTE_47_KOS: u32             = 0xEF2B;
pub const GL_SHARED_TEXTURE_PALETTE_48_KOS: u32             = 0xEF2C;
pub const GL_SHARED_TEXTURE_PALETTE_49_KOS: u32             = 0xEF2D;

pub const GL_SHARED_TEXTURE_PALETTE_50_KOS: u32             = 0xEF2E;
pub const GL_SHARED_TEXTURE_PALETTE_51_KOS: u32             = 0xEF2F;
pub const GL_SHARED_TEXTURE_PALETTE_52_KOS: u32             = 0xEF30;
pub const GL_SHARED_TEXTURE_PALETTE_53_KOS: u32             = 0xEF31;
pub const GL_SHARED_TEXTURE_PALETTE_54_KOS: u32             = 0xEF32;
pub const GL_SHARED_TEXTURE_PALETTE_55_KOS: u32             = 0xEF33;
pub const GL_SHARED_TEXTURE_PALETTE_56_KOS: u32             = 0xEF34;
pub const GL_SHARED_TEXTURE_PALETTE_57_KOS: u32             = 0xEF35;
pub const GL_SHARED_TEXTURE_PALETTE_58_KOS: u32             = 0xEF36;
pub const GL_SHARED_TEXTURE_PALETTE_59_KOS: u32             = 0xEF37;

pub const GL_SHARED_TEXTURE_PALETTE_60_KOS: u32             = 0xEF38;
pub const GL_SHARED_TEXTURE_PALETTE_61_KOS: u32             = 0xEF39;
pub const GL_SHARED_TEXTURE_PALETTE_62_KOS: u32             = 0xEF3A;
pub const GL_SHARED_TEXTURE_PALETTE_63_KOS: u32             = 0xEF3B;

// Pass to glTexParameteri to set the shared bank
pub const GL_SHARED_TEXTURE_BANK_KOS: u32                   = 0xEF3C;

extern "C" {
    // Memory allocation extension (GL_KOS_texture_memory_management)
    pub fn glDefragmentTextureMemory_KOS() -> GLvoid;
}

// glGet extensions
pub const GL_FREE_TEXTURE_MEMORY_KOS: u32                   = 0xEF3D;
pub const GL_USED_TEXTURE_MEMORY_KOS: u32                   = 0xEF3E;
pub const GL_FREE_CONTIGUOUS_TEXTURE_MEMORY_KOS: u32        = 0xEF3F;

// for palette internal format (glfcConfig)
pub const GL_RGB565_KOS: u32                                = 0xEF40;
pub const GL_ARGB4444_KOS: u32                              = 0xEF41;
pub const GL_ARGB1555_KOS: u32                              = 0xEF42;
pub const GL_RGB565_TWID_KOS: u32                           = 0xEF43;
pub const GL_ARGB4444_TWID_KOS: u32                         = 0xEF44;
pub const GL_ARGB1555_TWID_KOS: u32                         = 0xEF45;
pub const GL_COLOR_INDEX8_TWID_KOS: u32                     = 0xEF46;
pub const GL_COLOR_INDEX4_TWID_KOS: u32                     = 0xEF47;
pub const GL_RGB_TWID_KOS: u32                              = 0xEF48;
pub const GL_RGBA_TWID_KOS: u32                             = 0xEF49;

// glGet extensions
pub const GL_TEXTURE_INTERNAL_FORMAT_KOS: u32               = 0xEF50;

// If enabled, will twiddle texture uploads where possible
pub const GL_TEXTURE_TWIDDLE_KOS: u32                       = 0xEF51;
