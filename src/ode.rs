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
pub use cppode::{dQuaternion};

#[warn(unused)]
// #[warn(unused_imports)]
// #[warn(unused_attributes)]
#[warn(non_snake_case)]
#[warn(non_camel_case_types)]
#[warn(non_upper_case_globals)]

use std::error::Error;

pub mod err;
use err::*;

pub mod mat;
use mat::*;

pub mod prim;
use prim::*;
pub use prim::{Matrix4, Matrix3, Quaternion};

pub mod meta;
use meta::*;
pub use meta::{MetaInf};
pub use meta::{MetaSphere, MetaBox, MetaCapsule, MetaCylinder, MetaPlane};

pub mod cls;
use cls::*;

use std::collections::hash_map::Entry;
use std::collections::HashMap; // with #[derive(PartialEq, Eq, Hash)] struct
use std::collections::btree_map::Entry as BTreeEntry;
use std::collections::BTreeMap;
use std::collections::VecDeque;

use asciiz::u8z::{U8zBuf, u8zz::{CArgsBuf}};

use std::ffi::{c_void};

pub extern crate impl_sim;
// pub use impl_sim::{impl_sim_fn, impl_sim_derive};

use once_cell::sync::Lazy;

/// unsafe static mut OYK_MUT (management ODE singleton instance)
pub static mut OYK_MUT: Lazy<Vec<ODE>> = Lazy::new(|| vec![ODE::new(0.002)]);

/// ODE singleton
pub struct ODE { // unsafe
  sim: Option<Box<dyn Sim>>, // trait Sim must have callback functions
  ptt: Option<U8zBuf>, // relative path to textures
  wire_solid: i32, // 0: wireframe, 1: solid (for bunny)
  polyfill_wireframe: i32, // 0: solid, 1: wireframe (for all)
  sw_viewpoint: usize, // switch viewpoint
  /// viewpoint(s)
  pub cams: BTreeMap<usize, Cam>,
  tcms: HashMap<dGeomID, TCMaterial>, // material(s) mapped
  obgs: HashMap<dBodyID, Obg>, // object(s) mapped (obg has key: String)
  mbgs: BTreeMap<String, dBodyID>, // object id(s) ordered mapped (key: cloned)
  vbgs: VecDeque<dBodyID>, // object id(s) ordered (drawing order)
  modified: bool, // modify() is_modified()
  gws: Gws, // singleton
  /// step
  pub t_delta: dReal
}

/// $rs is Sim instance, $rf is callback function in Sim
#[macro_export]
macro_rules! ode_sim {
  ($rs: ident, $rf: ident) => {
    match &mut $rs.sim {
      Some(s) => s.$rf(),
      None => $rs.$rf()
    }
  };
  ($rs: ident, $rf: ident, $($e:expr),+) => {
    match &mut $rs.sim {
      Some(s) => s.$rf($($e),+),
      None => $rs.$rf($($e),+)
    }
  };
}
// pub use ode_sim;

/// ODE singleton getter (mutable)
#[macro_export]
macro_rules! ode_mut {
  () => { (&mut OYK_MUT)[0] };
}
// pub use ode_mut;

/// ODE singleton getter (immutable)
#[macro_export]
macro_rules! ode_ {
  () => { (&OYK_MUT)[0] };
}
// pub use ode_;

/// ODE attribute getter (mutable)
#[macro_export]
macro_rules! ode_get_mut {
  ($src: ident) => { (&mut OYK_MUT)[0].$src };
}
// pub use ode_get_mut;

/// ODE attribute getter (immutable)
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

/// singleton interface
impl ODE {

/// construct (must not call it, auto instanciate by once_cell lazy)
pub fn new(delta: dReal) -> ODE {
  ostatln!("new ODE");
  unsafe { dInitODE2(0); }
  ODE{sim: None, ptt: None,
    wire_solid: 1, polyfill_wireframe: 0, sw_viewpoint: 0,
    cams: vec![
      Cam::new(vec![5.0, 0.0, 2.0], vec![-180.0, 0.0, 0.0]),
      Cam::new(vec![5.36, 2.02, 4.28], vec![-162.0, -31.0, 0.0]),
      Cam::new(vec![-8.3, -14.1, 3.1], vec![84.5, 1.0, 0.0]),
      Cam::new(vec![4.0, 3.0, 5.0], vec![-150.0, -30.0, 3.0]),
      Cam::new(vec![10.0, 10.0, 5.0], vec![-150.0, 0.0, 3.0]),
      Cam::new(vec![-20.0, -20.0, 10.0], vec![45.0, -15.0, 0.0])
    ].into_iter().enumerate().collect(), // to BTreeMap<usize, Cam>
    tcms: vec![].into_iter().collect(),
    obgs: vec![].into_iter().collect(), mbgs: vec![].into_iter().collect(),
    vbgs: vec![].try_into().unwrap_or_else(|o: std::convert::Infallible|
      panic!("Expected VecDeque<dBodyID> from vec![] Infallible ({:?})", o)),
    modified: false, gws: Gws::new(), t_delta: delta}
}

/// ODE initialize
pub fn open() {
unsafe {
  // need this code (do nothing) to create instance
  let gws: &mut Gws = &mut ode_get_mut!(gws);
  gws.world_(0 as dWorldID); // force call new before create_world
/*
  // need this code (do nothing) to create instance
  let v = &mut OYK_MUT;
  v[0].gws.world_(0 as dWorldID); // force call new before create_world
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

/// modify
pub fn modify(&mut self) {
  self.modified = true;
}

/// is modified (set false when f is false, otherwise through)
pub fn is_modified(&mut self, f: bool) -> bool {
  let r = self.modified;
  if r { self.modified = f; }
  r
}

/// number of elements
pub fn num(&self) -> usize {
  self.obgs.len()
}

/// set color, TCMaterial and reg
pub fn set_material_and_reg(&mut self, key: String,
  body: dBodyID, geom: dGeomID, mi: Box<dyn MetaInf>) -> dBodyID {
unsafe {
  dGeomSetBody(geom, body);
}
  // self.tcms.entry(geom).or_insert_with(|| TCMaterial::new(0, col));
  match self.tcms.entry(geom) { // expect geom is never duplicated
    Entry::Occupied(mut oe) => {
      let e = oe.get_mut();
      println!("tcms {:?} already exists. skipped", geom); // just in case
      e
    },
    Entry::Vacant(ve) => { ve.insert(mi.get_tcm().clone()) }
  };
  let obg: Obg = Obg::new(key, body, geom, mi.get_tcm().col);
  self.reg(obg)
}

/// make sphere primitive object (register it to show on the ODE space world)
pub fn mk_sphere(&mut self, key: String, mi: Box<dyn MetaInf>) -> dBodyID {
  let ms = mi.as_sphere();
  let mut mass: dMass = dMass::new();
unsafe {
  let gws: &mut Gws = &mut self.gws;
  dMassSetSphereTotal(&mut mass, ms.m, ms.r);
  let body: dBodyID = dBodyCreate(gws.world());
  dBodySetMass(body, &mass);
  let geom: dGeomID = dCreateSphere(gws.space(), ms.r);
  self.set_material_and_reg(key, body, geom, mi)
}
}

/// make plane primitive object
pub fn mk_box(&mut self, key: String, mi: Box<dyn MetaInf>) -> dBodyID {
  let mb = mi.as_box();
  let mut mass: dMass = dMass::new();
unsafe {
  let gws: &mut Gws = &mut self.gws;
  dMassSetBox(&mut mass, mb.dm, mb.lxyz[0], mb.lxyz[1], mb.lxyz[2]);
  let body: dBodyID = dBodyCreate(gws.world());
  dBodySetMass(body, &mass);
  let geom: dGeomID = dCreateBox(gws.space(),
    mb.lxyz[0], mb.lxyz[1], mb.lxyz[2]);
  self.set_material_and_reg(key, body, geom, mi)
}
}

/// make plane primitive object
pub fn mk_capsule(&mut self, key: String, mi: Box<dyn MetaInf>) -> dBodyID {
  let mc = mi.as_capsule();
  let mut mass: dMass = dMass::new();
unsafe {
  let gws: &mut Gws = &mut self.gws;
  dMassSetCapsule(&mut mass, mc.dm, 3, mc.r, mc.l);
  let body: dBodyID = dBodyCreate(gws.world());
  dBodySetMass(body, &mass);
  let geom: dGeomID = dCreateCapsule(gws.space(), mc.r, mc.l);
  self.set_material_and_reg(key, body, geom, mi)
}
}

/// make plane primitive object
pub fn mk_cylinder(&mut self, key: String, mi: Box<dyn MetaInf>) -> dBodyID {
  let mc = mi.as_cylinder();
  let mut mass: dMass = dMass::new();
unsafe {
  let gws: &mut Gws = &mut self.gws;
  dMassSetCylinder(&mut mass, mc.dm, 3, mc.r, mc.l);
  let body: dBodyID = dBodyCreate(gws.world());
  dBodySetMass(body, &mass);
  let geom: dGeomID = dCreateCylinder(gws.space(), mc.r, mc.l);
  self.set_material_and_reg(key, body, geom, mi)
}
}

/// make plane primitive object
pub fn mk_plane(&mut self, key: String, mi: Box<dyn MetaInf>) -> dBodyID {
  let mp = mi.as_plane();
  let mut mass: dMass = dMass::new();
unsafe {
  let gws: &mut Gws = &mut self.gws;
  dMassSetBox(&mut mass, mp.dm, mp.lxyz[0], mp.lxyz[1], mp.lxyz[2]);
  let body: dBodyID = dBodyCreate(gws.world());
  dBodySetMass(body, &mass);
  let geom: dGeomID = dCreatePlane(gws.space(),
    mp.norm[0], mp.norm[1], mp.norm[2], mp.norm[3]);
  self.set_material_and_reg(key, body, geom, mi)
}
}

/// register object (to HashMap and BTreeMap)
pub fn reg(&mut self, obg: Obg) -> dBodyID {
  let id = obg.body();
  let obgs: &mut HashMap<dBodyID, Obg> = &mut self.obgs;
  // let key = obgs.entry(id).or_insert(obg).key.clone();
  let key = match obgs.entry(id) { // expect id is never duplicated
    Entry::Occupied(oe) => {
      let k = oe.get().key.clone();
      println!("obgs {:?}[{}] already exists. skipped", id, k); // just in case
      k
    },
    Entry::Vacant(ve) => { ve.insert(obg).key.clone() }
  };
  let mbgs: &mut BTreeMap<String, dBodyID> = &mut self.mbgs;
  // mbgs.insert(key, id);
  match mbgs.entry(key.clone()) { // expect key is never duplicated
    BTreeEntry::Occupied(mut oe) => {
      let mut e = oe.get_mut();
      println!("mbgs [{}] already exists. replaced", key); // warning
      *e = id;
      e
    },
    BTreeEntry::Vacant(ve) => { ve.insert(id) }
  };
  let vbgs: &mut VecDeque<dBodyID> = &mut self.vbgs;
  vbgs.push_back(id);
  self.modify();
  id
}

/// search material mut (from HashMap)
pub fn get_tcm_mut(&mut self, id: dGeomID) ->
  Result<&mut TCMaterial, Box<dyn Error>> {
  let tcms: &mut HashMap<dGeomID, TCMaterial> = &mut self.tcms;
  Ok(tcms.get_mut(&id).ok_or(ODEError::no_tcm_id(id))?)
}

/// search material (from HashMap)
pub fn get_tcm(&self, id: dGeomID) -> Result<&TCMaterial, Box<dyn Error>> {
  let tcms: &HashMap<dGeomID, TCMaterial> = &self.tcms;
  Ok(tcms.get(&id).ok_or(ODEError::no_tcm_id(id))?)
}

/// search id (from BTreeMap)
pub fn get_id(&self, k: String) -> Result<dBodyID, Box<dyn Error>> {
  let mbgs: &BTreeMap<String, dBodyID> = &self.mbgs;
  // not use mbgs[&k]
  Ok(*mbgs.get(&k).ok_or(ODEError::no_key(k))?)
}

/// search object mut (from HashMap)
pub fn get_mut(&mut self, id: dBodyID) -> Result<&mut Obg, Box<dyn Error>> {
  let obgs: &mut HashMap<dBodyID, Obg> = &mut self.obgs;
  Ok(obgs.get_mut(&id).ok_or(ODEError::no_id(id))?)
}

/// search object mut (from BTreeMap and HashMap)
pub fn find_mut(&mut self, k: String) -> Result<&mut Obg, Box<dyn Error>> {
  let id: dBodyID = self.get_id(k)?;
  self.get_mut(id)
}

/// search object (from HashMap)
pub fn get(&self, id: dBodyID) -> Result<&Obg, Box<dyn Error>> {
  let obgs: &HashMap<dBodyID, Obg> = &self.obgs;
  Ok(obgs.get(&id).ok_or(ODEError::no_id(id))?)
}

/// search object (from BTreeMap and HashMap)
pub fn find(&self, k: String) -> Result<&Obg, Box<dyn Error>> {
  let id: dBodyID = self.get_id(k)?;
  self.get(id)
}

/// each_id (may use immutable result with get_mut to avoid dup mutable borrow)
pub fn each_id(&self, la: fn(key: &str, id: dBodyID) -> bool) -> Vec<dBodyID> {
  let mut r: Vec<dBodyID> = vec![];
  for (k, v) in &self.mbgs {
    r.push(if la(k, *v) { *v } else { 0 as dBodyID });
  }
  r
}

/// each (can break by result of lambda)
pub fn each(&self, la: fn(key: &str, id: dBodyID, obg: &Obg) -> bool) -> bool {
  for (k, v) in &self.mbgs {
    match self.get(*v) {
      Err(e) => { println!("{}", e); }, // may not be arrived here
      Ok(obg) => { if !la(k, *v, obg) { return false; } }
    }
  }
  true
}

/// destroy object (not unregister)
pub fn destroy_obg(obg: &Obg) {
unsafe {
  // dGeomDestroy(obg.geom()); // not use it
  let mut geom: dGeomID = dBodyGetFirstGeom(obg.body());
  while(geom != 0 as dGeomID){
    let nextgeom: dGeomID = dBodyGetNextGeom(geom);
    // dGeomTriMeshDataDestroy(tmd); // when geom has dTriMeshDataID tmd
    // UnMapGeomTriMesh(geom); // needless (to be deleted in clear_obgs())
    // UnMapGeomConvex(geom); // needless (to be deleted in clear_obgs())
    dGeomDestroy(geom);
    geom = nextgeom;
  }
  dBodyDestroy(obg.body());
}
}

/// destroy and unregister all objects
pub fn clear_obgs() {
unsafe {
  let obgs: &HashMap<dBodyID, Obg> = &ode_get!(obgs);
  for (id, obg) in obgs {
    ODE::destroy_obg(obg); // not obgs.remove(id);
  }
  let obgs: &mut HashMap<dBodyID, Obg> = &mut ode_get_mut!(obgs);
  obgs.clear();
  let mbgs: &mut BTreeMap<String, dBodyID> = &mut ode_get_mut!(mbgs);
  mbgs.clear();
  let vbgs: &mut VecDeque<dBodyID> = &mut ode_get_mut!(vbgs);
  vbgs.clear();
  let tcms: &mut HashMap<dGeomID, TCMaterial> = &mut ode_get_mut!(tcms);
  tcms.clear();
  let rode: &mut ODE = &mut ode_mut!();
  rode.modify();
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
  let cams: &mut BTreeMap<usize, Cam> = &mut ode_get_mut!(cams);
  let cam = cams.get_mut(sw_viewpoint).unwrap(); // &mut cams[sw_viewpoint];
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
      let cams: &mut BTreeMap<usize, Cam> = &mut ode_get_mut!(cams);
      let cam = cams.get_mut(sw_viewpoint).unwrap(); // &mut cams[sw_viewpoint];
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
  r_sim: Option<Box<dyn Sim>>,
  a: &[u8]) {
unsafe {
  let sim: &mut Option<Box<dyn Sim>> = &mut ode_get_mut!(sim);
  *sim = r_sim;
  let ptt: &mut Option<U8zBuf> = &mut ode_get_mut!(ptt);
  *ptt = Some(U8zBuf::from_u8array(a)); // to keep lifetime
  let mut dsfn: dsFunctions = dsFunctions{
    version: DS_VERSION,
    start: Some(c_start_callback), // Option<unsafe extern "C" fn()>
    step: Some(c_step_callback), // Option<unsafe extern "C" fn(i32)>
    command: Some(c_command_callback), // Option<unsafe extern "C" fn(i32)>
    stop: Some(c_stop_callback), // Option<unsafe extern "C" fn()>
    path_to_textures: ptt.as_ref().expect("not init").as_i8p()
  };

  let mut cab: CArgsBuf = CArgsBuf::from(&std::env::args().collect());
  dsSimulationLoop(cab.as_argc(), cab.as_argv_ptr_mut(),
    width, height, &mut dsfn);
}
}

} // impl ODE

/// binding finalize ODE (auto called)
impl Drop for ODE {
  fn drop(&mut self) {
    unsafe { dCloseODE(); }
    ostatln!("dropped ODE");
  }
}

/// trait Sim must have callback functions
pub trait Sim {
  /// self.super mutable
  fn super_mut(&mut self) -> &mut ODE {
unsafe {
    &mut ode_mut!()
}
  }

  /// self.super immutable
  fn super_get(&self) -> &ODE {
unsafe {
    &ode_!()
}
  }

  /// draw_geom function
  fn draw_geom(&self, geom: dGeomID,
    pos: Option<*const dReal>, rot: Option<*const dReal>, ws: i32);
  /// draw default function
  fn draw_objects(&mut self);
  /// start default callback function
  fn start_callback(&mut self);
  /// near default callback function
  fn near_callback(&mut self, o1: dGeomID, o2: dGeomID);
  /// step default callback function
  fn step_callback(&mut self, pause: i32);
  /// command default callback function
  fn command_callback(&mut self, cmd: i32);
  /// stop default callback function
  fn stop_callback(&mut self);
}

/// trait Sim must have callback functions
impl Sim for ODE {

/// implements drawing composite
/// wire_solid i32 false/true for bunny
fn draw_objects(&mut self) {
  ostatln!("called default draw");
  let wire_solid = self.wire_solid; // for bunny
  let obgs = &self.obgs;
  let vbgs = &self.vbgs;
  for id in vbgs { // drawing order
    let obg = &obgs[&id];
unsafe {
    let mut g = dBodyGetFirstGeom(obg.body());
    while g != 0 as dGeomID {
      self.draw_geom(g, None, None, wire_solid);
      g = dBodyGetNextGeom(g);
    }
}
  }
}

/// draw_geom (called by draw_objects and recursive)
fn draw_geom(&self, geom: dGeomID,
  pos: Option<*const dReal>, rot: Option<*const dReal>, ws: i32) {
  if geom == 0 as dGeomID { return; }
unsafe {
  let pos: *const dReal = pos.unwrap_or_else(|| dGeomGetPosition(geom));
  let rot: *const dReal = rot.unwrap_or_else(|| dGeomGetRotation(geom));
  let col: &dVector4 = match self.get_tcm(geom) {
    Err(e) => {
      let obg = self.get(dGeomGetBody(geom)).unwrap(); // must care ok_or
      &obg.col
    },
    Ok(tcm) => {
      dsSetTexture(tcm.tex);
      &tcm.col
    }
  };
  let c: Vec<f32> = col.into_iter().map(|v| *v as f32).collect();
  dsSetColorAlpha(c[0], c[1], c[2], c[3]);
  let cls = dGeomGetClass(geom);
  match cls {
    dSphereClass => {
      let radius: dReal = dGeomSphereGetRadius(geom);
      dsDrawSphereD(pos, rot, radius as f32);
    },
    dBoxClass => {
      let mut lxyz: dVector3 = [0.0; 4];
      dGeomBoxGetLengths(geom, &mut lxyz[0] as *mut dReal);
      dsDrawBoxD(pos, rot, &lxyz[0] as *const dReal);
    },
    dCapsuleClass => {
      let mut r: dReal = 0.0;
      let mut l: dReal = 0.0;
      dGeomCapsuleGetParams(geom, &mut r as *mut dReal, &mut l as *mut dReal);
      dsDrawCapsuleD(pos, rot, l as f32, r as f32);
    },
    dCylinderClass => {
      let mut r: dReal = 0.0;
      let mut l: dReal = 0.0;
      dGeomCylinderGetParams(geom, &mut r as *mut dReal, &mut l as *mut dReal);
      dsDrawCylinderD(pos, rot, l as f32, r as f32);
    },
    dPlaneClass => {
      let mut norm: dVector4 = [0.0; 4];
      dGeomPlaneGetParams(geom, &mut norm[0] as *mut dReal);
      // (a Plane is not a Box) dGeomBoxGetLengths
      let lxyz: dVector3 = [10.0, 10.0, 0.05, 0.0]; // ***
      dsDrawBoxD(pos, rot, &lxyz[0] as *const dReal);
    },
    _ => { println!("unknown class: {}", cls); }
  }
}
}

/// start default callback function
fn start_callback(&mut self) {
  ostatln!("called default start");
  ODE::viewpoint_();
unsafe {
  dsSetSphereQuality(3); // default sphere 1
  dsSetCapsuleQuality(3); // default capsule 3
}
}

/// near default callback function
fn near_callback(&mut self, o1: dGeomID, o2: dGeomID) {
  ostatln!("called default near");
  let gws = &self.gws;
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
fn step_callback(&mut self, pause: i32) {
  ostatln!("called default step");
  let gws = &self.gws;
  let t_delta = &self.t_delta;
  if pause != 1 {
unsafe {
    dSpaceCollide(gws.space(), 0 as *mut c_void, Some(c_near_callback));
    dWorldStep(gws.world(), *t_delta);
    dJointGroupEmpty(gws.contactgroup());
}
  }
  ode_sim!(self, draw_objects)
}

/// command default callback function
fn command_callback(&mut self, cmd: i32) {
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
      *sw_viewpoint = (*sw_viewpoint + 1) % self.cams.len();
}
      ODE::viewpoint_();
      ODE::viewpoint(false);
    },
    'r' => {
      ODE::clear_obgs();
      ODE::clear_contactgroup();
      ode_sim!(self, start_callback)
    },
    _ => {}
  }
}

/// stop default callback function
fn stop_callback(&mut self) {
  ostatln!("called default stop");
}

} // impl Sim for ODE

unsafe extern "C"
fn c_start_callback() {
  let rode: &mut ODE = &mut ode_mut!();
  ode_sim!(rode, start_callback)
}

unsafe extern "C"
fn c_near_callback(_dat: *mut c_void, o1: dGeomID, o2: dGeomID) {
  let rode: &mut ODE = &mut ode_mut!();
  ode_sim!(rode, near_callback, o1, o2)
}

unsafe extern "C"
fn c_step_callback(pause: i32) {
  let rode: &mut ODE = &mut ode_mut!();
  ode_sim!(rode, step_callback, pause)
}

unsafe extern "C"
fn c_command_callback(cmd: i32) {
  let rode: &mut ODE = &mut ode_mut!();
  ode_sim!(rode, command_callback, cmd)
}

unsafe extern "C"
fn c_stop_callback() {
  let rode: &mut ODE = &mut ode_mut!();
  ode_sim!(rode, stop_callback)
}

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
