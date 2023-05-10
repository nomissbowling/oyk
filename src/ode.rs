//! ode integrates bindings to cppbridge cdrawstuff and cppode
//!
//! - cc-rs https://crates.io/crates/cc
//! - bindgen https://crates.io/crates/bindgen
//!
//! # cc-rs
//!
//! - include/bridge.hpp
//! - src/bridge.cpp
//!
//! # bindgen
//!
//! from
//!
//!  - include/bridge.hpp
//!  - ode/drawstuff.h (from modified preprocess -E dum.cpp includes drawstuff.h)
//!  - ode/ode.hpp (from modified preprocess -E dum.cpp includes ode.h)
//!
//! to
//!
//!  - include/bridge_bindings.rs
//!  - ode/drawstuff_bindings.rs
//!  - ode/ode_bindings.rs
//!
//! # Requirements
//!
//! in the running directory
//!
//! - drawstuff.dll
//! - ode.dll
//! - libstdc++-6.dll
//! - libgcc_s_seh-1.dll
//! - libwinpthread-1.dll
//!

#![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod cppbridge;
use cppbridge::*;
pub use cppbridge::{Bridge, bput};

mod cdrawstuff;
use cdrawstuff::*;

mod cppode;
use cppode::*;
pub use cppode::{dBodyID, dGeomID};
pub use cppode::{dMatrix4, dMatrix3, dVector4, dVector3, dReal}; // 16 12 4 4

#[warn(unused)]
// #[warn(unused_imports)]
// #[warn(unused_attributes)]
#[warn(non_snake_case)]
#[warn(non_camel_case_types)]
#[warn(non_upper_case_globals)]

use asciiz::u8z::{U8zBuf, u8zz::{CArgsBuf}};

// std::any::type_name_of_val https://github.com/rust-lang/rust/issues/66359
fn fake_type_name_of_val<T>(_: &T) -> &'static str {
  std::any::type_name::<T>()
}

/// id for from_id and chk_src_type (now inner use)
pub enum ClsId {
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

/// for debug output status
#[macro_export]
macro_rules! ostat {
  // ($($e:expr),+) => { print!($($e),*); };
  ($($e:expr),+) => {};
}
pub use ostat;

/// for debug output status with ln
#[macro_export]
macro_rules! ostatln {
  // ($($e:expr),+) => { println!($($e),*); };
  ($($e:expr),+) => {};
}
pub use ostatln;

use std::ffi::{c_void};

use once_cell::sync::Lazy;

/// unsafe static mut OYK_MUT (management ODE singleton instance)
pub static mut OYK_MUT: Lazy<Vec<ODE>> = Lazy::new(|| vec![ODE::new(0.002)]);

/// unsafe static COLORS (reference to preset colors)
pub static COLORS: Lazy<Vec<u32>> = Lazy::new(|| vec![
  0xcccccccc, 0xcc9933cc, 0x33cc99cc, 0x9933cccc,
  0x3399cccc, 0x99cc33cc, 0xcc3399cc, 0x999999cc,
  0x666666cc, 0x996633cc, 0x339966cc, 0x663399cc,
  0x336699cc, 0x669933cc, 0x993366cc, 0x333333cc]);

/// u32 RGBA (little endian) to dVector4 color
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

/// binding construct dContact
pub fn new() -> dContact {
  dContact{
    surface: dSurfaceParameters::new(),
    geom: dContactGeom::new(),
    fdir1: [0.0, 0.0, 0.0, 0.0]} // dVector3
}

}

impl dMass {

/// binding construct dMass (as dMassSetZero)
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

/// object(s) of ODE, obgs: Vec&lt;Obg&gt;
pub struct Obg { // unsafe *mut xxx
  body: usize, // dBodyID,
  geom: usize, // dGeomID,
  pub col: dVector4 // color
}

impl Obg {

/// construct
pub fn new(body: dBodyID, geom: dGeomID, col: &dVector4) -> Obg {
  Obg{body: body as usize, geom: geom as usize, col: *col}
}

/// setter
pub fn body_(&mut self, id: dBodyID) { from_id!(obg: self, id); }
/// getter
pub fn body(&self) -> dBodyID { as_id!(self, body) }
/// setter
pub fn geom_(&mut self, id: dGeomID) { from_id!(obg: self, id); }
/// getter
pub fn geom(&self) -> dGeomID { as_id!(self, geom) }
}

/// object of ODE, gws: singleton
pub struct Gws { // unsafe *mut xxx
  world: usize, // dWorldID,
  space: usize, // dSpaceID,
  ground: usize, // dGeomID,
  contactgroup: usize // dJointGroupID
}

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

// pub const ObgLen: usize = std::mem::size_of::<Obg>(); // 48
// pub const GwsLen: usize = std::mem::size_of::<Gws>(); // 32

/// object of ODE, fns: singleton (registered callback functions, textures)
pub struct Fns {
  start: Option<fn(rode: &mut ODE)>,
  near: Option<fn(rode: &mut ODE, o1: dGeomID, o2: dGeomID)>,
  step: Option<fn(rode: &mut ODE, i32)>,
  command: Option<fn(rode: &mut ODE, i32)>,
  stop: Option<fn(rode: &mut ODE)>,
  path_to_textures: Option<U8zBuf>
}

impl Fns {

/// construct
pub fn new() -> Fns {
  Fns{start: None, near: None, step: None, command: None, stop: None,
    path_to_textures: None}
}

}

/// viewpoint(s) of ODE, cams: Vec&lt;Cam&gt;
pub struct Cam {
  pub pos: Vec<f32>, // pos, look at [0, 0, 0]
  pub ypr: Vec<f32> // yaw, pitch, roll
}

impl Cam {

/// construct example let cam: Cam = new(vec![0.0f32; 3], vec![0.0f32; 3]);
pub fn new(p: Vec<f32>, y: Vec<f32>) -> Cam {
  Cam{pos: p, ypr: y}
}

}

/// ODE singleton
pub struct ODE { // unsafe
  fns: Fns, // registered callback functions
  wire_solid: i32, // 0: wireframe, 1: solid (for bunny)
  polyfill_wireframe: i32, // 0: solid, 1: wireframe (for all)
  sw_viewpoint: usize, // switch viewpoint
  pub cams: Vec<Cam>, // viewpoint(s)
  pub obgs: Vec<Obg>, // object(s)
  pub gws: Gws, // singleton
  pub t_delta: dReal // step
}

/// $rf is registered function in Fns, $df is default callback used when None
#[macro_export]
macro_rules! ode_fn {
  ($rf: ident, $df: ident) => {
unsafe {
    let fns: &Fns = &ode_get!(fns);
    match fns.$rf.as_ref() {
      Some(f) => *f,
      None => $df
    }
}
  };
}
// pub use ode_fn;

/// ODE singleton getter (always mut)
#[macro_export]
macro_rules! ode_mut {
  () => { (&mut OYK_MUT)[0] };
}
// pub use ode_mut;

/// ODE attribute getter (mut)
#[macro_export]
macro_rules! ode_get_mut {
  ($src: ident) => { (&mut OYK_MUT)[0].$src };
}
// pub use ode_get_mut;

/// ODE attribute getter (not mut)
#[macro_export]
macro_rules! ode_get {
  ($src: ident) => { (&OYK_MUT)[0].$src };
}
// pub use ode_get;

// #[macro_export]
macro_rules! ode_gws_set {
  ($dst: ident, $src: expr) => {
//  unsafe { (&mut OYK_MUT)[0].gws.$dst = $src as usize } // use outside unsafe
    (&mut OYK_MUT)[0].gws.$dst = $src as usize
  };
}
// pub use ode_gws_set;

// #[macro_export]
macro_rules! ode_gws_get {
  ($src: ident, $dst: ty) => {
//    unsafe { (&OYK_MUT)[0].gws.$src as $dst } // use outside unsafe
    (&OYK_MUT)[0].gws.$src as $dst
  };
}
// pub use ode_gws_get;

// #[macro_export]
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

/// construct (must not call it, auto instanciate by once_cell lazy)
pub fn new(delta: dReal) -> ODE {
  ostatln!("new ODE");
  unsafe { dInitODE2(0); }
  ODE{fns: Fns::new(), wire_solid: 1, polyfill_wireframe: 0, sw_viewpoint: 0,
    cams: vec![
      Cam::new(vec![5.0, 0.0, 2.0], vec![-180.0, 0.0, 0.0]),
      Cam::new(vec![5.36, 2.02, 4.28], vec![-162.0, -31.0, 0.0]),
      Cam::new(vec![-8.3, -14.1, 3.1], vec![84.5, 1.0, 0.0]),
      Cam::new(vec![4.0, 3.0, 5.0], vec![-150.0, -30.0, 3.0]),
      Cam::new(vec![10.0, 10.0, 5.0], vec![-150.0, 0.0, 3.0]),
      Cam::new(vec![5.0, 0.0, 1.0], vec![-180.0, 0.0, 0.0])],
    obgs: vec![], gws: Gws::new(), t_delta: delta}
}

/// ODE initialize
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

/// ODE finalize
pub fn close() {
  ODE::clear_obgs();
  ODE::clear_contactgroup();
  ODE::destroy_world();
unsafe {
  // need this code (drop element) to drop instance
  let v = &mut OYK_MUT;
  drop(&v[0]); // need it otherwise never called destructor
  v.pop();
  drop(v);
}
}

/// auto called by ODE::open() (custom start callback to create your objects)
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

/// auto called by ODE::close()
pub fn destroy_world() {
  ostatln!("destroy world");
unsafe {
  let gws: &mut Gws = &mut ode_get_mut!(gws);
  // dGeomDestroy(gws.ground());
  dSpaceDestroy(gws.space());
  dWorldDestroy(gws.world());
}
}

/// make sphere primitive object (need register to show on the ODE space world)
pub fn mk_sphere(m: dReal, r: dReal, col: &dVector4, pos: &dVector3) -> Obg {
  let mut mass: dMass = dMass::new();
unsafe {
  let gws: &mut Gws = &mut ode_get_mut!(gws);
  dMassSetSphereTotal(&mut mass, m, r);
  let body: dBodyID = dBodyCreate(gws.world());
  dBodySetMass(body, &mass);
  let geom: dGeomID = dCreateSphere(gws.space(), r);
  dGeomSetBody(geom, body);
  dBodySetPosition(body, pos[0], pos[1], pos[2]);
  Obg::new(body, geom, col) // in unsafe {}, otherwise ambiguous Self body geom
}
}

/// destroy object (not unregister)
pub fn destroy_obg(obg: &Obg) {
unsafe {
  dGeomDestroy(obg.geom());
  dBodyDestroy(obg.body());
}
}

/// destroy and unregister all objects
pub fn clear_obgs() {
unsafe {
  let obgs: &mut Vec<Obg> = &mut ode_get_mut!(obgs);
  for obg in &mut *obgs {
    ODE::destroy_obg(obg); // not obgs.pop();
  }
  obgs.clear();
}
}

/// destroy contact group and re initialize it
pub fn clear_contactgroup() {
unsafe {
  let gws: &mut Gws = &mut ode_get_mut!(gws);
  dJointGroupDestroy(gws.contactgroup());
  gws.contactgroup_(dJointGroupCreate(0));
}
}

/// set viewpoint (from the current viewpoint Cam[sw_viewpoint])
pub fn viewpoint_() {
unsafe {
  let sw_viewpoint: &usize = &ode_get!(sw_viewpoint);
  let cams: &mut Vec<Cam> = &mut ode_get_mut!(cams);
  let cam = &mut cams[*sw_viewpoint];
  let pos: &mut [f32] = &mut cam.pos;
  let ypr: &mut [f32] = &mut cam.ypr;
  dsSetViewpoint(pos as *mut [f32] as *mut f32, ypr as *mut [f32] as *mut f32);
}
}

/// get viewpoint (f: true, save to the current viewpoint Cam[sw_viewpoint])
pub fn viewpoint(f: bool) {
unsafe {
  let p: &mut [f32] = &mut vec![0.0; 4];
  let y: &mut [f32] = &mut vec![0.0; 4];
  dsGetViewpoint(p as *mut [f32] as *mut f32, y as *mut [f32] as *mut f32);
  let sw_viewpoint: &usize = &ode_get!(sw_viewpoint);
  println!("viewpoint {} {:?}, {:?}", *sw_viewpoint, p, y);
  match f {
    true => {
      let cams: &mut Vec<Cam> = &mut ode_get_mut!(cams);
      let cam = &mut cams[*sw_viewpoint];
      cam.pos = p.to_vec();
      cam.ypr = y.to_vec();
    },
    _ => {}
  }
}
}

/// default simulation loop
pub fn sim_loop(
  width: i32, height: i32,
  start_cb: Option<fn(rode: &mut ODE)>,
  near_cb: Option<fn(rode: &mut ODE, o1: dGeomID, o2: dGeomID)>,
  step_cb: Option<fn(rode: &mut ODE, pause: i32)>,
  command_cb: Option<fn(rode: &mut ODE, cmd: i32)>,
  stop_cb: Option<fn(rode: &mut ODE)>,
  a: &[u8]) {
unsafe {
  let fns: &mut Fns = &mut ode_get_mut!(fns);
  fns.start = start_cb;
  fns.near = near_cb;
  fns.step = step_cb;
  fns.command = command_cb;
  fns.stop = stop_cb;
  fns.path_to_textures = Some(U8zBuf::from_u8array(a)); // to keep lifetime
  let mut dsfn: dsFunctions = dsFunctions{
    version: DS_VERSION,
    start: Some(c_start_callback), // Option<unsafe extern "C" fn()>
    step: Some(c_step_callback), // Option<unsafe extern "C" fn(i32)>
    command: Some(c_command_callback), // Option<unsafe extern "C" fn(i32)>
    stop: Some(c_stop_callback), // Option<unsafe extern "C" fn()>
    path_to_textures: fns.path_to_textures.as_ref().expect("not init").as_i8p()
  };

  let mut cab: CArgsBuf = CArgsBuf::from(&std::env::args().collect());
  dsSimulationLoop(cab.as_argc(), cab.as_argv_ptr_mut(),
    width, height, &mut dsfn);
}
}

}

/// binding finalize ODE (auto called)
impl Drop for ODE {
  fn drop(&mut self) {
    unsafe { dCloseODE(); }
    ostatln!("dropped ODE");
  }
}

unsafe extern "C"
fn c_start_callback() {
  let rode: &mut ODE = &mut ode_mut!();
  ode_fn!(start, default_start_callback)(rode);
}

unsafe extern "C"
fn c_near_callback(_dat: *mut c_void, o1: dGeomID, o2: dGeomID) {
  let rode: &mut ODE = &mut ode_mut!();
  ode_fn!(near, default_near_callback)(rode, o1, o2);
}

unsafe extern "C"
fn c_step_callback(pause: i32) {
  let rode: &mut ODE = &mut ode_mut!();
  ode_fn!(step, default_step_callback)(rode, pause);
}

unsafe extern "C"
fn c_command_callback(cmd: i32) {
  let rode: &mut ODE = &mut ode_mut!();
  ode_fn!(command, default_command_callback)(rode, cmd);
}

unsafe extern "C"
fn c_stop_callback() {
  let rode: &mut ODE = &mut ode_mut!();
  ode_fn!(stop, default_stop_callback)(rode);
}

/// start default callback function
pub fn default_start_callback(rode: &mut ODE) {
  ostatln!("called default start");
  ODE::viewpoint_();
unsafe {
  dsSetSphereQuality(3); // default sphere 1
  dsSetCapsuleQuality(3); // default capsule 3
}
}

/// near default callback function
pub fn default_near_callback(rode: &mut ODE, o1: dGeomID, o2: dGeomID) {
  ostatln!("called default near");
  let gws = &rode.gws;
unsafe {
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
}

/// step default callback function
pub fn default_step_callback(rode: &mut ODE, pause: i32) {
  ostatln!("called default step");
  let gws = &rode.gws;
  let t_delta = &rode.t_delta;
  if pause != 1 {
unsafe {
    dSpaceCollide(gws.space(), 0 as *mut c_void, Some(c_near_callback));
    dWorldStep(gws.world(), *t_delta);
    dJointGroupEmpty(gws.contactgroup());
}
  }
  draw_objects(rode);
}

/// command default callback function
pub fn default_command_callback(rode: &mut ODE, cmd: i32) {
  ostatln!("called default command");
  match cmd as u8 as char {
    'p' => {
unsafe {
      let polyfill_wireframe: &mut i32 = &mut ode_get_mut!(polyfill_wireframe);
      *polyfill_wireframe = 1 - *polyfill_wireframe;
      dsSetDrawMode(*polyfill_wireframe);
}
    },
    'w' => {
unsafe {
      let wire_solid: &mut i32 = &mut ode_get_mut!(wire_solid);
      *wire_solid = 1 - *wire_solid;
}
    },
    'v' => {
      ODE::viewpoint(false);
    },
    's' => {
      ODE::viewpoint(true);
unsafe {
      let sw_viewpoint: &mut usize = &mut ode_get_mut!(sw_viewpoint);
      *sw_viewpoint = (*sw_viewpoint + 1) % rode.cams.len();
}
      ODE::viewpoint_();
      ODE::viewpoint(false);
    },
    'r' => {
      ODE::clear_obgs();
      ODE::clear_contactgroup();
      ode_fn!(start, default_start_callback)(rode);
    },
    _ => {}
  }
}

/// stop default callback function
pub fn default_stop_callback(rode: &mut ODE) {
  ostatln!("called default stop");
}

/// future implements drawing composite in this function
/// wire_solid false/true for bunny
pub fn draw_objects(rode: &mut ODE) {
  let _wire_solid = &rode.wire_solid; // for bunny
  let obgs = &rode.obgs;
  for obg in obgs {
unsafe {
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
}
