//! custom
//!

use crate::ode::*;

use once_cell::sync::Lazy;

static mut INDICES: Lazy<Vec<dTriIndex>> = Lazy::new(|| vec![ // 4 * 3
  3, 1, 0,
  3, 2, 1,
  3, 0, 2,
  2, 0, 1]);

static mut PLANES: Lazy<Vec<dReal>> = Lazy::new(|| vec![ // 4 * 4
  0.0, 0.0, -1.0, 0.0, // 0.25
  -1.0, 0.0, 0.0, 0.0, // 0.25
  0.0, -1.0, 0.0, 0.0, // 0.25
  0.57735, 0.57735, 0.57735, 0.57735]); // 0.14434 reset by RecalcFaces()
static mut VTX: Lazy<Vec<dReal>> = Lazy::new(|| vec![ // 4 * 3;
  0.75, -0.25, -0.25, // 1.0, 0.0, 0.0,
  -0.25, 0.75, -0.25, // 0.0, 1.0, 0.0,
  -0.25, -0.25, 0.75, // 0.0, 0.0, 1.0,
  -0.25, -0.25, -0.25]); // 0.0, 0.0, 0.0]);
static mut POLYGONS: Lazy<Vec<u32>> = Lazy::new(|| vec![ // 4 * (1 + 3)
  3, 3, 1, 0,
  3, 3, 2, 1,
  3, 3, 0, 2,
  3, 2, 0, 1]);

/// unsafe static mut
pub static mut tmv: Lazy<trimeshvi> = Lazy::new(||
  unsafe { trimeshvi::new(&mut VTX, &mut INDICES) });

/// unsafe static mut
pub static mut fvp: Lazy<convexfvp> = Lazy::new(||
  unsafe { convexfvp::new(&mut PLANES, &mut VTX, &mut POLYGONS) });
