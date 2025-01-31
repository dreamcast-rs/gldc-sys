// Raw bindings to GLdc's gl.h header
pub use core::{
    ffi::*,
};

// Define GL types
pub type GLbyte     = c_char;
pub type GLshort    = c_short;
pub type GLint      = c_int;
pub type GLfloat    = c_float;
pub type GLdouble   = c_float;
pub type GLvoid     = c_void;
pub type GLushort   = c_ushort;
pub type GLuint     = c_uint;
pub type GLenum     = c_uint;
pub type GLsizei    = c_uint;
pub type GLfixed    = c_uint;
pub type GLclampf   = c_float;
pub type GLclampd   = c_float;
pub type GLubyte    = c_uchar;
pub type GLbitfield = c_uint;
pub type GLboolean  = c_uchar;

// Primitive Types taken from GL for compatability
// Not all types are implemented in Open GL DC V.1.0
pub const GL_POINTS: u32                                = 0x0000;
pub const GL_LINES: u32                                 = 0x0001;
pub const GL_LINE_LOOP: u32                             = 0x0002;
pub const GL_LINE_STRIP: u32                            = 0x0003;
pub const GL_TRIANGLES: u32                             = 0x0004;
pub const GL_TRIANGLE_STRIP: u32                        = 0x0005;
pub const GL_TRIANGLE_FAN: u32                          = 0x0006;
pub const GL_QUADS: u32                                 = 0x0007;
pub const GL_QUAD_STRIP: u32                            = 0x0008;
pub const GL_POLYGON: u32                               = 0x0009;

// FrontFaceDirection
pub const GL_CW: u32                                    = 0x0900;
pub const GL_CCW: u32                                   = 0x0901;

pub const GL_NONE: u32                                  = 0x0;
pub const GL_FRONT_LEFT: u32                            = 0x0400;
pub const GL_FRONT_RIGHT: u32                           = 0x0401;
pub const GL_BACK_LEFT: u32                             = 0x0402;
pub const GL_BACK_RIGHT: u32                            = 0x0403;
pub const GL_FRONT: u32                                 = 0x0404;
pub const GL_BACK: u32                                  = 0x0405;
pub const GL_LEFT: u32                                  = 0x0406;
pub const GL_RIGHT: u32                                 = 0x0407;
pub const GL_FRONT_AND_BACK: u32                        = 0x0408;
pub const GL_AUX0: u32                                  = 0x0409;
pub const GL_AUX1: u32                                  = 0x040A;
pub const GL_AUX2: u32                                  = 0x040B;
pub const GL_AUX3: u32                                  = 0x040C;
pub const GL_CULL_FACE: u32                             = 0x0B44;
pub const GL_CULL_FACE_MODE: u32                        = 0x0B45;
pub const GL_FRONT_FACE: u32                            = 0x0B46;

// Scissor box
pub const GL_SCISSOR_TEST: u32                          = 0x0C11;
pub const GL_SCISSOR_BOX: u32                           = 0x0C10;

// Stencil actions
pub const GL_KEEP: u32                                  = 0x1E00;
pub const GL_INCR: u32                                  = 0x1E02;
pub const GL_DECR: u32                                  = 0x1E03;
pub const GL_INVERT: u32                                = 0x150A;

// Matrix modes
pub const GL_MATRIX_MODE: u32                           = 0x0BA0;
pub const GL_MODELVIEW: u32                             = 0x1700;
pub const GL_PROJECTION: u32                            = 0x1701;
pub const GL_TEXTURE: u32                               = 0x1702;

pub const GL_MODELVIEW_MATRIX: u32                      = 0x0BA6;
pub const GL_PROJECTION_MATRIX: u32                     = 0x0BA7;
pub const GL_TEXTURE_MATRIX: u32                        = 0x0BA8;

// Depth buffer
pub const GL_NEVER: u32                                 = 0x0200;
pub const GL_LESS: u32                                  = 0x0201;
pub const GL_EQUAL: u32                                 = 0x0202;
pub const GL_LEQUAL: u32                                = 0x0203;
pub const GL_GREATER: u32                               = 0x0204;
pub const GL_NOTEQUAL: u32                              = 0x0205;
pub const GL_GEQUAL: u32                                = 0x0206;
pub const GL_ALWAYS: u32                                = 0x0207;
pub const GL_DEPTH_TEST: u32                            = 0x0B71;
pub const GL_DEPTH_BITS: u32                            = 0x0D56;
pub const GL_DEPTH_CLEAR_VALUE: u32                     = 0x0B73;
pub const GL_DEPTH_FUNC: u32                            = 0x0B74;
pub const GL_DEPTH_RANGE: u32                           = 0x0B70;
pub const GL_DEPTH_WRITEMASK: u32                       = 0x0B72;
pub const GL_DEPTH_COMPONENT: u32                       = 0x1902;

// Blending: Simply Need to Map GL constants to PVR constants
pub const GL_BLEND_DST: u32                             = 0x0BE0;
pub const GL_BLEND_SRC: u32                             = 0x0BE1;
pub const GL_BLEND: u32        /* Compatibility bit */  = 0x0BE2;

pub const GL_ZERO: u32                                  = 0x0;
pub const GL_ONE: u32                                   = 0x1;
pub const GL_SRC_COLOR: u32                             = 0x0300;
pub const GL_ONE_MINUS_SRC_COLOR: u32                   = 0x0301;
pub const GL_SRC_ALPHA: u32                             = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: u32                   = 0x0303;
pub const GL_DST_ALPHA: u32                             = 0x0304;
pub const GL_ONE_MINUS_DST_ALPHA: u32                   = 0x0305;
pub const GL_DST_COLOR: u32                             = 0x0306;
pub const GL_ONE_MINUS_DST_COLOR: u32                   = 0x0307;
pub const GL_SRC_ALPHA_SATURATE: u32                    = 0x0308;

// Misc texture constants
pub const GL_TEXTURE_2D: u32                            = 0x0DE1;
pub const GL_TEXTURE_WRAP_S: u32                        = 0x2802;
pub const GL_TEXTURE_WRAP_T: u32                        = 0x2803;
pub const GL_TEXTURE_MAG_FILTER: u32                    = 0x2800;
pub const GL_TEXTURE_MIN_FILTER: u32                    = 0x2801;
pub const GL_REPEAT: u32                                = 0x2901;
pub const GL_CLAMP: u32                                 = 0x2900;

// Texture Environment
pub const GL_TEXTURE_ENV_MODE: u32                      = 0x2200;
pub const GL_REPLACE: u32                               = 0x1E01;
pub const GL_MODULATE: u32                              = 0x2100;
pub const GL_DECAL: u32                                 = 0x2101;

// TextureMagFilter
pub const GL_NEAREST: u32                               = 0x2600;
pub const GL_LINEAR: u32                                = 0x2601;

// Texture mapping
pub const GL_TEXTURE_ENV: u32                           = 0x2300;
pub const GL_TEXTURE_ENV_COLOR: u32                     = 0x2201;
pub const GL_NEAREST_MIPMAP_NEAREST: u32                = 0x2700;
pub const GL_NEAREST_MIPMAP_LINEAR: u32                 = 0x2702;
pub const GL_LINEAR_MIPMAP_NEAREST: u32                 = 0x2701;
pub const GL_LINEAR_MIPMAP_LINEAR: u32                  = 0x2703;

pub const GL_TEXTURE_BINDING_2D: u32                    = 0x8069;

// TextureUnit
pub const GL_TEXTURE0: u32                              = 0x84C0;
pub const GL_TEXTURE1: u32                              = 0x84C1;
pub const GL_TEXTURE2: u32                              = 0x84C2;
pub const GL_TEXTURE3: u32                              = 0x84C3;
pub const GL_TEXTURE4: u32                              = 0x84C4;
pub const GL_TEXTURE5: u32                              = 0x84C5;
pub const GL_TEXTURE6: u32                              = 0x84C6;
pub const GL_TEXTURE7: u32                              = 0x84C7;
pub const GL_TEXTURE8: u32                              = 0x84C8;
pub const GL_TEXTURE9: u32                              = 0x84C9;
pub const GL_TEXTURE10: u32                             = 0x84CA;
pub const GL_TEXTURE11: u32                             = 0x84CB;
pub const GL_TEXTURE12: u32                             = 0x84CC;
pub const GL_TEXTURE13: u32                             = 0x84CD;
pub const GL_TEXTURE14: u32                             = 0x84CE;
pub const GL_TEXTURE15: u32                             = 0x84CF;
pub const GL_TEXTURE16: u32                             = 0x84D0;
pub const GL_TEXTURE17: u32                             = 0x84D1;
pub const GL_TEXTURE18: u32                             = 0x84D2;
pub const GL_TEXTURE19: u32                             = 0x84D3;
pub const GL_TEXTURE20: u32                             = 0x84D4;
pub const GL_TEXTURE21: u32                             = 0x84D5;
pub const GL_TEXTURE22: u32                             = 0x84D6;
pub const GL_TEXTURE23: u32                             = 0x84D7;
pub const GL_TEXTURE24: u32                             = 0x84D8;
pub const GL_TEXTURE25: u32                             = 0x84D9;
pub const GL_TEXTURE26: u32                             = 0x84DA;
pub const GL_TEXTURE27: u32                             = 0x84DB;
pub const GL_TEXTURE28: u32                             = 0x84DC;
pub const GL_TEXTURE29: u32                             = 0x84DD;
pub const GL_TEXTURE30: u32                             = 0x84DE;
pub const GL_TEXTURE31: u32                             = 0x84DF;
pub const GL_ACTIVE_TEXTURE: u32                        = 0x84E0;
pub const GL_CLIENT_ACTIVE_TEXTURE: u32                 = 0x84E1;

pub const GL_CURRENT_BIT: u32                           = 0x00000001;
pub const GL_POINT_BIT: u32                             = 0x00000002;
pub const GL_LINE_BIT: u32                              = 0x00000004;
pub const GL_POLYGON_BIT: u32                           = 0x00000008;
pub const GL_POLYGON_STIPPLE_BIT: u32                   = 0x00000010;
pub const GL_PIXEL_MODE_BIT: u32                        = 0x00000020;
pub const GL_LIGHTING_BIT: u32                          = 0x00000040;
pub const GL_FOG_BIT: u32                               = 0x00000080;
pub const GL_DEPTH_BUFFER_BIT: u32                      = 0x00000100;
pub const GL_ACCUM_BUFFER_BIT: u32                      = 0x00000200;
pub const GL_STENCIL_BUFFER_BIT: u32                    = 0x00000400;
pub const GL_VIEWPORT_BIT: u32                          = 0x00000800;
pub const GL_TRANSFORM_BIT: u32                         = 0x00001000;
pub const GL_ENABLE_BIT: u32                            = 0x00002000;
pub const GL_COLOR_BUFFER_BIT: u32                      = 0x00004000;
pub const GL_HINT_BIT: u32                              = 0x00008000;
pub const GL_EVAL_BIT: u32                              = 0x00010000;
pub const GL_LIST_BIT: u32                              = 0x00020000;
pub const GL_TEXTURE_BIT: u32                           = 0x00040000;
pub const GL_SCISSOR_BIT: u32                           = 0x00080000;
pub const GL_ALL_ATTRIB_BITS: u32                       = 0x000FFFFF;

// Clip planes
pub const GL_CLIP_PLANE0: u32                           = 0x3000;
pub const GL_CLIP_PLANE1: u32                           = 0x3001;
pub const GL_CLIP_PLANE2: u32                           = 0x3002;
pub const GL_CLIP_PLANE3: u32                           = 0x3003;
pub const GL_CLIP_PLANE4: u32                           = 0x3004;
pub const GL_CLIP_PLANE5: u32                           = 0x3005;

// Fog
pub const GL_FOG: u32                                   = 0x0B60;
pub const GL_FOG_MODE: u32                              = 0x0B65;
pub const GL_FOG_DENSITY: u32                           = 0x0B62;
pub const GL_FOG_COLOR: u32                             = 0x0B66;
pub const GL_FOG_INDEX: u32                             = 0x0B61;
pub const GL_FOG_START: u32                             = 0x0B63;
pub const GL_FOG_END: u32                               = 0x0B64;
       // GL_LINEAR: u32                                = 0x2601;
pub const GL_EXP: u32                                   = 0x0800;
pub const GL_EXP2: u32                                  = 0x0801;

// Hints - Not used by the API, only here for compatibility
pub const GL_DONT_CARE: u32                             = 0x1100;
pub const GL_FASTEST: u32                               = 0x1101;
pub const GL_NICEST: u32                                = 0x1102;
pub const GL_PERSPECTIVE_CORRECTION_HINT: u32           = 0x0C50;
pub const GL_POINT_SMOOTH_HINT: u32                     = 0x0C51;
pub const GL_LINE_SMOOTH_HINT: u32                      = 0x0C52;
pub const GL_POLYGON_SMOOTH_HINT: u32                   = 0x0C53;
pub const GL_FOG_HINT: u32                              = 0x0C54;

// Lighting constants
pub const GL_LIGHTING: u32                              = 0x0b50;

pub const GL_LIGHT0: u32                                = 0x4000;
pub const GL_LIGHT1: u32                                = 0x4001;
pub const GL_LIGHT2: u32                                = 0x4002;
pub const GL_LIGHT3: u32                                = 0x4003;
pub const GL_LIGHT4: u32                                = 0x4004;
pub const GL_LIGHT5: u32                                = 0x4005;
pub const GL_LIGHT6: u32                                = 0x4006;
pub const GL_LIGHT7: u32                                = 0x4007;
pub const GL_LIGHT8: u32                                = 0x4008;
pub const GL_LIGHT9: u32                                = 0x4009;
pub const GL_LIGHT10: u32                               = 0x400A;
pub const GL_LIGHT11: u32                               = 0x400B;
pub const GL_LIGHT12: u32                               = 0x400C;
pub const GL_LIGHT13: u32                               = 0x400D;
pub const GL_LIGHT14: u32                               = 0x400E;
pub const GL_LIGHT15: u32                               = 0x400F;

// EXT_separate_specular_color.txt
pub const GL_LIGHT_MODEL_COLOR_CONTROL: u32             = 0x81F8;
pub const GL_SINGLE_COLOR: u32                          = 0x81F9;
pub const GL_SEPARATE_SPECULAR_COLOR: u32               = 0x81FA;

// glPolygonOffset
pub const GL_POLYGON_OFFSET_FACTOR: u32                 = 0x8038;
pub const GL_POLYGON_OFFSET_UNITS: u32                  = 0x2A00;
pub const GL_POLYGON_OFFSET_POINT: u32                  = 0x2A01;
pub const GL_POLYGON_OFFSET_LINE: u32                   = 0x2A02;
pub const GL_POLYGON_OFFSET_FILL: u32                   = 0x8037;

// Client state caps
pub const GL_VERTEX_ARRAY: u32                          = 0x8074;
pub const GL_NORMAL_ARRAY: u32                          = 0x8075;
pub const GL_COLOR_ARRAY: u32                           = 0x8076;
pub const GL_INDEX_ARRAY: u32                           = 0x8077;
pub const GL_TEXTURE_COORD_ARRAY: u32                   = 0x8078;

// LightParameter
pub const GL_AMBIENT: u32                               = 0x1200;
pub const GL_DIFFUSE: u32                               = 0x1201;
pub const GL_SPECULAR: u32                              = 0x1202;
pub const GL_POSITION: u32                              = 0x1203;
pub const GL_SPOT_DIRECTION: u32                        = 0x1204;
pub const GL_SPOT_EXPONENT: u32                         = 0x1205;
pub const GL_SPOT_CUTOFF: u32                           = 0x1206;
pub const GL_CONSTANT_ATTENUATION: u32                  = 0x1207;
pub const GL_LINEAR_ATTENUATION: u32                    = 0x1208;
pub const GL_QUADRATIC_ATTENUATION: u32                 = 0x1209;

// MaterialParameter
pub const GL_EMISSION: u32                              = 0x1600;
pub const GL_SHININESS: u32                             = 0x1601;
pub const GL_AMBIENT_AND_DIFFUSE: u32                   = 0x1602;
pub const GL_COLOR_INDEXES: u32                         = 0x1603;
pub const GL_COLOR_MATERIAL: u32                        = 0x0B57;
pub const GL_COLOR_MATERIAL_FACE: u32                   = 0x0B55;
pub const GL_COLOR_MATERIAL_PARAMETER: u32              = 0x0B56;
pub const GL_NORMALIZE: u32                             = 0x0BA1;
pub const GL_LIGHT_MODEL_TWO_SIDE: u32                  = 0x0B52;
pub const GL_LIGHT_MODEL_LOCAL_VIEWER: u32              = 0x0B51;
pub const GL_LIGHT_MODEL_AMBIENT: u32                   = 0x0B53;
       // GL_FRONT_AND_BACK: u32                        = 0x0408;
       // GL_FRONT: u32                                 = 0x0404;
       // GL_BACK: u32                                  = 0x0405;

pub const GL_SHADE_MODEL: u32                           = 0x0b54;
pub const GL_FLAT: u32                                  = 0x1d00;
pub const GL_SMOOTH: u32                                = 0x1d01;

// Data types
pub const GL_BYTE: u32                                  = 0x1400;
pub const GL_UNSIGNED_BYTE: u32                         = 0x1401;
pub const GL_SHORT: u32                                 = 0x1402;
pub const GL_UNSIGNED_SHORT: u32                        = 0x1403;
pub const GL_INT: u32                                   = 0x1404;
pub const GL_UNSIGNED_INT: u32                          = 0x1405;
pub const GL_FLOAT: u32                                 = 0x1406;
pub const GL_DOUBLE: u32                                = 0x140A;
pub const GL_2_BYTES: u32                               = 0x1407;
pub const GL_3_BYTES: u32                               = 0x1408;
pub const GL_4_BYTES: u32                               = 0x1409;

// ErrorCode 
pub const GL_NO_ERROR: u32                              = 0x0;
pub const GL_INVALID_ENUM: u32                          = 0x0500;
pub const GL_INVALID_VALUE: u32                         = 0x0501;
pub const GL_INVALID_OPERATION: u32                     = 0x0502;
pub const GL_STACK_OVERFLOW: u32                        = 0x0503;
pub const GL_STACK_UNDERFLOW: u32                       = 0x0504;
pub const GL_OUT_OF_MEMORY: u32                         = 0x0505;

// GetPName
pub const GL_SMOOTH_POINT_SIZE_RANGE: u32               = 0x0B12;
pub const GL_SMOOTH_LINE_WIDTH_RANGE: u32               = 0x0B22;
pub const GL_ALIASED_POINT_SIZE_RANGE: u32              = 0x846D;
pub const GL_ALIASED_LINE_WIDTH_RANGE: u32              = 0x846E;
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE_OES: u32    = 0x8B9A;
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT_OES: u32  = 0x8B9B;
pub const GL_MAX_LIGHTS: u32                            = 0x0D31;
pub const GL_MAX_TEXTURE_SIZE: u32                      = 0x0D33;
pub const GL_MAX_MODELVIEW_STACK_DEPTH: u32             = 0x0D36;
pub const GL_MAX_PROJECTION_STACK_DEPTH: u32            = 0x0D38;
pub const GL_MAX_TEXTURE_STACK_DEPTH: u32               = 0x0D39;
pub const GL_MAX_VIEWPORT_DIMS: u32                     = 0x0D3A;
pub const GL_MAX_ELEMENTS_VERTICES: u32                 = 0x80E8;
pub const GL_MAX_ELEMENTS_INDICES: u32                  = 0x80E9;
pub const GL_MAX_TEXTURE_UNITS: u32                     = 0x84E2;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: u32        = 0x86A2;
pub const GL_COMPRESSED_TEXTURE_FORMATS: u32            = 0x86A3;
pub const GL_SUBPIXEL_BITS: u32                         = 0x0D50;
pub const GL_RED_BITS: u32                              = 0x0D52;
pub const GL_GREEN_BITS: u32                            = 0x0D53;
pub const GL_BLUE_BITS: u32                             = 0x0D54;
pub const GL_ALPHA_BITS: u32                            = 0x0D55;
       // GL_DEPTH_BITS: u32                            = 0x0D56;
pub const GL_STENCIL_BITS: u32                          = 0x0D57;

// StringName
pub const GL_VENDOR: u32                                = 0x1F00;
pub const GL_RENDERER: u32                              = 0x1F01;
pub const GL_VERSION: u32                               = 0x1F02;
pub const GL_EXTENSIONS: u32                            = 0x1F03;

// GL KOS Texture Matrix Enable Bit
pub const GL_KOS_TEXTURE_MATRIX: u32                    = 0x002F;

pub const GL_UNSIGNED_SHORT_4_4_4_4: u32                = 0x8033;
pub const GL_UNSIGNED_SHORT_5_5_5_1: u32                = 0x8034;
pub const GL_UNSIGNED_SHORT_5_6_5: u32                  = 0x8363;
pub const GL_UNSIGNED_SHORT_5_6_5_REV: u32              = 0x8364;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: u32            = 0x8365;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: u32            = 0x8366;
pub const GL_UNSIGNED_INT_8_8_8_8_REV: u32              = 0x8367;
pub const GL_UNSIGNED_INT_2_10_10_10_REV: u32           = 0x8368;

pub const GL_COLOR_INDEX: u32                           = 0x1900;
pub const GL_RED: u32                                   = 0x1903;
pub const GL_GREEN: u32                                 = 0x1904;
pub const GL_BLUE: u32                                  = 0x1905;
pub const GL_ALPHA: u32                                 = 0x1906;
pub const GL_RGB: u32                                   = 0x1907;
pub const GL_RGBA: u32                                  = 0x1908;
pub const GL_LUMINANCE: u32                             = 0x1909;
pub const GL_LUMINANCE_ALPHA: u32                       = 0x190A;

pub const GL_R3_G3_B2: u32                              = 0x2A10;

pub const GL_ALPHA4: u32                                = 0x803B;
pub const GL_ALPHA8: u32                                = 0x803C;
pub const GL_ALPHA12: u32                               = 0x803D;
pub const GL_ALPHA16: u32                               = 0x803E;

pub const GL_LUMINANCE4: u32                            = 0x803F;
pub const GL_LUMINANCE8: u32                            = 0x8040;
pub const GL_LUMINANCE12: u32                           = 0x8041;
pub const GL_LUMINANCE16: u32                           = 0x8042;

pub const GL_LUMINANCE4_ALPHA4: u32                     = 0x8043;
pub const GL_LUMINANCE6_ALPHA2: u32                     = 0x8044;
pub const GL_LUMINANCE8_ALPHA8: u32                     = 0x8045;
pub const GL_LUMINANCE12_ALPHA4: u32                    = 0x8046;
pub const GL_LUMINANCE12_ALPHA12: u32                   = 0x8047;
pub const GL_LUMINANCE16_ALPHA16: u32                   = 0x8048;

pub const GL_INTENSITY4: u32                            = 0x804A;
pub const GL_INTENSITY8: u32                            = 0x804B;
pub const GL_INTENSITY12: u32                           = 0x804C;
pub const GL_INTENSITY16: u32                           = 0x804D;

pub const GL_BGR: u32                                   = 0x80E0;
pub const GL_BGRA: u32                                  = 0x80E1;
pub const GL_INTENSITY: u32                             = 0x8049;
pub const GL_RGB4: u32                                  = 0x804F;
pub const GL_RGB5: u32                                  = 0x8050;
pub const GL_RGB8: u32                                  = 0x8051;
pub const GL_RGB10: u32                                 = 0x8052;
pub const GL_RGB12: u32                                 = 0x8053;
pub const GL_RGB16: u32                                 = 0x8054;
pub const GL_RGBA2: u32                                 = 0x8055;
pub const GL_RGBA4: u32                                 = 0x8056;
pub const GL_RGB5_A1: u32                               = 0x8057;
pub const GL_RGBA8: u32                                 = 0x8058;
pub const GL_RGB10_A2: u32                              = 0x8059;
pub const GL_RGBA12: u32                                = 0x805A;
pub const GL_RGBA16: u32                                = 0x805B;

pub const GL_R8: u32                                    = 0x8229;
pub const GL_RG8: u32                                   = 0x822B;
pub const GL_RG: u32                                    = 0x8227;
pub const GL_R16: u32                                   = 0x822A;
pub const GL_RG16: u32                                  = 0x822C;
pub const GL_COMPRESSED_RED: u32                        = 0x8225;
pub const GL_COMPRESSED_RG: u32                         = 0x8226;

// Polygons
pub const GL_POINT: u32	                                = 0x1B00;
pub const GL_LINE: u32                                  = 0x1B01;
pub const GL_FILL: u32                                  = 0x1B02;
       // GL_CW: u32                                    = 0x0900;
       // GL_CCW: u32                                   = 0x0901;
       // GL_FRONT: u32                                 = 0x0404;
       // GL_BACK: u32                                  = 0x0405;
pub const GL_POLYGON_MODE: u32                          = 0x0B40;
pub const GL_POLYGON_SMOOTH: u32                        = 0x0B41;
pub const GL_POLYGON_STIPPLE: u32                       = 0x0B42;
pub const GL_EDGE_FLAG: u32                             = 0x0B43;
       // GL_CULL_FACE: u32                             = 0x0B44;
       // GL_CULL_FACE_MODE: u32                        = 0x0B45;
       // GL_FRONT_FACE: u32                            = 0x0B46;
       // GL_POLYGON_OFFSET_FACTOR: u32                 = 0x8038;
       // GL_POLYGON_OFFSET_UNITS: u32                  = 0x2A00;
       // GL_POLYGON_OFFSET_POINT: u32                  = 0x2A01;
       // GL_POLYGON_OFFSET_LINE: u32                   = 0x2A02;
       // GL_POLYGON_OFFSET_FILL: u32                   = 0x8037;

pub const GL_FALSE: u32                                 = 0;
pub const GL_TRUE: u32                                  = 1;

// Stubs for portability
pub const GL_LINE_SMOOTH: u32                           = 0x0B20;
pub const GL_ALPHA_TEST: u32                            = 0x0BC0;
pub const GL_STENCIL_TEST: u32                          = 0x0B90;
pub const GL_STENCIL_WRITEMASK: u32                     = 0x0B98;
pub const GL_INDEX_WRITEMASK: u32                       = 0x0C21;
pub const GL_COLOR_WRITEMASK: u32                       = 0x0C23;
pub const GL_UNPACK_SWAP_BYTES: u32                     = 0x0CF0;
pub const GL_UNPACK_LSB_FIRST: u32                      = 0x0CF1;
pub const GL_UNPACK_ROW_LENGTH : u32                    = 0x0CF2;
pub const GL_UNPACK_SKIP_ROWS: u32                      = 0x0CF3;
pub const GL_UNPACK_SKIP_PIXELS: u32                    = 0x0CF4;
pub const GL_UNPACK_ALIGNMENT: u32                      = 0x0CF5;
pub const GL_PACK_SWAP_BYTES: u32                       = 0x0D00;
pub const GL_PACK_LSB_FIRST: u32                        = 0x0D01;
pub const GL_PACK_ROW_LENGTH : u32                      = 0x0D02;
pub const GL_PACK_SKIP_ROWS: u32                        = 0x0D03;
pub const GL_PACK_SKIP_PIXELS: u32                      = 0x0D04;
pub const GL_PACK_ALIGNMENT: u32                        = 0x0D05;

pub const GL_MULTISAMPLE: u32                           = 0x809D;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: u32              = 0x809E;
pub const GL_SAMPLE_ALPHA_TO_ONE: u32                   = 0x809F;
pub const GL_SAMPLE_COVERAGE: u32                       = 0x80A0;
pub const GL_SAMPLE_BUFFERS : u32                       = 0x80A8;
pub const GL_SAMPLES: u32                               = 0x80A9;
pub const GL_SAMPLE_COVERAGE_VALUE: u32                 = 0x80AA;
pub const GL_SAMPLE_COVERAGE_INVERT: u32                = 0x80AB;
pub const GL_MULTISAMPLE_BIT: u32                       = 0x20000000;

#[link(name = "GL")]
extern "C" {
    pub fn glFlush();
    pub fn glFinish();

    // Start Submission of Primitive Data
    // Currently Supported Primitive Types:
    // -GL_POINTS   ( does NOT work with glDrawArrays )( ZClipping NOT supported )
    // -GL_TRIANGLES        ( works with glDrawArrays )( ZClipping supported )
    // -GL_TRIANLGLE_STRIP  ( works with glDrawArrays )( ZClipping supported )
    // -GL_QUADS            ( works with glDrawArrays )( ZClipping supported )

    pub fn glBegin(mode: GLenum);

    // Finish Submission of Primitive Data
    pub fn glEnd();

    // Primitive Texture Coordinate Submission
    pub fn glTexCoord1f(u: GLfloat);
    pub fn glTexCoord1fv(u: *const GLfloat);
    pub fn glTexCoord2f(u: GLfloat, v: GLfloat);
    pub fn glTexCoord2fv(uv: *const GLfloat);

    // Primitive Color Submission
    pub fn glColor1ui(argb: GLuint);
    pub fn glColor4ub(r: GLubyte, g: GLubyte, b: GLubyte, a: GLubyte);
    pub fn glColor4ubv(v: *const GLubyte);
    pub fn glColor3f(r: GLfloat, g: GLfloat, b: GLfloat);
    pub fn glColor3ub(r: GLubyte, g: GLubyte, b: GLubyte);
    pub fn glColor3ubv(v: *const GLubyte);
    pub fn glColor3fv(rgb: *const GLfloat);
    pub fn glColor4f(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat);
    pub fn glColor4fv(rgba: *const GLfloat);

    // Primitive Normal Submission
    pub fn glNormal3f(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glNormal3i(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glNormal3fv(xyz: *const GLfloat);
    pub fn glNormal3iv(xyz: *const GLfloat);

    // Primitive 2D Position Submission
    pub fn glVertex2f(x: GLfloat, y: GLfloat);
    pub fn glVertex2i(x: GLfloat, y: GLfloat);
    pub fn glVertex2fv(xy: *const GLfloat);
    pub fn glVertex2iv(xy: *const GLfloat);

    // Primitive 3D Position Submission
    pub fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glVertex3fv(xyz: *const GLfloat);

    // 2D Non-Textured Rectangle Submission
    pub fn glRectf(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat) -> GLvoid;
    pub fn glRectd(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat) -> GLvoid;
    pub fn glRectfv(v1: *const GLfloat, v2: *const GLfloat) -> GLvoid;
    pub fn glRectdv(v1: *const GLfloat, v2: *const GLfloat) -> GLvoid;
    pub fn glRecti(x1: GLint, y1: GLint, x2: GLint, y2: GLint) -> GLvoid;
    pub fn glRects(x1: GLint, y1: GLint, x2: GLint, y2: GLint) -> GLvoid;
    pub fn glRectiv(v1: *const GLint, v2: *const GLint) -> GLvoid;
    pub fn glRectsv(v1: *const GLint, v2: *const GLint) -> GLvoid;

    // Primitive configuration
    pub fn glLineWidth(width: GLfloat);
    pub fn glPointSize(size: GLfloat);

    // Enable / Disable Capability
    // Currently Supported Capabilities:
    //      GL_TEXTURE_2D
    //      GL_BLEND
    //      GL_DEPTH_TEST
    //      GL_LIGHTING
    //      GL_SCISSOR_TEST
    //      GL_FOG
    //      GL_CULL_FACE
    //      GL_KOS_NEARZ_CLIPPING
    //      GL_KOS_TEXTURE_MATRIX

    pub fn glEnable(cap: GLenum);
    pub fn glDisable(cap: GLenum);

    // Clear Caps
    pub fn glClear(mode: GLuint);
    pub fn glClearColor(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat);

    pub fn glReadBuffer(mode: GLenum);
    pub fn glDrawBuffer(mode: GLenum);

    // Depth Testing
    pub fn glClearDepth(depth: GLdouble);
    pub fn glClearDepthf(depth: GLfloat);
    pub fn glDepthMask(flag: GLboolean);
    pub fn glDepthFunc(func: GLenum);
    pub fn glDepthRange(n: GLdouble, f: GLdouble);
    pub fn glDepthRangef(n: GLfloat, f: GLfloat);

    // Hints
    // Currently Supported Capabilities:
    //    GL_PERSPECTIVE_CORRECTION_HINT - This will Enable Texture Super-Sampling on the PVR
    pub fn glHint(target: GLenum, mode: GLenum);

    // Culling
    pub fn glFrontFace(mode: GLenum);
    pub fn glCullFace(mode: GLenum);

    // Shading - Flat or Goraud
    pub fn glShadeModel(mode: GLenum);

    // Blending
    pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);

    // Texturing
    pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    pub fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
    pub fn glTexEnvi(target: GLenum, pname: GLenum, param: GLint);
    pub fn glTexEnvf(target: GLenum, pname: GLenum, param: GLfloat);

    pub fn glIsTexture(texture: GLuint) -> GLboolean;
    pub fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    pub fn glDeleteTextures(n: GLsizei, textures: *mut GLuint);
    pub fn glBindTexture(target: GLenum, texture: GLuint);

    // Loads texture from SH4 RAM into PVR VRAM applying color conversion if needed
    // internalformat must be one of the following constants:
    //   GL_RGB
    //   GL_RGBA
    //
    // format must be the same as internalformat
    //
    // if internal format is GL_RGB, type must be one of the following constants:
    //   GL_BYTE
    //   GL_UNSIGNED_BYTE
    //   GL_SHORT
    //   GL_UNSIGNED_SHORT
    //   GL_FLOAT
    //   GL_UNSIGNED_SHORT_5_6_5
    //   GL_UNSIGNED_SHORT_5_6_5_TWID
    //
    // if internal format is GL_RGBA, type must be one of the following constants:
    //   GL_BYTE
    //   GL_UNSIGNED_BYTE
    //   GL_SHORT
    //   GL_UNSIGNED_SHORT
    //   GL_FLOAT
    //   GL_UNSIGNED_SHORT_4_4_4_4
    //   GL_UNSIGNED_SHORT_4_4_4_4_TWID
    //   GL_UNSIGNED_SHORT_1_5_5_5
    //   GL_UNSIGNED_SHORT_1_5_5_5_TWID

    pub fn glTexImage2D(target: GLenum, level: GLint, internalFormat: GLint,
                        width: GLsizei, height: GLsizei, border: GLint,
                        format: GLenum, r#type: GLenum, data: *const GLvoid);

    pub fn glTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint,
                           width: GLsizei, height: GLsizei, format: GLenum, 
                           r#type: GLenum, pixels: *const GLvoid);
    pub fn glCopyTexSubImage2D(target: GLenum, level: GLint,
                               xoffset: GLint, yoffset: GLint,
                               x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glCopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint,
                               x: GLint, y: GLint, width: GLsizei);
    pub fn glCopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum,
                            x: GLint, y: GLint, width: GLsizei, height: GLsizei,
                            border: GLint);
    pub fn glCopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum,
                            x: GLint, y: GLint, width: GLsizei, border: GLint);
    pub fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei,
                        format: GLenum, r#type: GLenum, pixels: *mut GLvoid);


    // GL Array API - Only GL_TRIANGLES, GL_TRIANGLE_STRIP, and GL_QUADS are supported
    pub fn glVertexPointer(size: GLint, r#type: GLenum,
                           stride: GLsizei, pointer: *const GLvoid);

    pub fn glTexCoordPointer(size: GLint, r#type: GLenum,
                             stride: GLsizei, pointer: *const GLvoid);

    // If a Normal Pointer is set and GL Lighting has been enabled,
    // Vertex Lighting will be used instead of glColorPointer
    pub fn glNormalPointer(r#type: GLenum, stride: GLsizei, pointer: *const GLvoid);

    // Use either this OR glNormalPointer to color vertices, NOT both
    pub fn glColorPointer(size: GLint, r#type: GLenum,
                          stride: GLsizei, pointer: *const GLvoid);

    // Array Data Submission
    pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
    pub fn glDrawElements(mode: GLenum, count: GLsizei, r#type: GLenum,
                          indices: *const GLvoid);

    pub fn glEnableClientState(cap: GLenum);
    pub fn glDisableClientState(cap: GLenum);

    // Transformation / Matrix Functions

    pub fn glMatrixMode(mode: GLenum);

    pub fn glLoadIdentity();

    pub fn glLoadMatrixf(m: *const GLfloat);
    pub fn glLoadTransposeMatrixf(m: *const GLfloat);
    pub fn glMultMatrixf(m: *const GLfloat);
    pub fn glMultTransposeMatrixf(m: *const GLfloat);

    pub fn glPushMatrix();
    pub fn glPopMatrix();

    pub fn glTranslatef(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glTranslated(x: GLfloat, y: GLfloat, z: GLfloat);

    pub fn glScalef(x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glScaled(x: GLfloat, y: GLfloat, z: GLfloat);

    pub fn glRotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glRotated(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);

    pub fn glOrtho(left: GLdouble, right: GLdouble,
                   bottom: GLdouble, top: GLdouble,
                   znear: GLdouble, zfar: GLdouble);

    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

    pub fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

    pub fn glKosGetMatrix(mode: GLenum, params: *mut GLfloat);

    pub fn glFrustum(left: GLdouble, right: GLdouble,
                     bottom: GLdouble, top: GLdouble,
                     znear: GLdouble, zfar: GLdouble);

    // Fog Functions - client must enable GL_FOG for this to take effect
    pub fn glFogi(pname: GLenum, param: GLint);
    pub fn glFogf(pname: GLenum, param: GLfloat);
    pub fn glFogiv(pname: GLenum, params: *const GLint);
    pub fn glFogfv(pname: GLenum, params: *const GLfloat);

    // Lighting Functions - client must enable GL_LIGHTING for this to take effect

    // Set Individual Light Parameters
    pub fn glLightfv(light: GLenum, pname: GLenum, params: *const GLfloat);
    pub fn glLightf(light: GLenum, pname: GLenum, param: GLfloat);

    pub fn glLightModelf(pname: GLenum, param: GLfloat);
    pub fn glLightModeli(pname: GLenum, param: GLint);
    pub fn glLightModelfv(pname: GLenum, params: *const GLfloat);
    pub fn glLightModeliv(pname: GLenum, params: *const GLint);


    // Set Global Material Parameters
    pub fn glMateriali(face: GLenum, pname: GLenum, param: GLint);
    pub fn glMaterialf(face: GLenum, pname: GLenum, param: GLfloat);
    pub fn glMaterialfv(face: GLenum, pname: GLenum, params: *const GLfloat);
    pub fn glColorMaterial(face: GLenum, mode: GLenum);

    // glGet Functions
    pub fn glGetBooleanv(pname: GLenum, params: *mut GLboolean);
    pub fn glGetIntegerv(pname: GLenum, params: *mut GLint);
    pub fn glGetFloatv(pname: GLenum, params: *mut GLfloat);
    pub fn glIsEnabled(cap: GLenum) -> GLboolean;
    pub fn glGetString(name: GLenum) -> *const GLubyte;

    // Error handling
    pub fn glGetError() -> GLenum;

    // Non Operational Stubs for portability
    pub fn glAlphaFunc(func: GLenum, r#ref: GLclampf);
    pub fn glPolygonMode(face: GLenum, mode: GLenum);
    pub fn glPolygonOffset(factor: GLfloat, units: GLfloat);
    pub fn glGetTexParameterfv(target: GLenum, pname: GLenum, paramas: *mut GLfloat);
    pub fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
    pub fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean,
                       alpha: GLboolean);
    pub fn glPixelStorei(pname: GLenum, param: GLint);
    pub fn glStencilFunc(func: GLenum, r#ref: GLint, mask: GLuint);
    pub fn glStencilOp(sfail: GLenum, dpfail: GLenum, dppass: GLenum);
    pub fn glGetTexImage(tex: GLenum, lod: GLint, format: GLenum, r#type: GLenum,
                         img: *mut GLvoid);
}
