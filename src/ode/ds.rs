//! ds for trait Tdrawstuff and setter to change drawstuff implementation later
//!

use crate::ode::*;

use std::ffi::{c_uint, c_int, c_char};

/// DS_VERSION (for compatibility)
pub const DS_VERSION: c_int = 2;
/// DS_TEXTURE_NUMBER (for compatibililty)
/// pub type DS_TEXTURE_NUMBER = c_int; (drawstuff.h)
pub enum DS_TEXTURE_NUMBER {
  /// no texture only color
  DS_NONE = 0,
  /// wood
  DS_WOOD = 1,
  /// checkered
  DS_CHECKERED = 2,
  /// ground
  DS_GROUND = 3,
  /// sky
  DS_SKY = 4
}

/// dsFunctions_C (for compatibililty)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dsFunctions_C {
  /// version
  pub version: c_int,
  /// start
  pub start: Option<unsafe extern "C" fn()>,
  /// step
  pub step: Option<unsafe extern "C" fn(pause: c_int)>,
  /// command
  pub command: Option<unsafe extern "C" fn(cmd: c_int)>,
  /// stop
  pub stop: Option<unsafe extern "C" fn()>,
  /// path to texture
  pub path_to_textures: *const c_char
}

/// trait Tdrawstuff
pub trait Tdrawstuff {
  /// dsDebug void dsDebug(const char *msg, ...);
  fn Debug(&self, msg: *const c_char); // and some varargs
  /// dsDrawBox 3 12 3
  fn DrawBox(&self, pos: *const f32, rot: *const f32, lxyz: *const f32);
  /// dsDrawBoxD 3 12 3
  fn DrawBoxD(&self, pos: *const f64, rot: *const f64, lxyz: *const f64);
  /// dsDrawCapsule 3 12
  fn DrawCapsule(&self, pos: *const f32, rot: *const f32, l: f32, r: f32);
  /// dsDrawCapsuleD 3 12
  fn DrawCapsuleD(&self, pos: *const f64, rot: *const f64, l: f32, r: f32);
  /// dsDrawConvex 3 12 planecount pointcount x
  fn DrawConvex(&self, pos: *const f32, rot: *const f32,
    planes: *const f32, planecount: c_uint,
    points: *const f32, pointcount: c_uint,
    polygons: *const c_uint);
  /// dsDrawConvexD 3 12 planecount pointcount x
  fn DrawConvexD(&self, pos: *const f64, rot: *const f64,
    planes: *const f64, planecount: c_uint,
    points: *const f64, pointcount: c_uint,
    polygons: *const c_uint);
  /// dsDrawCylinder 3 12
  fn DrawCylinder(&self, pos: *const f32, rot: *const f32, l: f32, r: f32);
  /// dsDrawCylinderD 3 12
  fn DrawCylinderD(&self, pos: *const f64, rot: *const f64, l: f32, r: f32);
  /// dsDrawLine 3 3
  fn DrawLine(&self, pos1: *const f32, pos2: *const f32);
  /// dsDrawLineD 3 3
  fn DrawLineD(&self, pos1: *const f64, pos2: *const f64);
  /// dsDrawSphere 3 12
  fn DrawSphere(&self, pos: *const f32, rot: *const f32, radius: f32);
  /// dsDrawSphereD 3 12
  fn DrawSphereD(&self, pos: *const f64, rot: *const f64, radius: f32);
  /// dsDrawTriangle 3 12 (4 4 4) or (3 3 3)
  fn DrawTriangle(&self, pos: *const f32, rot: *const f32,
    v0: *const f32, v1: *const f32, v2: *const f32, solid: c_int);
  /// dsDrawTriangleD 3 12 (4 4 4) or (3 3 3)
  fn DrawTriangleD(&self, pos: *const f64, rot: *const f64,
    v0: *const f64, v1: *const f64, v2: *const f64, solid: c_int);
  /// dsDrawTriangles 3 12 4n or 3n
  fn DrawTriangles(&self, pos: *const f32, rot: *const f32,
    v: *const f32, n: c_int, solid: c_int);
  /// dsDrawTrianglesD 3 12 4n or 3n
  fn DrawTrianglesD(&self, pos: *const f64, rot: *const f64,
    v: *const f64, n: c_int, solid: c_int);
  /// dsElapsedTime
  fn ElapsedTime(&self) -> f64;
  /// dsError void dsError(const char *msg, ...);
  fn Error(&self, msg: *const c_char); // and some varargs
  /// dsGetViewpoint 3 3
  fn GetViewpoint(&self, xyz: *mut f32, hpr: *mut f32);
  /// dsPrint void dsPrint(const char *msg, ...);
  fn Print(&self, msg: *const c_char); // and some varargs
  /// dsSetCapsuleQuality default 3
  fn SetCapsuleQuality(&self, n: c_int);
  /// dsSetColor
  fn SetColor(&self, red: f32, green: f32, blue: f32);
  /// dsSetColorAlpha
  fn SetColorAlpha(&self, red: f32, green: f32, blue: f32, alpha: f32);
  /// dsSetDrawMode
  fn SetDrawMode(&self, mode: c_int);
  /// dsSetSphereQuality default 1
  fn SetSphereQuality(&self, n: c_int);
  /// dsSetTexture
  fn SetTexture(&self, texture_number: c_int);
  /// dsSetViewpoint 3 3
  fn SetViewpoint(&self, xyz: *mut f32, hpr: *mut f32);
  /// dsSimulationLoop
  fn SimulationLoop(&self, argc: c_int, argv: *mut *mut c_char,
    window_width: c_int, window_height: c_int, functions: *mut dsFunctions_C);
  /// dsStop
  fn Stop(&self);
}

/// trait TdrawstuffSetter
pub trait TdrawstuffSetter {
  /// set drawstuff
  fn set_drawstuff(ds: &mut Option<Box<dyn Tdrawstuff>>,
    drawstuff: impl Tdrawstuff + 'static) {
unsafe {
    *ds = Some(Box::new(drawstuff));
}
  }
}

/// TdrawstuffSetter for ODE
impl TdrawstuffSetter for ODE {
}
