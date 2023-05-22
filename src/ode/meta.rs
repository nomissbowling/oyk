//! meta
//!

use crate::ode::*;
use crate::ode::cls::*;

/// id for trait MetaInf
#[derive(Debug)]
pub enum MetaId {
  /// ODE Sphere
  Sphere = 0,
  /// ODE Box
  Box,
  /// ODE Capsule
  Capsule,
  /// ODE Cylinder
  Cylinder,
  /// ODE Plane
  Plane,
  /// ODE Ray
  Ray,
  /// ODE Convex
  Convex,
  /// ODE GeomTransform
  GeomTransform,
  /// ODE TriMesh
  TriMesh,
  /// ODE Heightfield
  Heightfield
}

/// MetaInf
pub trait MetaInf {
  /// MetaID
  fn id(&self) -> MetaId;
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial;
  /// as MetaSphere
  fn as_sphere(&self) -> Option<&MetaSphere> { None }
  /// as MetaPlane
  fn as_plane(&self) -> Option<&MetaPlane> { None }
}

/// MetaSphere
pub struct MetaSphere {
  /// mass
  pub m: dReal,
  /// radius
  pub r: dReal,
  /// material
  pub tcm: TCMaterial
}

impl MetaSphere {
  /// construct
  pub fn new(m: dReal, r: dReal, tex: i32, col: dVector4) -> MetaSphere {
    MetaSphere{m: m, r: r, tcm: TCMaterial::new(tex, col)}
  }
}

impl MetaInf for MetaSphere {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Sphere }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaSphere
  fn as_sphere(&self) -> Option<&MetaSphere> { Some(self) }
}

/// MetaPlane
pub struct MetaPlane {
  /// dens mass
  pub dm: dReal,
  /// lxyz
  pub lxyz: dVector3,
  /// norm
  pub norm: dVector4,
  /// material
  pub tcm: TCMaterial
}

impl MetaPlane {
  /// construct
  pub fn new(dm: dReal, lxyz: dVector3, norm: dVector4,
    tex: i32, col: dVector4) -> MetaPlane {
    MetaPlane{dm: dm, lxyz: lxyz, norm: norm, tcm: TCMaterial::new(tex, col)}
  }
}

impl MetaInf for MetaPlane {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Plane }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaPlane
  fn as_plane(&self) -> Option<&MetaPlane> { Some(self) }
}
