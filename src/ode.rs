/*
  module oyk::ode

  cc-rs https://crates.io/crates/cc
  bindgen https://crates.io/crates/bindgen

  cc-rs
   include/bridge.hpp
   src/bridge.cpp

  bindgen
   from
    include/bridge.hpp
    ode/drawstuff.h (from modified preprocess -E dum.cpp includes drawstuff.h)
    ode/ode.hpp (from modified preprocess -E dum.cpp includes ode.h)
   to
    include/bridge_bindings.rs
    ode/drawstuff_bindings.rs
    ode/ode_bindings.rs

  in the running directory
    drawstuff.dll
    ode.dll
    libstdc++-6.dll
    libgcc_s_seh-1.dll
    libwinpthread-1.dll
*/

#![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod cppbridge;
use cppbridge::*;
pub use cppbridge::{Bridge};

mod cdrawstuff;
use cdrawstuff::*;
pub use cdrawstuff::{DS_VERSION, dsFunctions, dsSimulationLoop};

mod cppode;
use cppode::*;
pub use cppode::{dMatrix4, dMatrix3, dVector4, dVector3, dReal}; // 16 12 4 4

#[warn(unused)]
// #[warn(unused_imports)]
// #[warn(unused_attributes)]
#[warn(non_snake_case)]
#[warn(non_camel_case_types)]
#[warn(non_upper_case_globals)]

// std::any::type_name_of_val https://github.com/rust-lang/rust/issues/66359
fn fake_type_name_of_val<T>(_: &T) -> &'static str {
  std::any::type_name::<T>()
}

pub enum ClsId {
  World = 1, Space, Body, Geom, JointGroup
}

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! ostat {
  // ($($e:expr),+) => { print!($($e),*); };
  ($($e:expr),+) => {};
}
pub use ostat;

#[macro_export]
macro_rules! ostatln {
  // ($($e:expr),+) => { println!($($e),*); };
  ($($e:expr),+) => {};
}
pub use ostatln;

use std::ffi::{c_void};

use once_cell::sync::Lazy;

// unsafe static mut
pub static mut OYK_MUT: Lazy<Vec<ODE>> = Lazy::new(|| vec![ODE::new(0.002)]);

pub static COLORS: Lazy<Vec<u32>> = Lazy::new(|| vec![
  0xcccccccc, 0xcc9933cc, 0x33cc99cc, 0x9933cccc,
  0x3399cccc, 0x99cc33cc, 0xcc3399cc, 0x999999cc,
  0x666666cc, 0x996633cc, 0x339966cc, 0x663399cc,
  0x336699cc, 0x669933cc, 0x993366cc, 0x333333cc]);

pub fn vec4_from_u32(col: u32) -> dVector4 {
  let mut c: dVector4 = [0.0, 0.0, 0.0, 0.0];
  let p: usize = &col as *const u32 as usize;
  for j in 0..4 {
unsafe {
    c[j] = *((p + (3 - j)) as *const u8) as dReal / 255.0; // little endian
}
  }
  c
}

impl dSurfaceParameters {

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

pub fn new() -> dContactGeom {
  dContactGeom{
    pos: [0.0, 0.0, 0.0, 0.0], // dVector3
    normal: [0.0, 0.0, 0.0, 0.0], // dVector3
    depth: 0.0, // dReal
    g1: 0 as dGeomID,
    g2: 0 as dGeomID,
    side1: 0, // c_int
    side2: 0} // c_int
}

}

impl dContact {

pub fn new() -> dContact {
  dContact{
    surface: dSurfaceParameters::new(),
    geom: dContactGeom::new(),
    fdir1: [0.0, 0.0, 0.0, 0.0]} // dVector3
}

}

impl dMass {

pub fn new() -> dMass {
  let mut mass: dMass = dMass{
    mass: 0.0,
    c: [0.0, 0.0, 0.0, 0.0], // dVector3
    I: [ // dMatrix3
      0.0, 0.0, 0.0, 0.0,
      0.0, 0.0, 0.0, 0.0,
      0.0, 0.0, 0.0, 0.0]};
  unsafe { dMassSetZero(&mut mass); } // may be needless
  mass
}

}

pub struct Obg { // unsafe *mut xxx
  body: usize, // dBodyID,
  geom: usize, // dGeomID,
  pub col: dVector4
}

impl Obg {

pub fn new(body: dBodyID, geom: dGeomID, col: &dVector4) -> Obg {
  Obg{body: body as usize, geom: geom as usize, col: *col}
}

pub fn body_(&mut self, id: dBodyID) { from_id!(obg: self, id); }
pub fn body(&self) -> dBodyID { as_id!(self, body) }
pub fn geom_(&mut self, id: dGeomID) { from_id!(obg: self, id); }
pub fn geom(&self) -> dGeomID { as_id!(self, geom) }

pub fn mk_sphere(gws: &mut Gws, m: dReal, r: dReal, col: &dVector4, pos: &dVector3) -> Obg {
  let mut mass: dMass = dMass::new();
unsafe {
  dMassSetSphereTotal(&mut mass, m, r);
  let body: dBodyID = dBodyCreate(gws.world());
  dBodySetMass(body, &mass);
  let geom: dGeomID = dCreateSphere(gws.space(), r);
  dGeomSetBody(geom, body);
  dBodySetPosition(body, pos[0], pos[1], pos[2]);
  Obg::new(body, geom, col) // in unsafe {}, otherwise ambiguous Self body geom
}
}
}

pub struct Gws { // unsafe *mut xxx
  world: usize, // dWorldID,
  space: usize, // dSpaceID,
  ground: usize, // dGeomID,
  contactgroup: usize // dJointGroupID
}

impl Gws {

pub fn new() -> Gws {
  Gws{world: 0, space: 0, ground: 0, contactgroup: 0}
}

pub fn world_(&mut self, id: dWorldID) { from_id!(gws: self, id); }
pub fn world(&self) -> dWorldID { as_id!(self, world) }
pub fn space_(&mut self, id: dSpaceID) { from_id!(gws: self, id); }
pub fn space(&self) -> dSpaceID { as_id!(self, space) }
pub fn ground_(&mut self, id: dGeomID) { from_id!(gws: self, id); }
pub fn ground(&self) -> dGeomID { as_id!(self, ground) }
pub fn contactgroup_(&mut self, id: dJointGroupID) { from_id!(gws: self, id); }
pub fn contactgroup(&self) -> dJointGroupID { as_id!(self, contactgroup) }

}

// pub const ObgLen: usize = std::mem::size_of::<Obg>(); // 48
// pub const GwsLen: usize = std::mem::size_of::<Gws>(); // 32

pub struct ODE { // unsafe
  pub obgs: Vec<Obg>,
  pub gws: Gws,
  pub t_delta: dReal
}

#[macro_export]
macro_rules! ode_get_mut {
  ($src: ident) => { (&mut OYK_MUT)[0].$src };
}
pub use ode_get_mut;

#[macro_export]
macro_rules! ode_get {
  ($src: ident) => { (&OYK_MUT)[0].$src };
}
pub use ode_get;

#[macro_export]
macro_rules! ode_gws_set {
  ($dst: ident, $src: expr) => {
//  unsafe { (&mut OYK_MUT)[0].gws.$dst = $src as usize } // use outside unsafe
    (&mut OYK_MUT)[0].gws.$dst = $src as usize
  };
}
// pub use ode_gws_set;

#[macro_export]
macro_rules! ode_gws_get {
  ($src: ident, $dst: ty) => {
//    unsafe { (&OYK_MUT)[0].gws.$src as $dst } // use outside unsafe
    (&OYK_MUT)[0].gws.$src as $dst
  };
}
// pub use ode_gws_get;

#[macro_export]
macro_rules! gws_dump {
  () => {
unsafe {
    let gws: &mut Gws = &mut ode_get_mut!(gws);
/*
    ostatln!("0x{:016x}", gws.world);
    ostatln!("0x{:016x}", gws.space);
    ostatln!("0x{:016x}", gws.ground);
    ostatln!("0x{:016x}", gws.contactgroup);
*/
    ostatln!("{:018p}", gws.world());
    ostatln!("{:018p}", gws.space());
    ostatln!("{:018p}", gws.ground());
    ostatln!("{:018p}", gws.contactgroup());
}
    ()
  };
}
// pub use gws_dump;

impl ODE {

pub fn new(delta: dReal) -> ODE {
  ostatln!("new ODE");
  unsafe { dInitODE2(0); }
  ODE{obgs: vec![], gws: Gws::new(), t_delta: delta}
}

pub fn open() {
unsafe {
  // need this code (do nothing) to create instance
  let gws: &mut Gws = &mut ode_get_mut!(gws);
  gws.world = 0; // force call new before create_world
/*
  // need this code (do nothing) to create instance
  let v = &mut OYK_MUT;
  v[0].gws.world = 0; // force call new before create_world
  drop(v);
*/
}
  ODE::create_world();
//  gws_dump!();
}

pub fn close() {
  ODE::clear_obgs();
  ODE::destroy_world();
unsafe {
  // need this code (drop element) to drop instance
  let v = &mut OYK_MUT;
  drop(&v[0]); // need it otherwise never called destructor
  v.pop();
  drop(v);
}
}

pub fn create_world() {
  ostatln!("create world");
unsafe {
  let gws: &mut Gws = &mut ode_get_mut!(gws);
  gws.world_(dWorldCreate());
  dWorldSetGravity(gws.world(), 0.0, 0.0, -9.8);
  gws.space_(dHashSpaceCreate(0 as dSpaceID));
  dSpaceSetCleanup(gws.space(), 1);
  gws.ground_(dCreatePlane(gws.space(), 0.0, 0.0, 1.0, 0.0));
  gws.contactgroup_(dJointGroupCreate(0));
}
}

pub fn destroy_world() {
  ostatln!("destroy world");
unsafe {
  let gws: &mut Gws = &mut ode_get_mut!(gws);
  // dGeomDestroy(gws.ground());
  dSpaceDestroy(gws.space());
  dWorldDestroy(gws.world());
}
}

pub fn destroy_obg(obg: &Obg) {
unsafe {
  dGeomDestroy(obg.geom());
  dBodyDestroy(obg.body());
}
}

pub fn clear_obgs() {
unsafe {
  let obgs: &mut Vec<Obg> = &mut ode_get_mut!(obgs);
  for obg in &mut *obgs {
    ODE::destroy_obg(obg); // not obgs.pop();
  }
  obgs.clear();
}
}

}

impl Drop for ODE {
  fn drop(&mut self) {
    unsafe { dCloseODE(); }
    ostatln!("dropped ODE");
  }
}

pub unsafe extern "C"
fn start_callback() {
  let xyz: &mut [f32] = &mut vec![4.0, 3.0, 5.0];
  let hpr: &mut [f32] = &mut vec![-150.0, -30.0, 3.0];
  dsSetViewpoint(xyz as *mut [f32] as *mut f32, hpr as *mut [f32] as *mut f32);
}

pub unsafe extern "C"
fn near_callback(_dat: *mut c_void, o1: dGeomID, o2: dGeomID) {
  let gws: &mut Gws = &mut ode_get_mut!(gws);
  // if !(gws.ground() == o1 || gws.ground() == o2) { return; }
  const num: usize = 40;
  let contacts: &mut Vec<dContact> = &mut vec![dContact::new(); num];
  let sz: i32 = std::mem::size_of::<dContact>() as i32;
  let n: i32 = dCollide(o1, o2, num as i32, &mut contacts[0].geom, sz);
  for i in 0..n as usize {
    let cntct: &mut dContact = &mut contacts[i];
    cntct.surface.mu = dInfinity;
    cntct.surface.mode = dContactBounce;
    cntct.surface.bounce = 0.95; // 0.0
    cntct.surface.bounce_vel = 0.0;
    let c: dJointID = dJointCreateContact(
      gws.world(), gws.contactgroup(), cntct);
    dJointAttach(c, dGeomGetBody(cntct.geom.g1), dGeomGetBody(cntct.geom.g2));
  }
}

pub unsafe extern "C"
fn step_callback(pause: i32) {
  let obgs: &mut Vec<Obg> = &mut ode_get_mut!(obgs);
  let gws: &mut Gws = &mut ode_get_mut!(gws);
  if pause != 1 {
    dSpaceCollide(gws.space(), 0 as *mut c_void, Some(near_callback));
    dWorldStep(gws.world(), ode_get!(t_delta));
    dJointGroupEmpty(gws.contactgroup());
  }
  for obg in obgs {
    let c: Vec<f32> = obg.col.into_iter().map(|v| v as f32).collect();
    dsSetColorAlpha(c[0], c[1], c[2], c[3]);
    let geom: dGeomID = obg.geom();
    let body: dBodyID = dGeomGetBody(geom); // same as obg.body()
    let cls = dGeomGetClass(geom);
    match cls {
      dSphereClass => {
        let pos: *const dReal = dBodyGetPosition(body);
        let rot: *const dReal = dBodyGetRotation(body);
        let radius: dReal = dGeomSphereGetRadius(geom);
        dsDrawSphereD(pos, rot, radius as f32);
      },
      _ => { println!("unknown class: {}", cls); }
    }
  }
}
