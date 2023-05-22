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

// #[macro_export]
macro_rules! meta_panic {
  ($s: ident, $e: expr) => { panic!("Expected {} but {:?}", $e, $s.id()); };
}
// pub use meta_panic;

/// MetaInf
pub trait MetaInf {
  /// MetaID
  fn id(&self) -> MetaId;
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial;
  /// as MetaSphere
  fn as_sphere(&self) -> &MetaSphere { meta_panic!(self, "Sphere"); }
  /// as MetaBox
  fn as_box(&self) -> &MetaBox { meta_panic!(self, "Box"); }
  /// as MetaCapsule
  fn as_capsule(&self) -> &MetaCapsule { meta_panic!(self, "Capsule"); }
  /// as MetaCylinder
  fn as_cylinder(&self) -> &MetaCylinder { meta_panic!(self, "Cylinder"); }
  /// as MetaPlane
  fn as_plane(&self) -> &MetaPlane { meta_panic!(self, "Plane"); }
}

/// MetaSphere
pub struct MetaSphere {
  /// mass
  pub m: dReal,
  /// radius
  pub r: dReal,
  /// bounce
  pub bounce: dReal,
  /// material
  pub tcm: TCMaterial
}

impl MetaSphere {
  /// construct
  pub fn new(m: dReal, r: dReal,
    bounce: dReal, tex: i32, col: dVector4) -> MetaSphere {
    MetaSphere{m: m, r: r, bounce: bounce, tcm: TCMaterial::new(tex, col)}
  }
}

impl MetaInf for MetaSphere {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Sphere }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaSphere
  fn as_sphere(&self) -> &MetaSphere { self }
}

/// MetaBox
pub struct MetaBox {
  /// dens mass
  pub dm: dReal,
  /// lxyz
  pub lxyz: dVector3,
  /// bounce
  pub bounce: dReal,
  /// material
  pub tcm: TCMaterial
}

impl MetaBox {
  /// construct
  pub fn new(dm: dReal, lxyz: dVector3,
    bounce: dReal, tex: i32, col: dVector4) -> MetaBox {
    MetaBox{dm: dm, lxyz: lxyz,
      bounce: bounce, tcm: TCMaterial::new(tex, col)}
  }
}

impl MetaInf for MetaBox {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Box }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaBox
  fn as_box(&self) -> &MetaBox { self }
}

/// MetaCapsule
pub struct MetaCapsule {
  /// dens mass
  pub dm: dReal,
  /// radius
  pub r: dReal,
  /// length
  pub l: dReal,
  /// bounce
  pub bounce: dReal,
  /// material
  pub tcm: TCMaterial
}

impl MetaCapsule {
  /// construct
  pub fn new(dm: dReal, r: dReal, l: dReal,
    bounce: dReal, tex: i32, col: dVector4) -> MetaCapsule {
    MetaCapsule{dm: dm, r: r, l: l,
      bounce: bounce, tcm: TCMaterial::new(tex, col)}
  }
}

impl MetaInf for MetaCapsule {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Capsule }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaCapsule
  fn as_capsule(&self) -> &MetaCapsule { self }
}

/// MetaCylinder
pub struct MetaCylinder {
  /// dens mass
  pub dm: dReal,
  /// radius
  pub r: dReal,
  /// length
  pub l: dReal,
  /// bounce
  pub bounce: dReal,
  /// material
  pub tcm: TCMaterial
}

impl MetaCylinder {
  /// construct
  pub fn new(dm: dReal, r: dReal, l: dReal,
    bounce: dReal, tex: i32, col: dVector4) -> MetaCylinder {
    MetaCylinder{dm: dm, r: r, l: l,
      bounce: bounce, tcm: TCMaterial::new(tex, col)}
  }
}

impl MetaInf for MetaCylinder {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Cylinder }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaCylinder
  fn as_cylinder(&self) -> &MetaCylinder { self }
}

/// MetaPlane
pub struct MetaPlane {
  /// dens mass
  pub dm: dReal,
  /// lxyz
  pub lxyz: dVector3,
  /// norm
  pub norm: dVector4,
  /// bounce
  pub bounce: dReal,
  /// material
  pub tcm: TCMaterial
}

impl MetaPlane {
  /// construct
  pub fn new(dm: dReal, lxyz: dVector3, norm: dVector4,
    bounce: dReal, tex: i32, col: dVector4) -> MetaPlane {
    MetaPlane{dm: dm, lxyz: lxyz, norm: norm,
      bounce: bounce, tcm: TCMaterial::new(tex, col)}
  }
}

impl MetaInf for MetaPlane {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Plane }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaPlane
  fn as_plane(&self) -> &MetaPlane { self }
}
