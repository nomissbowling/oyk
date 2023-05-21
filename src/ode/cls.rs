//! cls
//!

use crate::ode::*;
use crate::ode::mat::*;

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
  body: usize, // dBodyID,
  geom: usize, // dGeomID,
  /// color (low priority obg.col &lt; tcm.col)
  pub col: dVector4
}

/// body geom
impl Obg {

/// construct
pub fn new(key: String, body: dBodyID, geom: dGeomID, col: dVector4) -> Obg {
  Obg{key: key, body: body as usize, geom: geom as usize, col: col}
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
  dBodySetRotation(self.body(), &rot[0] as *const dReal as *mut dReal);
}
  self
}

/// set quaternion
pub fn set_quaternion(&mut self, q: dQuaternion) -> &mut Obg {
unsafe {
  dBodySetQuaternion(self.body(), &q[0] as *const dReal as *mut dReal);
}
  self
}

}

/// object of ODE, gws: singleton
pub struct Gws { // unsafe *mut xxx
  world: usize, // dWorldID,
  space: usize, // dSpaceID,
  ground: usize, // dGeomID,
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

/// material(s) of ODE, tcms: HashMap&lt;dGeomID, TCMaterial&gt;
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

impl dSurfaceParameters {

/// binding construct dSurfaceParameters
pub fn new() -> dSurfaceParameters {
  dSurfaceParameters{
    mode: 0, // c_int
    mu: 0.0, // dReal
    mu2: 0.0, // dReal
    rho: 0.0, // dReal
    rho2: 0.0, // dReal
    rhoN: 0.0, // dReal
    bounce: 0.0, // dReal
    bounce_vel: 0.0, // dReal
    soft_erp: 0.0, // dReal
    soft_cfm: 0.0, // dReal
    motion1: 0.0, // dReal
    motion2: 0.0, // dReal
    motionN: 0.0, // dReal
    slip1: 0.0, // dReal
    slip2: 0.0} // dReal
}

}

impl dContactGeom {

/// binding construct dContactGeom
pub fn new() -> dContactGeom {
  dContactGeom{
    pos: [0.0; 4], // dVector3
    normal: [0.0; 4], // dVector3
    depth: 0.0, // dReal
    g1: 0 as dGeomID,
    g2: 0 as dGeomID,
    side1: 0, // c_int
    side2: 0} // c_int
}

}

impl dContact {

/// binding construct dContact
pub fn new() -> dContact {
  dContact{
    surface: dSurfaceParameters::new(),
    geom: dContactGeom::new(),
    fdir1: [0.0; 4]} // dVector3
}

}

/// constructor and converter for primitive type
pub trait Quaternion {
  /// construct as Identity
  fn new() -> dQuaternion {
    let mut q: dQuaternion = [0.0; 4];
unsafe {
    dQSetIdentity(&mut q[0] as *mut dReal);
}
    q
  }

  /// converter
  fn from_R(m: dMatrix3) -> dQuaternion {
    let mut q: dQuaternion = [0.0; 4];
unsafe {
    dQfromR(&mut q[0] as *mut dReal, &m[0] as *const dReal as *mut dReal);
}
    q
  }

  /// converter
  fn to_R(q: dQuaternion) -> dMatrix3 {
    dMatrix3::from_Q(q)
  }

  /// constructor
  fn from_axis_and_angle(axis: [dReal; 3], angle: dReal) -> dQuaternion {
    let mut q: dQuaternion = [0.0; 4];
unsafe {
    dQFromAxisAndAngle(&mut q[0] as *mut dReal,
      axis[0], axis[1], axis[2], angle);
}
    q
  }
}
impl Quaternion for dQuaternion {}

/// constructor and converter for primitive type
pub trait Matrix3 {
  /// construct as Identity
  fn new() -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRSetIdentity(&mut m[0] as *mut dReal);
}
    m
  }

  /// converter
  fn from_Q(q: dQuaternion) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRfromQ(&mut m[0] as *mut dReal, &q[0] as *const dReal as *mut dReal);
}
    m
  }

  /// converter
  fn to_Q(m: dMatrix3) -> dQuaternion {
    dQuaternion::from_R(m)
  }

  /// constructor
  fn from_axis_and_angle(axis: [dReal; 3], angle: dReal) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRFromAxisAndAngle(&mut m[0] as *mut dReal,
      axis[0], axis[1], axis[2], angle);
}
    m
  }

  /// constructor
  fn from_euler_angles(phi: dReal, theta: dReal, psi: dReal) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRFromEulerAngles(&mut m[0] as *mut dReal, phi, theta, psi);
}
    m
  }

  /// constructor
  fn from_2_axes(e0: [dReal; 3], e1: [dReal; 3]) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRFrom2Axes(&mut m[0] as *mut dReal,
      e0[0], e0[1], e0[2], e1[0], e1[1], e1[2]);
}
    m
  }

  /// constructor
  fn from_z_axis(e: [dReal; 3]) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRFromZAxis(&mut m[0] as *mut dReal, e[0], e[1], e[2]);
}
    m
  }
}
impl Matrix3 for dMatrix3 {}

/// constructor and converter for primitive type
pub trait Matrix4 {
  /// construct as Identity
  fn new() -> dMatrix4 {
    let mut m: dMatrix4 = [0.0; 16];
    for i in 0..4 { m[i * 5] = 1.0; }
    m
  }
}
impl Matrix4 for dMatrix4 {}

impl dMass {

/// binding construct dMass (as dMassSetZero)
pub fn new() -> dMass {
  let mut mass: dMass = dMass{
    mass: 0.0,
    c: [0.0; 4], // dVector3
    I: [0.0; 12]}; // dMatrix3
  unsafe { dMassSetZero(&mut mass); } // may be needless
  mass
}

}
