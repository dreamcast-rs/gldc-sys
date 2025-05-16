// Raw bindings to GLdc's glext.h header
use crate::gl::*;

pub const GL_TEXTURE0_ARB: u32                                  = 0x84C0;
pub const GL_TEXTURE1_ARB: u32                                  = 0x84C1;
pub const GL_TEXTURE2_ARB: u32                                  = 0x84C2;
pub const GL_TEXTURE3_ARB: u32                                  = 0x84C3;
pub const GL_TEXTURE4_ARB: u32                                  = 0x84C4;
pub const GL_TEXTURE5_ARB: u32                                  = 0x84C5;
pub const GL_TEXTURE6_ARB: u32                                  = 0x84C6;
pub const GL_TEXTURE7_ARB: u32                                  = 0x84C7;
pub const GL_TEXTURE8_ARB: u32                                  = 0x84C8;
pub const GL_TEXTURE9_ARB: u32                                  = 0x84C9;
pub const GL_TEXTURE10_ARB: u32                                 = 0x84CA;
pub const GL_TEXTURE11_ARB: u32                                 = 0x84CB;
pub const GL_TEXTURE12_ARB: u32                                 = 0x84CC;
pub const GL_TEXTURE13_ARB: u32                                 = 0x84CD;
pub const GL_TEXTURE14_ARB: u32                                 = 0x84CE;
pub const GL_TEXTURE15_ARB: u32                                 = 0x84CF;
pub const GL_TEXTURE16_ARB: u32                                 = 0x84D0;
pub const GL_TEXTURE17_ARB: u32                                 = 0x84D1;
pub const GL_TEXTURE18_ARB: u32                                 = 0x84D2;
pub const GL_TEXTURE19_ARB: u32                                 = 0x84D3;
pub const GL_TEXTURE20_ARB: u32                                 = 0x84D4;
pub const GL_TEXTURE21_ARB: u32                                 = 0x84D5;
pub const GL_TEXTURE22_ARB: u32                                 = 0x84D6;
pub const GL_TEXTURE23_ARB: u32                                 = 0x84D7;
pub const GL_TEXTURE24_ARB: u32                                 = 0x84D8;
pub const GL_TEXTURE25_ARB: u32                                 = 0x84D9;
pub const GL_TEXTURE26_ARB: u32                                 = 0x84DA;
pub const GL_TEXTURE27_ARB: u32                                 = 0x84DB;
pub const GL_TEXTURE28_ARB: u32                                 = 0x84DC;
pub const GL_TEXTURE29_ARB: u32                                 = 0x84DD;
pub const GL_TEXTURE30_ARB: u32                                 = 0x84DE;
pub const GL_TEXTURE31_ARB: u32                                 = 0x84DF;
pub const GL_ACTIVE_TEXTURE_ARB: u32                            = 0x84E0;
pub const GL_CLIENT_ACTIVE_TEXTURE_ARB: u32                     = 0x84E1;
pub const GL_MAX_TEXTURE_UNITS_ARB: u32                         = 0x84E2;

pub const GL_CLAMP_TO_EDGE: u32                                 = 0x812F;

pub const GL_TRANSPOSE_MODELVIEW_MATRIX_ARB: u32                = 0x84E3;
pub const GL_TRANSPOSE_PROJECTION_MATRIX_ARB: u32               = 0x84E4;
pub const GL_TRANSPOSE_TEXTURE_MATRIX_ARB: u32                  = 0x84E5;
pub const GL_TRANSPOSE_COLOR_MATRIX_ARB: u32                    = 0x84E6;

pub const GL_NORMAL_MAP_ARB: u32                                = 0x8511;
pub const GL_REFLECTION_MAP_ARB: u32                            = 0x8512;
pub const GL_TEXTURE_CUBE_MAP_ARB: u32                          = 0x8513;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARB: u32                  = 0x8514;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X_ARB: u32               = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X_ARB: u32               = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y_ARB: u32               = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y_ARB: u32               = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z_ARB: u32               = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z_ARB: u32               = 0x851A;
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARB: u32                    = 0x851B;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE_ARB: u32                 = 0x851C;

pub const GL_COMPRESSED_ALPHA_ARB: u32                          = 0x84E9;
pub const GL_COMPRESSED_LUMINANCE_ARB: u32                      = 0x84EA;
pub const GL_COMPRESSED_LUMINANCE_ALPHA_ARB: u32                = 0x84EB;
pub const GL_COMPRESSED_INTENSITY_ARB: u32                      = 0x84EC;
pub const GL_COMPRESSED_RGB_ARB: u32                            = 0x84ED;
pub const GL_COMPRESSED_RGBA_ARB: u32                           = 0x84EE;
pub const GL_TEXTURE_COMPRESSION_HINT_ARB: u32                  = 0x84EF;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE_ARB: u32             = 0x86A0;
pub const GL_TEXTURE_COMPRESSED_ARB: u32                        = 0x86A1;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS_ARB: u32            = 0x86A2;
pub const GL_COMPRESSED_TEXTURE_FORMATS_ARB: u32                = 0x86A3;

pub const GL_COLOR_ATTACHMENT0_EXT: u32                         = 0x8CE0;
pub const GL_COLOR_ATTACHMENT1_EXT: u32                         = 0x8CE1;
pub const GL_COLOR_ATTACHMENT2_EXT: u32                         = 0x8CE2;
pub const GL_COLOR_ATTACHMENT3_EXT: u32                         = 0x8CE3;
pub const GL_COLOR_ATTACHMENT4_EXT: u32                         = 0x8CE4;
pub const GL_COLOR_ATTACHMENT5_EXT: u32                         = 0x8CE5;
pub const GL_COLOR_ATTACHMENT6_EXT: u32                         = 0x8CE6;
pub const GL_COLOR_ATTACHMENT7_EXT: u32                         = 0x8CE7;
pub const GL_COLOR_ATTACHMENT8_EXT: u32                         = 0x8CE8;
pub const GL_COLOR_ATTACHMENT9_EXT: u32                         = 0x8CE9;
pub const GL_COLOR_ATTACHMENT10_EXT: u32                        = 0x8CEA;
pub const GL_COLOR_ATTACHMENT11_EXT: u32                        = 0x8CEB;
pub const GL_COLOR_ATTACHMENT12_EXT: u32                        = 0x8CEC;
pub const GL_COLOR_ATTACHMENT13_EXT: u32                        = 0x8CED;
pub const GL_COLOR_ATTACHMENT14_EXT: u32                        = 0x8CEE;
pub const GL_COLOR_ATTACHMENT15_EXT: u32                        = 0x8CEF;
pub const GL_DEPTH_ATTACHMENT_EXT: u32                          = 0x8D00;
pub const GL_STENCIL_ATTACHMENT_EXT: u32                        = 0x8D20;
pub const GL_FRAMEBUFFER_EXT: u32                               = 0x8D40;
pub const GL_RENDERBUFFER_EXT: u32                              = 0x8D41;
pub const GL_RENDERBUFFER_WIDTH_EXT: u32                        = 0x8D42;
pub const GL_RENDERBUFFER_HEIGHT_EXT: u32                       = 0x8D43;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT_EXT: u32              = 0x8D44;

pub const GL_FRAMEBUFFER_COMPLETE_EXT: u32                      = 0x8CD5;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT_EXT : u32        = 0x8CD6;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_EXT: u32 = 0x8CD7;
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER_EXT: u32        = 0x8CDB;
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER_EXT: u32        = 0x8CDC;
pub const GL_FRAMEBUFFER_UNSUPPORTED_EXT: u32                   = 0x8CDD;
pub const GL_INVALID_FRAMEBUFFER_OPERATION_EXT: u32             = 0x0506;

extern "C" {
    // Multitexture extensions
    pub fn glActiveTextureARB(texture: GLenum);
    pub fn glClientActiveTextureARB(texture: GLenum);
    pub fn glMultiTexCoord2fARB(target: GLenum, s: GLfloat, t: GLfloat);
    pub fn glGenFramebuffersEXT(n: GLsizei, framebuffers: *mut GLuint);
    pub fn glDeleteFramebuffersEXT(n: GLsizei, framebuffers: *const GLuint);
    pub fn glBindFramebufferEXT(target: GLenum, framebuffer: GLuint);
    pub fn glFramebufferTexture2DEXT(target: GLenum, attachment: GLenum,
                                     textarget: GLenum, texture: GLuint, level: GLint);
    pub fn glGenerateMipmap(target: GLenum);
    pub fn glCheckFramebufferStatusEXT(target: GLenum) -> GLenum;
    pub fn glIsFramebufferEXT(framebuffer: GLuint) -> GLboolean;
}

// ext_paletted_texture
pub const GL_COLOR_INDEX1_EXT: u32                              = 0x80E2;
pub const GL_COLOR_INDEX2_EXT: u32                              = 0x80E3;
pub const GL_COLOR_INDEX4_EXT: u32                              = 0x80E4;
pub const GL_COLOR_INDEX8_EXT: u32                              = 0x80E5;
pub const GL_COLOR_INDEX12_EXT: u32                             = 0x80E6;
pub const GL_COLOR_INDEX16_EXT: u32                             = 0x80E7;

pub const GL_COLOR_TABLE_FORMAT_EXT: u32                        = 0x80D8;
pub const GL_COLOR_TABLE_WIDTH_EXT: u32                         = 0x80D9;
pub const GL_COLOR_TABLE_RED_SIZE_EXT: u32                      = 0x80DA;
pub const GL_COLOR_TABLE_GREEN_SIZE_EXT: u32                    = 0x80DB;
pub const GL_COLOR_TABLE_BLUE_SIZE_EXT: u32                     = 0x80DC;
pub const GL_COLOR_TABLE_ALPHA_SIZE_EXT: u32                    = 0x80DD;
pub const GL_COLOR_TABLE_LUMINANCE_SIZE_EXT: u32                = 0x80DE;
pub const GL_COLOR_TABLE_INTENSITY_SIZE_EXT: u32                = 0x80DF;

pub const GL_TEXTURE_INDEX_SIZE_EXT: u32                        = 0x80ED;

pub const GL_SHARED_TEXTURE_PALETTE_EXT: u32                    = 0x81FB;

extern "C" {
    pub fn glColorTableEXT(target: GLenum, internalFormat: GLenum, width: GLsizei,
                           format: GLenum, r#type: GLenum, data: *const GLvoid);
    pub fn glColorSubTableEXT(target: GLenum, start: GLsizei, count: GLsizei,
                              format: GLenum, r#type: GLenum, data: *const GLvoid);
    pub fn glGetColorTableEXT(target: GLenum, format: GLenum, r#type: GLenum,
                              data: *mut GLvoid);
    pub fn glGetColorTableParameterivEXT(target: GLenum, pname: GLenum,
                                         params: *mut GLint);
    pub fn glGetColorTableParameterfvEXT(target: GLenum, pname: GLenum,
                                         params: *mut GLfloat);
}

// ext OES_compressed_paletted_texture

// PixelInternalFormat
// Ozzy: used MesaGL definitions please adjust if it causes probs.
pub const GL_PALETTE4_RGB8_OES: u32                             = 0x8B90;
pub const GL_PALETTE4_RGBA8_OES: u32                            = 0x8B91;
pub const GL_PALETTE4_R5_G6_B5_OES: u32                         = 0x8B92;
pub const GL_PALETTE4_RGBA4_OES: u32                            = 0x8B93;
pub const GL_PALETTE4_RGB5_A1_OES: u32                          = 0x8B94;
pub const GL_PALETTE8_RGB8_OES: u32                             = 0x8B95;
pub const GL_PALETTE8_RGBA8_OES: u32                            = 0x8B96;
pub const GL_PALETTE8_R5_G6_B5_OES: u32                         = 0x8B97;
pub const GL_PALETTE8_RGBA4_OES: u32                            = 0x8B98;
pub const GL_PALETTE8_RGB5_A1_OES: u32                          = 0x8B99;

extern "C" {
    // Loads VQ compressed texture from SH4 RAM into PVR VRAM
    // internalformat must be one of the following constants:
    //  GL_UNSIGNED_SHORT_5_6_5_VQ
    //  GL_UNSIGNED_SHORT_5_6_5_VQ_TWID
    //  GL_UNSIGNED_SHORT_4_4_4_4_VQ
    //  GL_UNSIGNED_SHORT_4_4_4_4_VQ_TWID
    //  GL_UNSIGNED_SHORT_1_5_5_5_VQ
    //  GL_UNSIGNED_SHORT_1_5_5_5_VQ_TWID
    pub fn glCompressedTexImage2DARB(target: GLenum, level: GLint, internalformat: GLenum,
                                     width: GLsizei, height: GLsizei, border: GLint,
                                     imageSize: GLsizei, data: *const GLvoid);

    pub fn glCompressedTexSubImage2DARB(target: GLenum, level: GLint,
                                        xoffset: GLint, yoffset: GLint,
                                        width: GLsizei, height: GLsizei,
                                        format: GLenum, imageSize: GLsizei,
                                        data: *const GLvoid);
}

// Core aliases
pub const GL_INVALID_FRAMEBUFFER_OPERATION: u32 = GL_INVALID_FRAMEBUFFER_OPERATION_EXT;

extern "C" {
    // Function aliases
    pub fn glActiveTexture(texture: GLenum);
    pub fn glClientActiveTexture(texture: GLenum);
    pub fn glMultiTexCoord2f(target: GLenum, s: GLfloat, t: GLfloat);
    pub fn glGenerateMipmapEXT(target: GLenum);
    pub fn glCompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum,
                                  width: GLsizei, height: GLsizei, border: GLint,
                                  imageSize: GLsizei, data: *const GLvoid);
}

pub const GL_MAX_TEXTURE_LOD_BIAS: u32                          = 0x84FD;
pub const GL_TEXTURE_LOD_BIAS: u32                              = 0x8501;
pub const GL_MAX_TEXTURE_LOD_BIAS_DEFAULT: u32                  = 7;
pub const GL_KOS_INTERNAL_DEFAULT_MIPMAP_LOD_BIAS: u32          = 4;

// GL_EXT_texture_lod_bias
pub const GL_MAX_TEXTURE_LOD_BIAS_EXT: u32                      = 0x84FD;
pub const GL_TEXTURE_FILTER_CONTROL_EXT: u32                    = 0x8500;
pub const GL_TEXTURE_LOD_BIAS_EXT: u32                          = 0x8501;

// ATI_meminfo
pub const GL_VBO_FREE_MEMORY_ATI: u32                           = 0x87FB;
pub const GL_TEXTURE_FREE_MEMORY_ATI: u32                       = 0x87FC;
pub const GL_RENDERBUFFER_FREE_MEMORY_ATI: u32                  = 0x87FD;
