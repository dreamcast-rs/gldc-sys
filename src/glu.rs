// Raw bindings to GLdc's glu.h header
use crate::gl::*;

pub const GLU_FALSE: u32 = 0;
pub const GLU_TRUE: u32  = 1;

extern "C" {
    pub fn gluOrtho2D(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble);

    // gluPerspective - Set the Perspective for Rendering.
    pub fn gluPerspective(fovy: GLdouble, aspect: GLdouble,
                          znear: GLdouble, zFar: GLdouble);

    // gluLookAt - Set Camera Position for Rendering
    pub fn gluLookAt(eyex: GLdouble, eyey: GLdouble, eyez: GLdouble,
                     centerx: GLdouble, centery: GLdouble, centerz: GLdouble,
                     upx: GLdouble, upy: GLdouble, upz: GLdouble);

    // generate mipmaps for any image provided by the user and then pass them to OpenGL
    pub fn gluBuild2DMipmaps(target: GLenum, internalFormat: GLint,
                             width: GLsizei, height: GLsizei,
                             format: GLenum, r#type: GLenum,
                             data: *const c_void) -> GLint;

    pub fn gluErrorString(error: GLenum) -> *const GLubyte;
}
