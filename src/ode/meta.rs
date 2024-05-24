//! meta
//!

use crate::ode::*;
use crate::ode::cls::*;
use crate::ode::krp::*;

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
  /// every struct has krp
  fn get_krp_mut(&mut self) -> &mut Krp;
  /// every struct has krp
  fn get_krp(&self) -> &Krp;
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
  /// as MetaConvex
  fn as_convex(&self) -> &MetaConvex { meta_panic!(self, "Convex"); }
  /// as MetaTriMesh
  fn as_trimesh(&self) -> &MetaTriMesh { meta_panic!(self, "TriMesh"); }
  /// as MetaComposite
  fn as_composite(&self) -> &MetaComposite { meta_panic!(self, "Composite"); }
  /// clone MetaInf
  fn dup(&self) -> Box<dyn MetaInf> { meta_panic!(self, "Clone"); }
}

/// MetaSphere
#[derive(Clone)]
pub struct MetaSphere {
  /// mass density
  pub dm: dReal,
  /// radius
  pub r: dReal,
  /// krp
  pub krp: Krp,
  /// material
  pub tcm: TCMaterial
}

impl MetaSphere {
  /// construct
  pub fn new(dm: dReal, r: dReal,
    krp: Krp, tex: i32, col: dVector4) -> Box<MetaSphere> {
    Box::new(MetaSphere{dm: dm, r: r,
      krp: krp, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaSphere {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Sphere }
  /// every struct has krp
  fn get_krp_mut(&mut self) -> &mut Krp { &mut self.krp }
  /// every struct has krp
  fn get_krp(&self) -> &Krp { &self.krp }
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
  /// mass density
  pub dm: dReal,
  /// lxyz
  pub lxyz: dVector3,
  /// krp
  pub krp: Krp,
  /// material
  pub tcm: TCMaterial
}

impl MetaBox {
  /// construct
  pub fn new(dm: dReal, lxyz: dVector3,
    krp: Krp, tex: i32, col: dVector4) -> Box<MetaBox> {
    Box::new(MetaBox{dm: dm, lxyz: lxyz,
      krp: krp, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaBox {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Box }
  /// every struct has krp
  fn get_krp_mut(&mut self) -> &mut Krp { &mut self.krp }
  /// every struct has krp
  fn get_krp(&self) -> &Krp { &self.krp }
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
  /// mass density
  pub dm: dReal,
  /// radius
  pub r: dReal,
  /// length
  pub l: dReal,
  /// krp
  pub krp: Krp,
  /// material
  pub tcm: TCMaterial
}

impl MetaCapsule {
  /// construct
  pub fn new(dm: dReal, r: dReal, l: dReal,
    krp: Krp, tex: i32, col: dVector4) -> Box<MetaCapsule> {
    Box::new(MetaCapsule{dm: dm, r: r, l: l,
      krp: krp, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaCapsule {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Capsule }
  /// every struct has krp
  fn get_krp_mut(&mut self) -> &mut Krp { &mut self.krp }
  /// every struct has krp
  fn get_krp(&self) -> &Krp { &self.krp }
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
  /// mass density
  pub dm: dReal,
  /// radius
  pub r: dReal,
  /// length
  pub l: dReal,
  /// krp
  pub krp: Krp,
  /// material
  pub tcm: TCMaterial
}

impl MetaCylinder {
  /// construct
  pub fn new(dm: dReal, r: dReal, l: dReal,
    krp: Krp, tex: i32, col: dVector4) -> Box<MetaCylinder> {
    Box::new(MetaCylinder{dm: dm, r: r, l: l,
      krp: krp, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaCylinder {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Cylinder }
  /// every struct has krp
  fn get_krp_mut(&mut self) -> &mut Krp { &mut self.krp }
  /// every struct has krp
  fn get_krp(&self) -> &Krp { &self.krp }
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
  /// mass density
  pub dm: dReal,
  /// lxyz
  pub lxyz: dVector3,
  /// norm
  pub norm: dVector4,
  /// krp
  pub krp: Krp,
  /// material
  pub tcm: TCMaterial
}

impl MetaPlane {
  /// construct
  pub fn new(dm: dReal, lxyz: dVector3, norm: dVector4,
    krp: Krp, tex: i32, col: dVector4) -> Box<MetaPlane> {
    Box::new(MetaPlane{dm: dm, lxyz: lxyz, norm: norm,
      krp: krp, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaPlane {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Plane }
  /// every struct has krp
  fn get_krp_mut(&mut self) -> &mut Krp { &mut self.krp }
  /// every struct has krp
  fn get_krp(&self) -> &Krp { &self.krp }
  /// every struct has tcm
  fn get_tcm_mut(&mut self) -> &mut TCMaterial { &mut self.tcm }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaPlane
  fn as_plane(&self) -> &MetaPlane { self }
  /// clone MetaInf
  fn dup(&self) -> Box<dyn MetaInf> { Box::new(self.clone()) }
}

/// MetaConvex
#[derive(Clone)]
pub struct MetaConvex {
  /// free flag
  pub ff: bool,
  /// mass density
  pub dm: dReal,
  /// convexfvp *** CAUTION how to clone ***
  pub fvp: *mut convexfvp,
  /// krp
  pub krp: Krp,
  /// material
  pub tcm: TCMaterial
}

impl MetaConvex {
  /// construct
  pub fn new(ff: bool, dm: dReal, fvp: *mut convexfvp,
    krp: Krp, tex: i32, col: dVector4) -> Box<MetaConvex> {
    Box::new(MetaConvex{ff: ff, dm: dm, fvp: fvp,
      krp: krp, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaConvex {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Convex }
  /// every struct has krp
  fn get_krp_mut(&mut self) -> &mut Krp { &mut self.krp }
  /// every struct has krp
  fn get_krp(&self) -> &Krp { &self.krp }
  /// every struct has tcm
  fn get_tcm_mut(&mut self) -> &mut TCMaterial { &mut self.tcm }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaConvex
  fn as_convex(&self) -> &MetaConvex { self }
  /// clone MetaInf
  fn dup(&self) -> Box<dyn MetaInf> { Box::new(self.clone()) }
}

impl Drop for MetaConvex {
  fn drop(&mut self) {
    if self.ff && self.fvp != 0 as *mut convexfvp {
unsafe {
      FreeConvexFVP(self.fvp, self.ff);
}
    }
  }
}

/// MetaTriMesh
#[derive(Clone)]
pub struct MetaTriMesh {
  /// free flag
  pub ff: bool,
  /// mass density
  pub dm: dReal,
  /// trimeshvi *** CAUTION how to clone ***
  pub tmv: *mut trimeshvi,
  /// krp
  pub krp: Krp,
  /// material
  pub tcm: TCMaterial
}

impl MetaTriMesh {
  /// construct
  pub fn new(ff: bool, dm: dReal, tmv: *mut trimeshvi,
    krp: Krp, tex: i32, col: dVector4) -> Box<MetaTriMesh> {
    Box::new(MetaTriMesh{ff: ff, dm: dm, tmv: tmv,
      krp: krp, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaTriMesh {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::TriMesh }
  /// every struct has krp
  fn get_krp_mut(&mut self) -> &mut Krp { &mut self.krp }
  /// every struct has krp
  fn get_krp(&self) -> &Krp { &self.krp }
  /// every struct has tcm
  fn get_tcm_mut(&mut self) -> &mut TCMaterial { &mut self.tcm }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaTriMesh
  fn as_trimesh(&self) -> &MetaTriMesh { self }
  /// clone MetaInf
  fn dup(&self) -> Box<dyn MetaInf> { Box::new(self.clone()) }
}

impl Drop for MetaTriMesh {
  fn drop(&mut self) {
    if self.ff && self.tmv != 0 as *mut trimeshvi {
unsafe {
      FreeTriMeshVI(self.tmv, self.ff);
}
    }
  }
}

/// MetaComposite (not have dup #[derive(Clone)])
pub struct MetaComposite {
  /// elements
  pub elems: Vec<Box<dyn MetaInf>>,
  /// quaternions
  pub qs: Vec<dQuaternion>,
  /// offsets
  pub ofs: Vec<dVector3>,
  /// krp
  pub krp: Krp,
  /// material
  pub tcm: TCMaterial
}

impl MetaComposite {
  /// construct
  pub fn new(elems: Vec<Box<dyn MetaInf>>,
    qs: Vec<dQuaternion>, ofs: Vec<dVector3>,
    krp: Krp, tex: i32, col: dVector4) -> Box<MetaComposite> {
    Box::new(MetaComposite{elems: elems, ofs: ofs, qs: qs,
      krp: krp, tcm: TCMaterial::new(tex, col)})
  }
}

impl MetaInf for MetaComposite {
  /// MetaID
  fn id(&self) -> MetaId { MetaId::Composite }
  /// every struct has krp
  fn get_krp_mut(&mut self) -> &mut Krp { &mut self.krp }
  /// every struct has krp
  fn get_krp(&self) -> &Krp { &self.krp }
  /// every struct has tcm
  fn get_tcm_mut(&mut self) -> &mut TCMaterial { &mut self.tcm }
  /// every struct has tcm
  fn get_tcm(&self) -> &TCMaterial { &self.tcm }
  /// as MetaComposite
  fn as_composite(&self) -> &MetaComposite { self }
}
