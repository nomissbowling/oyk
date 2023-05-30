//! trimeshconvex
//!

use crate::ode::*;
use crate::ode::cls::*;
use crate::ode::krp::*;
use crate::ode::meta::*;

/// constructor trimeshvi
pub trait TriMesh {
  /// constructor trimeshvi
  fn new(vtx: &mut Vec<dReal>, indices: &mut Vec<dTriIndex>) -> trimeshvi {
unsafe {
    trimeshvi{
      vtxCount: (vtx.len() / 3) as u32,
      vtx: vtx.as_ptr_mut(),
      indices: indices.as_ptr_mut(),
      indexCount: indices.len() as u32}
}
  }
}

impl TriMesh for trimeshvi {
}

/// constructor convexfvp
pub trait Convex {
  /// constructor convexfvp
  fn new(planes: &mut Vec<dReal>, vtx: &mut Vec<dReal>,
    polygons: &mut Vec<u32>) -> convexfvp {
unsafe {
    convexfvp{
      faceCount: (planes.len() / 4) as u32,
      faces: planes.as_ptr_mut(),
      vtxCount: (vtx.len() / 3) as u32,
      vtx: vtx.as_ptr_mut(),
      polygons: polygons.as_ptr_mut()}
}
  }
}

impl Convex for convexfvp {
}

pub mod trimesh_tetra;
use trimesh_tetra::*;
pub use trimesh_tetra::{tmv_tetra, fvp_tetra};

pub mod trimesh_cube;
use trimesh_cube::*;
pub use trimesh_cube::{tmv_cube, fvp_cube};

pub mod trimesh_icosahedron;
use trimesh_icosahedron::*;
pub use trimesh_icosahedron::{tmv_icosahedron, fvp_icosahedron};

pub mod trimesh_bunny;
use trimesh_bunny::*;
pub use trimesh_bunny::{tmv_bunny, fvp_bunny};

pub mod trimesh_custom;
use trimesh_custom::*;
pub use trimesh_custom::{tmv_custom, fvp_custom};
