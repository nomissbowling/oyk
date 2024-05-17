//! obg
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

/// get mass to mut dMass by id
pub fn get_mass_by_id(id: dBodyID, mass: &mut dMass) {
unsafe {
  dBodyGetMass(id, mass as *mut dMass)
}
}

/// get mass to mut dMass
pub fn get_mass(&self, mass: &mut dMass) {
  Obg::get_mass_by_id(self.body(), mass)
}

/// get linear vel dVector3 as mut by id
pub fn get_linear_vel_mut_by_id(id: dBodyID) -> dVector3 {
unsafe {
  *(dBodyGetLinearVel(id) as *mut dVector3)
}
}

/// linear vel dVector3 as *mut [dReal; 4]
pub fn linear_vel_ptr_mut(&mut self) -> *mut [dReal; 4] {
unsafe {
  let p: *mut dReal = dBodyGetLinearVel(self.body()) as *mut dReal;
  p as *mut [dReal; 4]
}
}

/// linear vel dVector3 as &mut [dReal] 4 usize
pub fn linear_vel_(&mut self) -> &mut [dReal] {
unsafe {
  std::slice::from_raw_parts_mut(self.linear_vel_ptr_mut() as *mut dReal, 4)
}
}

/// linear vel dVector3 as &[dReal] 4 usize
pub fn linear_vel(&self) -> &[dReal] {
unsafe {
  let p: *const dReal = dBodyGetLinearVel(self.body());
  std::slice::from_raw_parts(p, 4)
}
}

/// linear vel dVector3 as ODEMat
pub fn linear_vel_vec(&self) -> ODEMat {
  ODEMat::as_vec(self.linear_vel())
}

/// get angular vel dVector3 as mut by id
pub fn get_angular_vel_mut_by_id(id: dBodyID) -> dVector3 {
unsafe {
  *(dBodyGetAngularVel(id) as *mut dVector3)
}
}

/// angular vel dVector3 as *mut [dReal; 4]
pub fn angular_vel_ptr_mut(&mut self) -> *mut [dReal; 4] {
unsafe {
  let p: *mut dReal = dBodyGetAngularVel(self.body()) as *mut dReal;
  p as *mut [dReal; 4]
}
}

/// angular vel dVector3 as &mut [dReal] 4 usize
pub fn angular_vel_(&mut self) -> &mut [dReal] {
unsafe {
  std::slice::from_raw_parts_mut(self.angular_vel_ptr_mut() as *mut dReal, 4)
}
}

/// angular vel dVector3 as &[dReal] 4 usize
pub fn angular_vel(&self) -> &[dReal] {
unsafe {
  let p: *const dReal = dBodyGetAngularVel(self.body());
  std::slice::from_raw_parts(p, 4)
}
}

/// angular vel dVector3 as ODEMat
pub fn angular_vel_vec(&self) -> ODEMat {
  ODEMat::as_vec(self.angular_vel())
}

/// get pos dVector3 as mut by id
pub fn get_pos_mut_by_id(id: dBodyID) -> dVector3 {
unsafe {
  *(dBodyGetPosition(id) as *mut dVector3)
}
}

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

/// get quaternion dVector4 as mut by id
pub fn get_quaternion_mut_by_id(id: dBodyID) -> dVector4 {
unsafe {
  *(dBodyGetQuaternion(id) as *mut dVector4)
}
}

/// quaternion dQuaternion as *mut [dReal; 4]
pub fn quaternion_ptr_mut(&mut self) -> *mut [dReal; 4] {
unsafe {
  let p: *mut dReal = dBodyGetQuaternion(self.body()) as *mut dReal;
  p as *mut [dReal; 4]
}
}

/// quaternion dQuaternion as &mut [dReal] 4 usize
pub fn quaternion_(&mut self) -> &mut [dReal] {
unsafe {
  std::slice::from_raw_parts_mut(self.quaternion_ptr_mut() as *mut dReal, 4)
}
}

/// quaternion dQuaternion as &[dReal] 4 usize
pub fn quaternion(&self) -> &[dReal] {
unsafe {
  let p: *const dReal = dBodyGetQuaternion(self.body());
  std::slice::from_raw_parts(p, 4)
}
}

/// quaternion dQuaternion as ODEMat
pub fn quaternion_vec(&self) -> ODEMat {
  ODEMat::as_vec(self.quaternion())
}

/// get rot dMatrix3 as mut by id
pub fn get_rot_mut_by_id(id: dBodyID) -> dMatrix3 {
unsafe {
  *(dBodyGetRotation(id) as *mut dMatrix3)
}
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

/// set mass by id
pub fn set_mass_by_id(id: dBodyID, mass: &dMass) {
unsafe {
  dBodySetMass(id, mass as *const dMass)
}
}

/// set mass
pub fn set_mass(&mut self, mass: &dMass) -> &mut Obg {
  Obg::set_mass_by_id(self.body(), mass);
  self
}

/// set linear vel by id
pub fn set_linear_vel_by_id(id: dBodyID, xyz: &dVector3) {
unsafe {
  dBodySetLinearVel(id, xyz[0], xyz[1], xyz[2]);
}
}

/// set linear vel
pub fn set_linear_vel(&mut self, xyz: dVector3) -> &mut Obg {
  Obg::set_linear_vel_by_id(self.body(), &xyz);
  self
}

/// set angular vel by id
pub fn set_angular_vel_by_id(id: dBodyID, xyz: &dVector3) {
unsafe {
  dBodySetAngularVel(id, xyz[0], xyz[1], xyz[2]);
}
}

/// set angular vel
pub fn set_angular_vel(&mut self, xyz: dVector3) -> &mut Obg {
  Obg::set_angular_vel_by_id(self.body(), &xyz);
  self
}

/// set position by id
pub fn set_pos_by_id(id: dBodyID, pos: &dVector3) {
unsafe {
  dBodySetPosition(id, pos[0], pos[1], pos[2]);
}
}

/// set position
pub fn set_pos(&mut self, pos: dVector3) -> &mut Obg {
  Obg::set_pos_by_id(self.body(), &pos);
  self
}

/// set quaternion by id
pub fn set_quaternion_by_id(id: dBodyID, q: &dQuaternion) {
unsafe {
  dBodySetQuaternion(id, q.as_ptr() as *mut dReal);
}
}

/// set quaternion
pub fn set_quaternion(&mut self, q: dQuaternion) -> &mut Obg {
  Obg::set_quaternion_by_id(self.body(), &q);
  self
}

/// set rotation by id
pub fn set_rot_by_id(id: dBodyID, rot: &dMatrix3) {
unsafe {
  dBodySetRotation(id, rot.as_ptr() as *mut dReal);
}
}

/// set rotation
pub fn set_rot(&mut self, rot: dMatrix3) -> &mut Obg {
  Obg::set_rot_by_id(self.body(), &rot);
  self
}

/// set torque by id
pub fn set_torque_by_id(id: dBodyID, t: &[dReal; 3]) {
unsafe {
  dBodySetTorque(id, t[0], t[1], t[2]);
}
}

/// set torque
pub fn set_torque(&mut self, t: [dReal; 3]) -> &mut Obg {
  Obg::set_torque_by_id(self.body(), &t);
  self
}

/// add torque by id
pub fn add_torque_by_id(id: dBodyID, t: &[dReal; 3]) {
unsafe {
  dBodyAddTorque(id, t[0], t[1], t[2]);
}
}

/// add torque
pub fn add_torque(&mut self, t: [dReal; 3]) -> &mut Obg {
  Obg::add_torque_by_id(self.body(), &t);
  self
}

/// add rel torque by id
pub fn add_rel_torque_by_id(id: dBodyID, t: &[dReal; 3]) {
unsafe {
  dBodyAddRelTorque(id, t[0], t[1], t[2]);
}
}

/// add rel torque
pub fn add_rel_torque(&mut self, t: [dReal; 3]) -> &mut Obg {
  Obg::add_rel_torque_by_id(self.body(), &t);
  self
}

/// set force by id
pub fn set_force_by_id(id: dBodyID, f: &[dReal; 3]) {
unsafe {
  dBodySetForce(id, f[0], f[1], f[2]);
}
}

/// set force
pub fn set_force(&mut self, f: [dReal; 3]) -> &mut Obg {
  Obg::set_force_by_id(self.body(), &f);
  self
}

/// add force by id
pub fn add_force_by_id(id: dBodyID, f: &[dReal; 3]) {
unsafe {
  dBodyAddForce(id, f[0], f[1], f[2]);
}
}

/// add force
pub fn add_force(&mut self, f: [dReal; 3]) -> &mut Obg {
  Obg::add_force_by_id(self.body(), &f);
  self
}

/// add rel force by id
pub fn add_rel_force_by_id(id: dBodyID, f: &[dReal; 3]) {
unsafe {
  dBodyAddRelForce(id, f[0], f[1], f[2]);
}
}

/// add rel force
pub fn add_rel_force(&mut self, f: [dReal; 3]) -> &mut Obg {
  Obg::add_rel_force_by_id(self.body(), &f);
  self
}

/// add force at pos by id
pub fn add_force_at_by_id(id: dBodyID, f: &[dReal; 3], p: &[dReal; 3]) {
unsafe {
  dBodyAddForceAtPos(id, f[0], f[1], f[2], p[0], p[1], p[2]);
}
}

/// add force at pos
pub fn add_force_at(&mut self, f: [dReal; 3], p: [dReal; 3]) -> &mut Obg {
  Obg::add_force_at_by_id(self.body(), &f, &p);
  self
}

/// add rel force at pos by id
pub fn add_rel_force_at_by_id(id: dBodyID, f: &[dReal; 3], p: &[dReal; 3]) {
unsafe {
  dBodyAddRelForceAtPos(id, f[0], f[1], f[2], p[0], p[1], p[2]);
}
}

/// add rel force at pos
pub fn add_rel_force_at(&mut self, f: [dReal; 3], p: [dReal; 3]) -> &mut Obg {
  Obg::add_rel_force_at_by_id(self.body(), &f, &p);
  self
}

/// add force at rel pos by id
pub fn add_force_rel_by_id(id: dBodyID, f: &[dReal; 3], p: &[dReal; 3]) {
unsafe {
  dBodyAddForceAtRelPos(id, f[0], f[1], f[2], p[0], p[1], p[2]);
}
}

/// add force at rel pos
pub fn add_force_rel(&mut self, f: [dReal; 3], p: [dReal; 3]) -> &mut Obg {
  Obg::add_force_rel_by_id(self.body(), &f, &p);
  self
}

/// add rel force at rel pos by id
pub fn add_rel_force_rel_by_id(id: dBodyID, f: &[dReal; 3], p: &[dReal; 3]) {
unsafe {
  dBodyAddRelForceAtRelPos(id, f[0], f[1], f[2], p[0], p[1], p[2]);
}
}

/// add rel force at rel pos
pub fn add_rel_force_rel(&mut self, f: [dReal; 3], p: [dReal; 3]) -> &mut Obg {
  Obg::add_rel_force_rel_by_id(self.body(), &f, &p);
  self
}

/// is enabled
pub fn is_enabled(&self) -> bool {
unsafe {
  dBodyIsEnabled(self.body()) != 0
}
}

/// disable
pub fn disable(&mut self) {
unsafe {
  dBodyDisable(self.body());
}
}

/// enable
pub fn enable(&mut self) {
unsafe {
  dBodyEnable(self.body());
}
}

}

/// object of ODE, gws: singleton
pub struct Gws { // unsafe *mut xxx
  /// dWorldID
  world: usize,
  /// dSpaceID
  space: usize,
  /// dGeomID
  ground: usize,
  /// dJointGroupID
  contactgroup: usize,
  /// number of contacts vec dContact
  num_contact: usize,
  /// contacts
  pub contacts: Vec<dContact>
}

/// world space
impl Gws {

/// construct
pub fn new(num_contact: usize) -> Gws {
  let contacts = (0..num_contact).into_iter().map(|_|
    dContact::new()
  ).collect::<Vec<_>>();
  Gws{world: 0, space: 0, ground: 0, contactgroup: 0, num_contact, contacts}
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
/// setter
pub fn num_contact_(&mut self, nc: usize) { self.num_contact = nc; }
/// getter
pub fn num_contact(&self) -> usize { self.num_contact }

}

// pub const GwsLen: usize = std::mem::size_of::<Gws>(); // 32
