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
  Heightfield,
  /// ODE Composite
  Composite
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
  fn get_tcm_mut(&mut self) -> &mut TCMaterial;
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
  /// as MetaComposite
  fn as_composite(&self) -> &MetaComposite { meta_panic!(self, "Composite"); }
  /// clone MetaInf
  fn dup(&self) -> Box<dyn MetaInf> { meta_panic!(self, "Clone"); }
}

/// MetaSphere
#[derive(Clone)]
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
    bounce: dReal, tex: i32, col: dVector4) -> Box<MetaSphere> {
    Box::new(MetaSphere{m: m, r: r,
      bounce: bounce, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaSphere {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Sphere }
  /// every struct has tcm
  fn get_tcm_mut(&mut self) -> &mut TCMaterial { &mut self.tcm }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaSphere
  fn as_sphere(&self) -> &MetaSphere { self }
  /// clone MetaInf
  fn dup(&self) -> Box<dyn MetaInf> { Box::new(self.clone()) }
}

/// MetaBox
#[derive(Clone)]
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
    bounce: dReal, tex: i32, col: dVector4) -> Box<MetaBox> {
    Box::new(MetaBox{dm: dm, lxyz: lxyz,
      bounce: bounce, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaBox {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Box }
  /// every struct has tcm
  fn get_tcm_mut(&mut self) -> &mut TCMaterial { &mut self.tcm }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaBox
  fn as_box(&self) -> &MetaBox { self }
  /// clone MetaInf
  fn dup(&self) -> Box<dyn MetaInf> { Box::new(self.clone()) }
}

/// MetaCapsule
#[derive(Clone)]
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
    bounce: dReal, tex: i32, col: dVector4) -> Box<MetaCapsule> {
    Box::new(MetaCapsule{dm: dm, r: r, l: l,
      bounce: bounce, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaCapsule {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Capsule }
  /// every struct has tcm
  fn get_tcm_mut(&mut self) -> &mut TCMaterial { &mut self.tcm }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaCapsule
  fn as_capsule(&self) -> &MetaCapsule { self }
  /// clone MetaInf
  fn dup(&self) -> Box<dyn MetaInf> { Box::new(self.clone()) }
}

/// MetaCylinder
#[derive(Clone)]
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
    bounce: dReal, tex: i32, col: dVector4) -> Box<MetaCylinder> {
    Box::new(MetaCylinder{dm: dm, r: r, l: l,
      bounce: bounce, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaCylinder {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Cylinder }
  /// every struct has tcm
  fn get_tcm_mut(&mut self) -> &mut TCMaterial { &mut self.tcm }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaCylinder
  fn as_cylinder(&self) -> &MetaCylinder { self }
  /// clone MetaInf
  fn dup(&self) -> Box<dyn MetaInf> { Box::new(self.clone()) }
}

/// MetaPlane
#[derive(Clone)]
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
    bounce: dReal, tex: i32, col: dVector4) -> Box<MetaPlane> {
    Box::new(MetaPlane{dm: dm, lxyz: lxyz, norm: norm,
      bounce: bounce, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaPlane {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Plane }
  /// every struct has tcm
  fn get_tcm_mut(&mut self) -> &mut TCMaterial { &mut self.tcm }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaPlane
  fn as_plane(&self) -> &MetaPlane { self }
  /// clone MetaInf
  fn dup(&self) -> Box<dyn MetaInf> { Box::new(self.clone()) }
}

/// MetaComposite (not have dup #[derive(Clone)])
pub struct MetaComposite {
  /// elements
  pub elems: Vec<Box<dyn MetaInf>>,
  /// offsets
  pub ofs: Vec<dVector3>,
  /// quaternions
  pub qs: Vec<dQuaternion>,
  /// bounce
  pub bounce: dReal,
  /// material
  pub tcm: TCMaterial
}

impl MetaComposite {
  /// construct
  pub fn new(elems: Vec<Box<dyn MetaInf>>,
    ofs: Vec<dVector3>, qs: Vec<dQuaternion>,
    bounce: dReal, tex: i32, col: dVector4) -> Box<MetaComposite> {
    Box::new(MetaComposite{elems: elems, ofs: ofs, qs: qs,
      bounce: bounce, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaComposite {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Composite }
  /// every struct has tcm
  fn get_tcm_mut(&mut self) -> &mut TCMaterial { &mut self.tcm }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaComposite
  fn as_composite(&self) -> &MetaComposite { self }
}
