//! cls
//!

use crate::ode::*;
use crate::ode::mat::*;
use crate::ode::prim::*;
use crate::ode::meta::*;

// std::any::type_name_of_val https://github.com/rust-lang/rust/issues/66359
fn fake_type_name_of_val<T>(_: &T) -> &'static str {
  std::any::type_name::<T>()
}

// id for from_id and chk_src_type (now inner use)
// not pub
enum ClsId {
  World = 1, Space, Body, Geom, JointGroup
}

// #[macro_export]
macro_rules! chk_src_type {
  ($n: expr, $obj: ident, $src: expr) => {{
    let o = fake_type_name_of_val(&$obj); // "*mut (self name)::oyk::ode::Xxx"
    let t = fake_type_name_of_val(&$src);
    let v = $src as usize;
    let mut r = (None, v);
    if &t[..5] == "*mut " { // dTypeID as "*mut (self name)::oyk::ode::dxType"
      let p: Vec<_> = t.match_indices("::").collect(); // [(pos, "::"), ...]
      if p.len() > 0 {
        let s = &t[p[p.len() - 1].0+4..]; // skip 4 bytes "::dx"
        // println!("{}", s);
        r = (match s {
          "World" => { Some(ClsId::World) },
          "Space" => { Some(ClsId::Space) },
          "Body" => { Some(ClsId::Body) },
          "Geom" => { Some(ClsId::Geom) },
          "JointGroup" => { Some(ClsId::JointGroup) },
          _ => { println!("unknown pattern of {} {}, {}", $n, o, t); None }
        }, v)
      }
    }
    r
  }};
}
// pub use chk_src_type;

// #[macro_export]
macro_rules! from_id {
  (obg: $obj: ident, $src: expr) => {
    let (k, v) = chk_src_type!("Obg", $obj, $src);
    match k {
      Some(ClsId::Body) => { $obj.body = v },
      Some(ClsId::Geom) => { $obj.geom = v },
      _ => {}
    }
  };
  (gws: $obj: ident, $src: expr) => {
    let (k, v) = chk_src_type!("Gws", $obj, $src);
    match k {
      Some(ClsId::World) => { $obj.world = v },
      Some(ClsId::Space) => { $obj.space = v },
      Some(ClsId::Geom) => { $obj.ground = v },
      Some(ClsId::JointGroup) => { $obj.contactgroup = v },
      _ => {}
    }
  };
  ($obj: ident, $src: expr, $dst: ident) => { $obj.$dst = $src as usize };
}
// pub use from_id;

// #[macro_export]
macro_rules! as_id { // common Obg and Gws
  ($obj: ident, body) => { $obj.body as dBodyID };
  ($obj: ident, geom) => { $obj.geom as dGeomID };
  ($obj: ident, world) => { $obj.world as dWorldID };
  ($obj: ident, space) => { $obj.space as dSpaceID };
  ($obj: ident, ground) => { $obj.ground as dGeomID };
  ($obj: ident, contactgroup) => { $obj.contactgroup as dJointGroupID };
  ($obj: ident, $src: ident, $dst: ty) => { $obj.$src as $dst };
}
// pub use as_id;

/// object(s) of ODE, obgs: HashMap&lt;dBodyID, Obg&gt;
pub struct Obg { // unsafe *mut xxx
  /// key
  pub key: String,
  body: usize, // dBodyID
  geom: usize, // dGeomID (handle only first geom)
  /// color (low priority obg.col &lt; tcm.col)
  pub col: dVector4
}

/// body geom
impl Obg {

/// construct
pub fn new(key: &str, body: dBodyID, geom: dGeomID, col: dVector4) -> Obg {
  Obg{key: key.to_string(), body: body as usize, geom: geom as usize, col: col}
}

/// setter
pub fn body_(&mut self, id: dBodyID) { from_id!(obg: self, id); }
/// getter
pub fn body(&self) -> dBodyID { as_id!(self, body) }
/// setter
pub fn geom_(&mut self, id: dGeomID) { from_id!(obg: self, id); }
/// getter
pub fn geom(&self) -> dGeomID { as_id!(self, geom) }

/// pos dVector3 as *mut [dReal; 4]
pub fn pos_ptr_mut(&mut self) -> *mut [dReal; 4] {
unsafe {
  let p: *mut dReal = dBodyGetPosition(self.body()) as *mut dReal;
  p as *mut [dReal; 4]
}
}

/// pos dVector3 as &mut [dReal] 4 usize
pub fn pos_(&mut self) -> &mut [dReal] {
unsafe {
  std::slice::from_raw_parts_mut(self.pos_ptr_mut() as *mut dReal, 4)
}
}

/// pos dVector3 as &[dReal] 4 usize
pub fn pos(&self) -> &[dReal] {
unsafe {
  let p: *const dReal = dBodyGetPosition(self.body());
  std::slice::from_raw_parts(p, 4)
}
}

/// pos dVector3 as ODEMat
pub fn pos_vec(&self) -> ODEMat {
  ODEMat::as_vec(self.pos())
}

/// rot dMatrix3 as *mut [[dReal; 4]; 3]
pub fn rot_ptr_mut(&mut self) -> *mut [[dReal; 4]; 3] {
unsafe {
  let p: *mut dReal = dBodyGetRotation(self.body()) as *mut dReal;
  p as *mut [[dReal; 4]; 3]
}
}

/// rot dMatrix3 as &mut [dReal] 12 usize
pub fn rot_(&mut self) -> &mut [dReal] {
unsafe {
  std::slice::from_raw_parts_mut(self.rot_ptr_mut() as *mut dReal, 12)
}
}

/// rot dMatrix3 as &[dReal] 12 usize
pub fn rot(&self) -> &[dReal] {
unsafe {
  let p: *const dReal = dBodyGetRotation(self.body());
  std::slice::from_raw_parts(p, 12)
}
}

/// rot dMatrix3 as ODEMat
pub fn rot_mat3(&self) -> ODEMat {
  ODEMat::as_mat(3, self.rot())
}

/// set position
pub fn set_pos(&mut self, pos: dVector3) -> &mut Obg {
unsafe {
  dBodySetPosition(self.body(), pos[0], pos[1], pos[2]);
}
  self
}

/// set rotation
pub fn set_rot(&mut self, rot: dMatrix3) -> &mut Obg {
unsafe {
  dBodySetRotation(self.body(), rot.as_ptr() as *mut dReal);
}
  self
}

/// set quaternion
pub fn set_quaternion(&mut self, q: dQuaternion) -> &mut Obg {
unsafe {
  dBodySetQuaternion(self.body(), q.as_ptr() as *mut dReal);
}
  self
}

}

/// object of ODE, gws: singleton
pub struct Gws { // unsafe *mut xxx
  world: usize, // dWorldID
  space: usize, // dSpaceID
  ground: usize, // dGeomID
  contactgroup: usize // dJointGroupID
}

/// world space
impl Gws {

/// construct
pub fn new() -> Gws {
  Gws{world: 0, space: 0, ground: 0, contactgroup: 0}
}

/// setter
pub fn world_(&mut self, id: dWorldID) { from_id!(gws: self, id); }
/// getter
pub fn world(&self) -> dWorldID { as_id!(self, world) }
/// setter
pub fn space_(&mut self, id: dSpaceID) { from_id!(gws: self, id); }
/// getter
pub fn space(&self) -> dSpaceID { as_id!(self, space) }
/// setter
pub fn ground_(&mut self, id: dGeomID) { from_id!(gws: self, id); }
/// getter
pub fn ground(&self) -> dGeomID { as_id!(self, ground) }
/// setter
pub fn contactgroup_(&mut self, id: dJointGroupID) { from_id!(gws: self, id); }
/// getter
pub fn contactgroup(&self) -> dJointGroupID { as_id!(self, contactgroup) }

}

// pub const GwsLen: usize = std::mem::size_of::<Gws>(); // 32

/// GeomOffset
pub struct GeomOffset<'a> {
  /// geom sub
  pub gsub: dGeomID,
  /// offset
  pub o: &'a dVector3
}

/// use for Vec<T>
pub trait AsPtr<T> {
  /// &amp;mut self[0] as *mut T
  fn as_ptr_mut(&mut self) -> *mut T;
  /// &amp;self[0] as *const T
  fn as_ptr(&self) -> *const T;
}

impl<T> AsPtr<T> for Vec<T> {
  /// &amp;mut self[0] as *mut T
  fn as_ptr_mut(&mut self) -> *mut T { &mut self[0] as *mut T }
  /// &amp;self[0] as *const T
  fn as_ptr(&self) -> *const T { &self[0] as *const T }
}

/// material(s) of ODE, tcms: HashMap&lt;dGeomID, TCMaterial&gt;
#[derive(Clone)]
pub struct TCMaterial {
  /// texture id
  pub tex: i32,
  /// color (high priority tcm.col &gt; obg.col)
  pub col: dVector4
}

/// materials
impl TCMaterial {

/// construct example let tcm = TCMaterial::new(0, [1.0, 0.0, 0.0, 0.8]);
pub fn new(t: i32, c: dVector4) -> TCMaterial {
  TCMaterial{tex: t, col: c}
}

}

/// viewpoint(s) of ODE, cams: BTreeMap&lt;usize, Cam&gt;
pub struct Cam {
  /// pos, look at [0, 0, 0]
  pub pos: Vec<f32>,
  /// yaw, pitch, roll
  pub ypr: Vec<f32>
}

/// viewpoint
impl Cam {

/// construct example let cam = Cam::new(vec![0.0f32; 3], vec![0.0f32; 3]);
pub fn new(p: Vec<f32>, y: Vec<f32>) -> Cam {
  Cam{pos: p, ypr: y}
}

}
