//! cube
//!

use crate::ode::*;

use once_cell::sync::Lazy;

static mut INDICES: Lazy<Vec<dTriIndex>> = Lazy::new(|| vec![ // 12 * 3
  0, 2, 6,
  6, 4, 0,
  1, 0, 4,
  4, 5, 1,
  0, 1, 3,
  3, 2, 0,
  3, 1, 5,
  5, 7, 3,
  2, 3, 7,
  7, 6, 2,
  5, 4, 6,
  6, 7, 5]);

static mut PLANES: Lazy<Vec<dReal>> = Lazy::new(|| vec![ // 6 * 4
  1.0, 0.0, 0.0, 0.25,
  0.0, 1.0, 0.0, 0.25,
  0.0, 0.0, 1.0, 0.25,
  0.0, 0.0, -1.0, 0.25,
  0.0, -1.0, 0.0, 0.25,
  -1.0, 0.0, 0.0, 0.25]); // reset by RecalcFaces()
/*
static mut PLANES: Lazy<Vec<dReal>> = Lazy::new(|| vec![ // 6 * 4
  1.0, 0.0, 0.0, 2.0,
  0.0, 1.0, 0.0, 1.0,
  0.0, 0.0, 1.0, 1.0,
  0.0, 0.0, -1.0, 1.0,
  0.0, -1.0, 0.0, 1.0,
  -1.0, 0.0, 0.0, 0.0]);
*/
static mut VTX: Lazy<Vec<dReal>> = Lazy::new(|| vec![ // 8 * 3;
  0.25, 0.25, 0.25,
  -0.25, 0.25, 0.25,
  0.25, -0.25, 0.25,
  -0.25, -0.25, 0.25,
  0.25, 0.25, -0.25,
  -0.25, 0.25, -0.25,
  0.25, -0.25, -0.25,
  -0.25, -0.25, -0.25]);
static mut POLYGONS: Lazy<Vec<u32>> = Lazy::new(|| vec![ // 6 * (1 + 4)
  4, 0, 2, 6, 4, // +X
  4, 1, 0, 4, 5, // +Y
  4, 0, 1, 3, 2, // +Z
  4, 3, 1, 5, 7, // -X
  4, 2, 3, 7, 6, // -Y
  4, 5, 4, 6, 7]); // -Z

/// unsafe static mut
pub static mut tmv: Lazy<trimeshvi> = Lazy::new(||
  unsafe { trimeshvi::new(&mut VTX, &mut INDICES) });

/// unsafe static mut
pub static mut fvp: Lazy<convexfvp> = Lazy::new(||
  unsafe { convexfvp::new(&mut PLANES, &mut VTX, &mut POLYGONS) });
