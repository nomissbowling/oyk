//! trimesh_tetra
//!

use crate::ode::*;

use once_cell::sync::Lazy;

static mut INDICES: Lazy<Vec<dTriIndex>> = Lazy::new(|| vec![ // 4 * 3
  3, 1, 0,
  3, 2, 1,
  3, 0, 2,
  2, 0, 1]);

static mut PLANES: Lazy<Vec<dReal>> = Lazy::new(|| vec![ // 4 * 4
  0.0, 0.0, -1.0, 0.2041,
  -0.9107, 0.2440, 0.3333, 0.2041,
  0.2440, -0.9107, 0.3333, 0.2041,
  0.6667, 0.6667, 0.3334, 0.2042]); // reset by RecalcFaces()
static mut VTX: Lazy<Vec<dReal>> = Lazy::new(|| vec![ // 4 * 3;
  0.5577, -0.1494, -0.2041, // (r6+3r2)/12, (r6-3r2)/12, -r6/12
  -0.1494, 0.5577, -0.2041, // (r6-3r2)/12, (r6+3r2)/12, -r6/12
  0.0, 0.0, 0.6124, // 0, 0, r6/4
  -0.4082, -0.4082, -0.2041]); // -r6/6, -r6/6, -r6/12
static mut POLYGONS: Lazy<Vec<u32>> = Lazy::new(|| vec![ // 4 * (1 + 3)
  3, 3, 1, 0,
  3, 3, 2, 1,
  3, 3, 0, 2,
  3, 2, 0, 1]);

/// unsafe static mut
pub static mut tmv_tetra: Lazy<trimeshvi> = Lazy::new(||
  unsafe { trimeshvi::new(&mut VTX, &mut INDICES) });

/// unsafe static mut
pub static mut fvp_tetra: Lazy<convexfvp> = Lazy::new(||
  unsafe { convexfvp::new(&mut PLANES, &mut VTX, &mut POLYGONS) });
