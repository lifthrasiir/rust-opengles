// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Functions for X11 support.

use libc::{c_char, c_int, c_ulong};

// Types

pub enum Display {}

pub type GLXDrawable = c_ulong;

pub enum __GLXFBConfig {}

pub type GLXFBConfig = *mut __GLXFBConfig;

pub type GLXPixmap = c_ulong;

pub type Pixmap = c_ulong;

pub enum __XVisualInfo {}

pub type XVisualInfo = *mut __XVisualInfo;

// Constants

pub const GLX_BIND_TO_TEXTURE_RGB_EXT: c_int     = 0x20d0;
pub const GLX_BIND_TO_TEXTURE_RGBA_EXT: c_int    = 0x20d1;
pub const GLX_BIND_TO_MIPMAP_TEXTURE_EXT: c_int  = 0x20d2;
pub const GLX_BIND_TO_TEXTURE_TARGETS_EXT: c_int = 0x20d3;
pub const GLX_Y_INVERTED_EXT: c_int              = 0x20d4;

pub const GLX_TEXTURE_FORMAT_EXT: c_int = 0x20d5;
pub const GLX_TEXTURE_TARGET_EXT: c_int = 0x20d6;
pub const GLX_MIPMAP_TEXTURE_EXT: c_int = 0x20d7;

pub const GLX_TEXTURE_FORMAT_NONE_EXT: c_int = 0x20d8;
pub const GLX_TEXTURE_FORMAT_RGB_EXT: c_int  = 0x20d9;
pub const GLX_TEXTURE_FORMAT_RGBA_EXT: c_int = 0x20da;

pub const GLX_TEXTURE_1D_BIT_EXT: c_int        = 0x1;
pub const GLX_TEXTURE_2D_BIT_EXT: c_int        = 0x2;
pub const GLX_TEXTURE_RECTANGLE_BIT_EXT: c_int = 0x4;

pub const GLX_TEXTURE_1D_EXT: c_int        = 0x20db;
pub const GLX_TEXTURE_2D_EXT: c_int        = 0x20dc;
pub const GLX_TEXTURE_RECTANGLE_EXT: c_int = 0x20dd;

pub const GLX_FRONT_LEFT_EXT: c_int  = 0x20de;
pub const GLX_FRONT_RIGHT_EXT: c_int = 0x20df;
pub const GLX_BACK_LEFT_EXT: c_int   = 0x20e0;
pub const GLX_BACK_RIGHT_EXT: c_int  = 0x20e1;
pub const GLX_FRONT_EXT: c_int       = GLX_FRONT_LEFT_EXT;
pub const GLX_BACK_EXT: c_int        = GLX_BACK_LEFT_EXT;
pub const GLX_AUX0_EXT: c_int        = 0x20e2;
pub const GLX_AUX1_EXT: c_int        = 0x20e3;
pub const GLX_AUX2_EXT: c_int        = 0x20e4;
pub const GLX_AUX3_EXT: c_int        = 0x20e5;
pub const GLX_AUX4_EXT: c_int        = 0x20e6;
pub const GLX_AUX5_EXT: c_int        = 0x20e7;
pub const GLX_AUX6_EXT: c_int        = 0x20e8;
pub const GLX_AUX7_EXT: c_int        = 0x20e9;
pub const GLX_AUX8_EXT: c_int        = 0x20ea;
pub const GLX_AUX9_EXT: c_int        = 0x20eb;

pub const GLX_DRAWABLE_TYPE: c_int   = 0x8010;
pub const GLX_RENDER_TYPE: c_int     = 0x8011;

pub const GLX_VENDOR: c_int          = 1;

pub const GLX_RGBA: c_int            = 4;
pub const GLX_DOUBLEBUFFER: c_int    = 5;
pub const GLX_ALPHA_SIZE: c_int      = 11;
pub const GLX_DEPTH_SIZE: c_int      = 12;

pub const GLX_WINDOW_BIT: c_int      = 0x01;
pub const GLX_PIXMAP_BIT: c_int      = 0x02;
pub const GLX_PBUFFER_BIT: c_int     = 0x04;
pub const GLX_AUX_BUFFERS_BIT: c_int = 0x10;

pub const GLX_RGBA_BIT: c_int        = 0x00000001;
// Functions

extern {
    pub fn glXQueryVersion(dpy: *mut Display, major: *mut c_int, minor: *mut c_int) -> bool;

    pub fn glXGetProcAddress(procName: *mut c_char) -> extern "C" fn();

    pub fn glXReleaseTexImageEXT(dpy: *mut Display, drawable: GLXDrawable, buffer: c_int);

    pub fn glXChooseFBConfig(dpy: *mut Display,
                             screen: c_int,
                             attrib_list: *mut c_int,
                             n_elements: *mut c_int)
                             -> *mut GLXFBConfig;

    pub fn glXChooseVisual(dpy: *mut Display, screen: c_int, attribList: *mut c_int) -> *mut XVisualInfo;

    // For GLX 1.3+
    pub fn glXCreatePixmap(dpy: *mut Display, config: GLXFBConfig, pixmap: Pixmap, attribList: *mut c_int)
                           -> GLXPixmap;

    pub fn glXDestroyPixmap(dpy: *mut Display, pixmap: GLXPixmap);

    // For GLX < 1.3. Use only to match behavior with other libraries (i.e. Skia) that
    // access GLX pixmaps using the visual instead of fbconfig.
    pub fn glXCreateGLXPixmap(dpy: *mut Display, visual: *mut XVisualInfo, pixmap: Pixmap) -> GLXPixmap;

    pub fn glXDestroyGLXPixmap(dpy: *mut Display, pix: GLXPixmap);

    pub fn glXGetFBConfigAttrib(dpy: *mut Display,
                                config: GLXFBConfig,
                                attribute: c_int,
                                value: *mut c_int)
                                -> c_int;

    pub fn glXGetFBConfigs(dpy: *mut Display, screen: c_int, nelements: *mut c_int) -> *mut GLXFBConfig;

    pub fn glXGetVisualFromFBConfig(dpy: *mut Display, config: GLXFBConfig) -> *mut XVisualInfo;
}

pub fn get_version(display: *mut Display) -> (int, int) {
    unsafe {
        let mut major = 0;
        let mut minor = 0;
        glXQueryVersion(display, &mut major, &mut minor);
        (major as int, minor as int)
    }
}
