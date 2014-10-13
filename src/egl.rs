// Copyright 2013 Kang Seonghoon. See the COPYRIGHT file at the top-level
// directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bindings to EGL (currently limited to win32 platform)

#![allow(non_uppercase_statics)]

use libc::{c_uint, c_void, c_char, HANDLE};
use std::ptr::{null, null_mut};
use std::string::raw::from_buf;

pub type EGLint = i32;
pub type EGLBoolean = c_uint;
pub type EGLenum = c_uint;
pub type EGLConfig = *const c_void;
pub type EGLContext = *const c_void;
pub type EGLDisplay = *const c_void;
pub type EGLSurface = *const c_void;
pub type EGLClientBuffer = *const c_void;

pub type AttribList<'r> = &'r [(EGLenum, EGLint)];
pub struct Config(EGLConfig);
pub struct Context(EGLContext);
pub struct Display(EGLDisplay);
pub struct Surface(EGLSurface);

#[cfg(target_os="windows")] pub type EGLNativeDisplayType = HANDLE;
#[cfg(target_os="windows")] pub type EGLNativePixmapType = HANDLE;
#[cfg(target_os="windows")] pub type EGLNativeWindowType = HANDLE;

pub const FALSE: EGLBoolean = 0;
pub const TRUE: EGLBoolean = 1;
pub const DEFAULT_DISPLAY: EGLNativeDisplayType = 0 as EGLNativeDisplayType;
pub const NO_CONTEXT: EGLContext = 0 as EGLContext;
pub const NO_DISPLAY: EGLDisplay = 0 as EGLDisplay;
pub const NO_SURFACE: EGLSurface = 0 as EGLSurface;
pub const DONT_CARE: EGLint = -1;
pub const SUCCESS: EGLenum = 0x3000;
pub const NOT_INITIALIZED: EGLenum = 0x3001;
pub const BAD_ACCESS: EGLenum = 0x3002;
pub const BAD_ALLOC: EGLenum = 0x3003;
pub const BAD_ATTRIBUTE: EGLenum = 0x3004;
pub const BAD_CONFIG: EGLenum = 0x3005;
pub const BAD_CONTEXT: EGLenum = 0x3006;
pub const BAD_CURRENT_SURFACE: EGLenum = 0x3007;
pub const BAD_DISPLAY: EGLenum = 0x3008;
pub const BAD_MATCH: EGLenum = 0x3009;
pub const BAD_NATIVE_PIXMAP: EGLenum = 0x300A;
pub const BAD_NATIVE_WINDOW: EGLenum = 0x300B;
pub const BAD_PARAMETER: EGLenum = 0x300C;
pub const BAD_SURFACE: EGLenum = 0x300D;
pub const CONTEXT_LOST: EGLenum = 0x300E;
pub const BUFFER_SIZE: EGLenum = 0x3020;
pub const ALPHA_SIZE: EGLenum = 0x3021;
pub const BLUE_SIZE: EGLenum = 0x3022;
pub const GREEN_SIZE: EGLenum = 0x3023;
pub const RED_SIZE: EGLenum = 0x3024;
pub const DEPTH_SIZE: EGLenum = 0x3025;
pub const STENCIL_SIZE: EGLenum = 0x3026;
pub const CONFIG_CAVEAT: EGLenum = 0x3027;
pub const CONFIG_ID: EGLenum = 0x3028;
pub const LEVEL: EGLenum = 0x3029;
pub const MAX_PBUFFER_HEIGHT: EGLenum = 0x302A;
pub const MAX_PBUFFER_PIXELS: EGLenum = 0x302B;
pub const MAX_PBUFFER_WIDTH: EGLenum = 0x302C;
pub const NATIVE_RENDERABLE: EGLenum = 0x302D;
pub const NATIVE_VISUAL_ID: EGLenum = 0x302E;
pub const NATIVE_VISUAL_TYPE: EGLenum = 0x302F;
pub const SAMPLES: EGLenum = 0x3031;
pub const SAMPLE_BUFFERS: EGLenum = 0x3032;
pub const SURFACE_TYPE: EGLenum = 0x3033;
pub const TRANSPARENT_TYPE: EGLenum = 0x3034;
pub const TRANSPARENT_BLUE_VALUE: EGLenum = 0x3035;
pub const TRANSPARENT_GREEN_VALUE: EGLenum = 0x3036;
pub const TRANSPARENT_RED_VALUE: EGLenum = 0x3037;
pub const NONE: EGLenum = 0x3038;
pub const BIND_TO_TEXTURE_RGB: EGLenum = 0x3039;
pub const BIND_TO_TEXTURE_RGBA: EGLenum = 0x303A;
pub const MIN_SWAP_INTERVAL: EGLenum = 0x303B;
pub const MAX_SWAP_INTERVAL: EGLenum = 0x303C;
pub const LUMINANCE_SIZE: EGLenum = 0x303D;
pub const ALPHA_MASK_SIZE: EGLenum = 0x303E;
pub const COLOR_BUFFER_TYPE: EGLenum = 0x303F;
pub const RENDERABLE_TYPE: EGLenum = 0x3040;
pub const MATCH_NATIVE_PIXMAP: EGLenum = 0x3041;
pub const CONFORMANT: EGLenum = 0x3042;
pub const SLOW_CONFIG: EGLenum = 0x3050;
pub const NON_CONFORMANT_CONFIG: EGLenum = 0x3051;
pub const TRANSPARENT_RGB: EGLenum = 0x3052;
pub const RGB_BUFFER: EGLenum = 0x308E;
pub const LUMINANCE_BUFFER: EGLenum = 0x308F;
pub const NO_TEXTURE: EGLenum = 0x305C;
pub const TEXTURE_RGB: EGLenum = 0x305D;
pub const TEXTURE_RGBA: EGLenum = 0x305E;
pub const TEXTURE_2D: EGLenum = 0x305F;
pub const PBUFFER_BIT: EGLint = 0x0001;
pub const PIXMAP_BIT: EGLint = 0x0002;
pub const WINDOW_BIT: EGLint = 0x0004;
pub const VG_COLORSPACE_LINEAR_BIT: EGLint = 0x0020;
pub const VG_ALPHA_FORMAT_PRE_BIT: EGLint = 0x0040;
pub const MULTISAMPLE_RESOLVE_BOX_BIT: EGLint = 0x0200;
pub const SWAP_BEHAVIOR_PRESERVED_BIT: EGLint = 0x0400;
pub const OPENGL_ES_BIT: EGLint = 0x0001;
pub const OPENVG_BIT: EGLint = 0x0002;
pub const OPENGL_ES2_BIT: EGLint = 0x0004;
pub const OPENGL_BIT: EGLint = 0x0008;
pub const VENDOR: EGLenum = 0x3053;
pub const VERSION: EGLenum = 0x3054;
pub const EXTENSIONS: EGLenum = 0x3055;
pub const CLIENT_APIS: EGLenum = 0x308D;
pub const HEIGHT: EGLenum = 0x3056;
pub const WIDTH: EGLenum = 0x3057;
pub const LARGEST_PBUFFER: EGLenum = 0x3058;
pub const TEXTURE_FORMAT: EGLenum = 0x3080;
pub const TEXTURE_TARGET: EGLenum = 0x3081;
pub const MIPMAP_TEXTURE: EGLenum = 0x3082;
pub const MIPMAP_LEVEL: EGLenum = 0x3083;
pub const RENDER_BUFFER: EGLenum = 0x3086;
pub const VG_COLORSPACE: EGLenum = 0x3087;
pub const VG_ALPHA_FORMAT: EGLenum = 0x3088;
pub const HORIZONTAL_RESOLUTION: EGLenum = 0x3090;
pub const VERTICAL_RESOLUTION: EGLenum = 0x3091;
pub const PIXEL_ASPECT_RATIO: EGLenum = 0x3092;
pub const SWAP_BEHAVIOR: EGLenum = 0x3093;
pub const MULTISAMPLE_RESOLVE: EGLenum = 0x3099;
pub const BACK_BUFFER: EGLenum = 0x3084;
pub const SINGLE_BUFFER: EGLenum = 0x3085;
pub const VG_COLORSPACE_sRGB: EGLenum = 0x3089;
pub const VG_COLORSPACE_LINEAR: EGLenum = 0x308A;
pub const VG_ALPHA_FORMAT_NONPRE: EGLenum = 0x308B;
pub const VG_ALPHA_FORMAT_PRE: EGLenum = 0x308C;
pub const DISPLAY_SCALING: EGLint = 10000;
pub const UNKNOWN: EGLint = -1;
pub const BUFFER_PRESERVED: EGLenum = 0x3094;
pub const BUFFER_DESTROYED: EGLenum = 0x3095;
pub const OPENVG_IMAGE: EGLenum = 0x3096;
pub const CONTEXT_CLIENT_TYPE: EGLenum = 0x3097;
pub const CONTEXT_CLIENT_VERSION: EGLenum = 0x3098;
pub const MULTISAMPLE_RESOLVE_DEFAULT: EGLenum = 0x309A;
pub const MULTISAMPLE_RESOLVE_BOX: EGLenum = 0x309B;
pub const OPENGL_ES_API: EGLenum = 0x30A0;
pub const OPENVG_API: EGLenum = 0x30A1;
pub const OPENGL_API: EGLenum = 0x30A2;
pub const DRAW: EGLenum = 0x3059;
pub const READ: EGLenum = 0x305A;
pub const CORE_NATIVE_ENGINE: EGLenum = 0x305B;
pub const COLORSPACE: EGLenum = VG_COLORSPACE;
pub const ALPHA_FORMAT: EGLenum = VG_ALPHA_FORMAT;
pub const COLORSPACE_sRGB: EGLenum = VG_COLORSPACE_sRGB;
pub const COLORSPACE_LINEAR: EGLenum = VG_COLORSPACE_LINEAR;
pub const ALPHA_FORMAT_NONPRE: EGLenum = VG_ALPHA_FORMAT_NONPRE;
pub const ALPHA_FORMAT_PRE: EGLenum = VG_ALPHA_FORMAT_PRE;

fn wrap_error<T>() -> Result<T,EGLenum> {
    unsafe { Err(eglGetError() as EGLenum) }
}

fn wrap_boolean(v: EGLBoolean) -> Result<(),EGLenum> {
    if v == TRUE {
        Ok(())
    } else {
        wrap_error()
    }
}

fn wrap_display(display: EGLDisplay) -> Result<Display,EGLenum> {
    if display == NO_DISPLAY {
        wrap_error()
    } else {
        Ok(Display(display))
    }
}

fn wrap_surface(surface: EGLSurface) -> Result<Surface,EGLenum> {
    if surface == NO_SURFACE {
        wrap_error()
    } else {
        Ok(Surface(surface))
    }
}

fn wrap_context(context: EGLContext) -> Result<Context,EGLenum> {
    if context == NO_CONTEXT {
        wrap_error()
    } else {
        Ok(Context(context))
    }
}

fn unwrap_attrib_list<T>(attrib_list: AttribList, f: |*const EGLint| -> T) -> T {
    let mut unwrapped = Vec::new();
    for &(attribute, value) in attrib_list.iter() {
        unwrapped.push(attribute as EGLint);
        unwrapped.push(value);
    }
    unwrapped.push(NONE as EGLint);
    f(unwrapped.as_ptr())
}

pub fn get_display(display_id: EGLNativeDisplayType) -> Result<Display,EGLenum> {
    unsafe { wrap_display(eglGetDisplay(display_id)) }
}

pub fn initialize(dpy: Display) -> Result<(int,int),EGLenum> {
    let Display(dpy) = dpy;
    unsafe {
        let mut major = 0;
        let mut minor = 0;
        if eglInitialize(dpy, &mut major, &mut minor) == TRUE {
            Ok((major as int, minor as int))
        } else {
            wrap_error()
        }
    }
}

pub fn terminate(dpy: Display) -> Result<(),EGLenum> {
    let Display(dpy) = dpy;
    unsafe { wrap_boolean(eglTerminate(dpy)) }
}

pub fn query_string(dpy: Display, name: EGLenum) -> String {
    let Display(dpy) = dpy;
    unsafe {
        let s = eglQueryString(dpy, name as EGLint);
        if !s.is_null() {
            from_buf(s as *const u8)
        } else {
            "".to_string()
        }
    }
}

pub fn num_configs(dpy: Display, attrib_list: AttribList) -> Result<int,EGLenum> {
    let Display(dpy) = dpy;
    unsafe {
        let mut actual: EGLint = 0;
        let ret = if attrib_list.is_empty() {
            eglGetConfigs(dpy, null_mut(), 0, &mut actual)
        } else {
            unwrap_attrib_list(attrib_list, |attrib_list| {
                eglChooseConfig(dpy, attrib_list, null_mut(), 0, &mut actual)
            })
        };
        if ret == TRUE {
            Ok(actual as int)
        } else {
            wrap_error()
        }
    }
}

pub fn get_configs(dpy: Display, attrib_list: AttribList, num: Option<uint>) -> Result<Vec<Config>,EGLenum> {
    let Display(dpy) = dpy;
    unsafe {
        let requested = match num {
            Some(n) => n,
            None => match num_configs(Display(dpy), attrib_list) {
                Ok(n) => n as uint,
                Err(err) => return Err(err)
            }
        };
        let mut configs = Vec::from_elem(requested, null());

        let mut actual: EGLint = 0;
        let ll_configs = configs.as_mut_ptr();
        let ll_requested = requested as EGLint;
        let ret = if attrib_list.is_empty() {
            eglGetConfigs(dpy, ll_configs, ll_requested, &mut actual)
        } else {
            unwrap_attrib_list(attrib_list, |attrib_list| {
                eglChooseConfig(dpy, attrib_list, ll_configs, ll_requested, &mut actual)
            })
        };
        if ret == TRUE {
            configs.truncate(actual as uint);
            Ok(configs.into_iter().map(Config).collect())
        } else {
            wrap_error()
        }
    }
}

pub fn get_config_attrib(dpy: Display, config: Config, attribute: EGLenum) -> Result<EGLint,EGLenum> {
    let Display(dpy) = dpy;
    let Config(config) = config;
    unsafe {
        let mut ret: EGLint = 0;
        if eglGetConfigAttrib(dpy, config, attribute as EGLint, &mut ret) == TRUE {
            Ok(ret)
        } else {
            wrap_error()
        }
    }
}

pub fn create_window_surface(dpy: Display, config: Config, win: EGLNativeWindowType, attrib_list: AttribList) -> Result<Surface,EGLenum> {
    let Display(dpy) = dpy;
    let Config(config) = config;
    unsafe {
        unwrap_attrib_list(attrib_list, |attrib_list| {
            wrap_surface(eglCreateWindowSurface(dpy, config, win, attrib_list))
        })
    }
}

pub fn create_pbuffer_surface(dpy: Display, config: Config, attrib_list: AttribList) -> Result<Surface,EGLenum> {
    let Display(dpy) = dpy;
    let Config(config) = config;
    unsafe {
        unwrap_attrib_list(attrib_list, |attrib_list| {
            wrap_surface(eglCreatePbufferSurface(dpy, config, attrib_list))
        })
    }
}

pub fn create_pixmap_surface(dpy: Display, config: Config, pixmap: EGLNativePixmapType, attrib_list: AttribList) -> Result<Surface,EGLenum> {
    let Display(dpy) = dpy;
    let Config(config) = config;
    unsafe {
        unwrap_attrib_list(attrib_list, |attrib_list| {
            wrap_surface(eglCreatePixmapSurface(dpy, config, pixmap, attrib_list))
        })
    }
}

pub fn destroy_surface(dpy: Display, surface: Surface) -> Result<(),EGLenum> {
    let Display(dpy) = dpy;
    let Surface(surface) = surface;
    unsafe { wrap_boolean(eglDestroySurface(dpy, surface)) }
}

pub fn query_surface(dpy: Display, surface: Surface, attribute: EGLenum) -> Result<EGLint,EGLenum> {
    let Display(dpy) = dpy;
    let Surface(surface) = surface;
    unsafe {
        let mut value: EGLint = 0;
        if eglQuerySurface(dpy, surface, attribute as EGLint, &mut value) == TRUE {
            Ok(value)
        } else {
            wrap_error()
        }
    }
}

pub fn bind_api(api: EGLenum) -> bool {
    unsafe { eglBindAPI(api) == TRUE }
}

pub fn query_api() -> EGLenum {
    unsafe { eglQueryAPI() }
}

pub fn wait_client() -> Result<(),EGLenum> {
    unsafe { wrap_boolean(eglWaitClient()) }
}

pub fn release_thread() -> Result<(),EGLenum> {
    unsafe { wrap_boolean(eglReleaseThread()) }
}

pub fn create_pbuffer_from_client_buffer(dpy: Display, buftype: EGLenum, buffer: EGLClientBuffer, config: Config, attrib_list: AttribList) -> Result<Surface,EGLenum> {
    let Display(dpy) = dpy;
    let Config(config) = config;
    unsafe {
        unwrap_attrib_list(attrib_list, |attrib_list| {
            wrap_surface(eglCreatePbufferFromClientBuffer(dpy, buftype, buffer, config, attrib_list))
        })
    }
}

pub fn surface_attrib(dpy: Display, surface: Surface, attribute: EGLenum, value: EGLint) -> Result<(),EGLenum> {
    let Display(dpy) = dpy;
    let Surface(surface) = surface;
    unsafe { wrap_boolean(eglSurfaceAttrib(dpy, surface, attribute as EGLint, value)) }
}

pub fn bind_tex_image(dpy: Display, surface: Surface, buffer: EGLenum) -> Result<(),EGLenum> {
    let Display(dpy) = dpy;
    let Surface(surface) = surface;
    unsafe { wrap_boolean(eglBindTexImage(dpy, surface, buffer as EGLint)) }
}

pub fn release_tex_image(dpy: Display, surface: Surface, buffer: EGLenum) -> Result<(),EGLenum> {
    let Display(dpy) = dpy;
    let Surface(surface) = surface;
    unsafe { wrap_boolean(eglReleaseTexImage(dpy, surface, buffer as EGLint)) }
}

pub fn swap_interval(dpy: Display, interval: EGLint) -> Result<(),EGLenum> {
    let Display(dpy) = dpy;
    unsafe { wrap_boolean(eglSwapInterval(dpy, interval)) }
}

pub fn create_context(dpy: Display, config: Config, share_context: Option<Context>, attrib_list: AttribList) -> Result<Context,EGLenum> {
    let Display(dpy) = dpy;
    let Config(config) = config;
    let share_context = match share_context {
        Some(Context(context)) => context,
        None => null()
    };
    unsafe {
        unwrap_attrib_list(attrib_list, |attrib_list| {
            wrap_context(eglCreateContext(dpy, config, share_context, attrib_list))
        })
    }
}

pub fn destroy_context(dpy: Display, ctx: Context) -> Result<(),EGLenum> {
    let Display(dpy) = dpy;
    let Context(ctx) = ctx;
    unsafe { wrap_boolean(eglDestroyContext(dpy, ctx)) }
}

pub fn make_current(dpy: Display, draw: Surface, read: Surface, ctx: Context) -> Result<(),EGLenum> {
    let Display(dpy) = dpy;
    let Surface(draw) = draw;
    let Surface(read) = read;
    let Context(ctx) = ctx;
    unsafe { wrap_boolean(eglMakeCurrent(dpy, draw, read, ctx)) }
}

pub fn get_current_context() -> Option<Context> {
    unsafe {
        let context = eglGetCurrentContext();
        if context == NO_CONTEXT {
            None
        } else {
            Some(Context(context))
        }
    }
}

fn get_current_surface(readdraw: EGLenum) -> Option<Surface> {
    unsafe {
        let surface = eglGetCurrentSurface(readdraw as EGLint);
        if surface == NO_SURFACE {
            None
        } else {
            Some(Surface(surface))
        }
    }
}

pub fn get_current_read_surface() -> Option<Surface> {
    get_current_surface(READ)
}

pub fn get_current_draw_surface() -> Option<Surface> {
    get_current_surface(DRAW)
}

pub fn get_current_display() -> Option<Display> {
    unsafe {
        let display = eglGetCurrentDisplay();
        if display == NO_DISPLAY {
            None
        } else {
            Some(Display(display))
        }
    }
}

pub fn query_context(dpy: Display, ctx: Context, attribute: EGLenum) -> Result<EGLint,EGLenum> {
    let Display(dpy) = dpy;
    let Context(ctx) = ctx;
    unsafe {
        let mut value: EGLint = 0;
        if eglQueryContext(dpy, ctx, attribute as EGLint, &mut value) == TRUE {
            Ok(value)
        } else {
            wrap_error()
        }
    }
}

pub fn wait_gl() -> Result<(),EGLenum> {
    unsafe { wrap_boolean(eglWaitGL()) }
}

pub fn wait_native(engine: EGLint) -> Result<(),EGLenum> {
    unsafe { wrap_boolean(eglWaitNative(engine)) }
}

pub fn swap_buffers(dpy: Display, surface: Surface) -> Result<(),EGLenum> {
    let Display(dpy) = dpy;
    let Surface(surface) = surface;
    unsafe { wrap_boolean(eglSwapBuffers(dpy, surface)) }
}

pub fn copy_buffers(dpy: Display, surface: Surface, target: EGLNativePixmapType) -> Result<(),EGLenum> {
    let Display(dpy) = dpy;
    let Surface(surface) = surface;
    unsafe { wrap_boolean(eglCopyBuffers(dpy, surface, target)) }
}

pub fn get_proc_address(procname: &str) -> *const c_void {
    unsafe {
        let procname = procname.to_c_str();
        eglGetProcAddress(procname.as_ptr())
    }
}

#[link(name = "EGL")]
extern "system" {
    pub fn eglGetError() -> EGLint;
    pub fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay;
    pub fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean;
    pub fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean;
    pub fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const c_char;
    pub fn eglGetConfigs(dpy: EGLDisplay, configs: *mut EGLConfig, config_size: EGLint, num_config: *mut EGLint) -> EGLBoolean;
    pub fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *const EGLint, configs: *mut EGLConfig, config_size: EGLint, num_config: *mut EGLint) -> EGLBoolean;
    pub fn eglGetConfigAttrib(dpy: EGLDisplay, config: EGLConfig, attribute: EGLint, value: *mut EGLint) -> EGLBoolean;
    pub fn eglCreateWindowSurface(dpy: EGLDisplay, config: EGLConfig, win: EGLNativeWindowType, attrib_list: *const EGLint) -> EGLSurface;
    pub fn eglCreatePbufferSurface(dpy: EGLDisplay, config: EGLConfig, attrib_list: *const EGLint) -> EGLSurface;
    pub fn eglCreatePixmapSurface(dpy: EGLDisplay, config: EGLConfig, pixmap: EGLNativePixmapType, attrib_list: *const EGLint) -> EGLSurface;
    pub fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    pub fn eglQuerySurface(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: *mut EGLint) -> EGLBoolean;
    pub fn eglBindAPI(api: EGLenum) -> EGLBoolean;
    pub fn eglQueryAPI() -> EGLenum;
    pub fn eglWaitClient() -> EGLBoolean;
    pub fn eglReleaseThread() -> EGLBoolean;
    pub fn eglCreatePbufferFromClientBuffer(dpy: EGLDisplay, buftype: EGLenum, buffer: EGLClientBuffer, config: EGLConfig, attrib_list: *const EGLint) -> EGLSurface;
    pub fn eglSurfaceAttrib(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: EGLint) -> EGLBoolean;
    pub fn eglBindTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;
    pub fn eglReleaseTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;
    pub fn eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean;
    pub fn eglCreateContext(dpy: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: *const EGLint) -> EGLContext;
    pub fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;
    pub fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext) -> EGLBoolean;
    pub fn eglGetCurrentContext() -> EGLContext;
    pub fn eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface;
    pub fn eglGetCurrentDisplay() -> EGLDisplay;
    pub fn eglQueryContext(dpy: EGLDisplay, ctx: EGLContext, attribute: EGLint, value: *mut EGLint) -> EGLBoolean;
    pub fn eglWaitGL() -> EGLBoolean;
    pub fn eglWaitNative(engine: EGLint) -> EGLBoolean;
    pub fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    pub fn eglCopyBuffers(dpy: EGLDisplay, surface: EGLSurface, target: EGLNativePixmapType) -> EGLBoolean;
    pub fn eglGetProcAddress(procname: *const c_char) -> *const c_void;
}

