//! ode integrates bindings to cppbridge and cppode
//!
//! oyk is replaced to submodule of crate ode-rs after version 1.0.1
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
//!
//! to
//!
//!  - include/bridge_bindings.rs
//!
//! # Requirements
//!
//! in the running directory
//!
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

// mod cppbridge;
// use cppbridge::*;
// pub use cppbridge::{Bridge, bput};

//pub use ode_base::ode::*;
use ode_base::ode::{self, *};
pub use ode::{dBodyID, dGeomID, dTriIndex};
pub use ode::{dMatrix4, dMatrix3, dVector4, dVector3, dReal}; // 16 12 4 4
pub use ode::{dQuaternion};

#[warn(unused)]
// #[warn(unused_imports)]
// #[warn(unused_attributes)]
#[warn(non_snake_case)]
#[warn(non_camel_case_types)]
#[warn(non_upper_case_globals)]

use std::error::Error;

// pub mod err in ode_base::ode::err
pub use ode::err::{self, *};

// pub mod mat in ode_base::ode::mat
pub use ode::mat::{self, *};

// pub mod prim in ode_base::ode::prim
use ode::prim::{self, *};
pub use prim::{Matrix4, M4I, Matrix3, M3I, Quaternion, QI};
pub use prim::{PId, PI, PIh, PIt, PIq, PIx};
pub use prim::{PIh3, PIt2, PIt4, PIt5, PIq3, PIx5};

// pub mod krp in ode_base::ode::krp
use ode::krp::{self, *};
pub use krp::{Krp, KRPnk, KRP100, KRP095, KRP080, KRP001};

pub mod trimeshconvex;
use trimeshconvex::*;
pub use trimeshconvex::{TriMesh, Convex};
pub use trimeshconvex::{custom, tetra, cube, icosahedron, bunny};

pub mod meta;
use meta::*;
pub use meta::{MetaInf, MetaConvex, MetaTriMesh, MetaComposite};
pub use meta::{MetaSphere, MetaBox, MetaCapsule, MetaCylinder, MetaPlane};

pub mod cls;
use cls::*;
pub use cls::{obg::{Obg, Gws}, AsPtr};

pub mod ds;
use ds::*;
pub use ds::{Tdrawstuff, TdrawstuffSetter};

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

/// help default defined key set
const KEY_HELP: &str = "
  default defined key set
  ctrl + 'P': pause on/off
  ctrl + 'O': single step
  ctrl + 'S': shadow on/off
  ctrl + 'T': texture on/off
  'w': wireframe (trimesh) solid/wireframe (show hidden face edge)
  'p': wireframe (all) polyfill/wireframe (not show hidden face edge)
  'v': show viewpoint (current num pos hpr)
  's': change viewpoint (8 pattern preset/set)
  'r': reset
  '?': help (this message)
";

/// ODE singleton
pub struct ODE { // unsafe
  ds: Option<Box<dyn Tdrawstuff>>, // trait Tdrawstuff (for late binding)
  sim: Option<Box<dyn Sim>>, // trait Sim must have callback functions
  ptt: Option<U8zBuf>, // relative path to textures
  wire_solid: i32, // 0: wireframe, 1: solid (for bunny)
  polyfill_wireframe: i32, // 0: solid, 1: wireframe (for all)
  sw_viewpoint: usize, // switch viewpoint
  /// viewpoint(s)
  pub cams: BTreeMap<usize, Cam>,
  rgts: HashMap<dGeomID, dGeomID>, // reverse geom gtrans dGeomGetGeomTransform
  mgms: HashMap<dGeomID, Box<dyn MetaInf>>, // metainf(s) material(s) mapped
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
  ODE{ds: None, sim: None, ptt: None,
    wire_solid: 1, polyfill_wireframe: 0, sw_viewpoint: 0,
    cams: vec![
      Cam::new(vec![5.0, 0.0, 2.0], vec![-180.0, 0.0, 0.0]),
      Cam::new(vec![0.0, 0.0, 20.0], vec![-180.0, -30.0, 0.0]),
      Cam::new(vec![5.36, 2.02, 4.28], vec![-162.0, -31.0, 0.0]),
      Cam::new(vec![-8.3, -14.1, 3.1], vec![84.5, 1.0, 0.0]),
      Cam::new(vec![4.0, 3.0, 5.0], vec![-150.0, -30.0, 3.0]),
      Cam::new(vec![10.0, 10.0, 5.0], vec![-150.0, 0.0, 3.0]),
      Cam::new(vec![-20.0, -20.0, 10.0], vec![45.0, -15.0, 0.0])
    ].into_iter().enumerate().collect(), // to BTreeMap<usize, Cam>
    rgts: vec![].into_iter().collect(), mgms: vec![].into_iter().collect(),
    obgs: vec![].into_iter().collect(), mbgs: vec![].into_iter().collect(),
    vbgs: vec![].try_into().unwrap_or_else(|o: std::convert::Infallible|
      panic!("Expected VecDeque<dBodyID> from vec![] Infallible ({:?})", o)),
    modified: false, gws: Gws::new(), t_delta: delta}
}

/// ds trait Tdrawstuff getter
pub fn ds_as_ref() -> &'static Box<dyn Tdrawstuff> {
unsafe {
    ode_get!(ds).as_ref().expect("get ds trait Tdrawstuff")
}
}

/// ODE initialize
pub fn open(drawstuff: impl Tdrawstuff + 'static) {
unsafe {
  ODE::set_drawstuff(&mut ode_get_mut!(ds), drawstuff);

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

/// reg mgm (MetaInf, TCMaterial) (geom to HashMap) returns color
pub fn reg_mgm(&mut self, geom: dGeomID, mi: Box<dyn MetaInf>) -> dVector4 {
  let col = mi.get_tcm().col;
  // self.mgms.entry(geom).or_insert_with(|| mi);
  match self.mgms.entry(geom) { // expect geom is never duplicated
    Entry::Occupied(mut oe) => {
      let e = oe.get_mut();
      println!("mgms {:?} already exists. skipped", geom); // just in case
      e
    },
    Entry::Vacant(ve) => { ve.insert(mi) }
  };
  col
}

/// reg obg (body to HashMap, BTreeMap, and VecDeque) returns body
pub fn reg_obg(&mut self, obg: Obg) -> dBodyID {
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

/// create primitive object (register it to show on the ODE space world)
pub fn creator_dm(&mut self, key: &str, mi: Box<dyn MetaInf>, fmdm: bool) ->
  (dBodyID, dGeomID, Box<dMass>) {
  let gws: &mut Gws = &mut self.gws;
  let world: dWorldID = gws.world();
  let space: dSpaceID = if key == "" { 0 as dSpaceID } else { gws.space() };
  let body: dBodyID;
  let geom: dGeomID;
  let mut mass: Box<dMass> = Box::new(dMass::new());
unsafe {
  geom = match mi.id() {
    MetaId::Sphere => {
      let ms = mi.as_sphere();
      if fmdm {
        dMassSetSphereTotal(&mut *mass, ms.dm, ms.r); // dm as m
      }else{
        dMassSetSphere(&mut *mass, ms.dm, ms.r);
      }
      dCreateSphere(space, ms.r)
    },
    MetaId::Box => {
      let mb = mi.as_box();
      if fmdm {
        dMassSetBoxTotal(&mut *mass, mb.dm,
          mb.lxyz[0], mb.lxyz[1], mb.lxyz[2]); // dm as m
      }else{
        dMassSetBox(&mut *mass, mb.dm, mb.lxyz[0], mb.lxyz[1], mb.lxyz[2]);
      }
      dCreateBox(space, mb.lxyz[0], mb.lxyz[1], mb.lxyz[2])
    },
    MetaId::Capsule => {
      let mc = mi.as_capsule();
      dMassSetCapsule(&mut *mass, mc.dm, 3, mc.r, mc.l);
      dCreateCapsule(space, mc.r, mc.l)
    },
    MetaId::Cylinder => {
      let mc = mi.as_cylinder();
      dMassSetCylinder(&mut *mass, mc.dm, 3, mc.r, mc.l);
      dCreateCylinder(space, mc.r, mc.l)
    },
    MetaId::Plane => {
      let mp = mi.as_plane();
      if fmdm {
        dMassSetBoxTotal(&mut *mass, mp.dm,
          mp.lxyz[0], mp.lxyz[1], mp.lxyz[2]); // dm as m
      }else{
        dMassSetBox(&mut *mass, mp.dm, mp.lxyz[0], mp.lxyz[1], mp.lxyz[2]);
      }
      dCreatePlane(space, mp.norm[0], mp.norm[1], mp.norm[2], mp.norm[3])
    },
    MetaId::Convex => {
      let mc = mi.as_convex();
      let g: dGeomID = CreateGeomConvexFromFVP(space, mc.fvp);
      MassSetConvexAsTrimesh(&mut *mass, mc.dm, mc.fvp);
      dGeomSetPosition(g, -mass.c[0], -mass.c[1], -mass.c[2]); // ***
      dMassTranslate(&mut *mass, -mass.c[0], -mass.c[1], -mass.c[2]); // ***
      g
    },
    MetaId::TriMesh => {
      let mt = mi.as_trimesh();
      let g: dGeomID = CreateGeomTrimeshFromVI(space, mt.tmv);
      dMassSetTrimesh(&mut *mass, mt.dm, g);
      dGeomSetPosition(g, -mass.c[0], -mass.c[1], -mass.c[2]); // ***
      dMassTranslate(&mut *mass, -mass.c[0], -mass.c[1], -mass.c[2]); // ***
      g
    },
    MetaId::Composite => {
      panic!("use creator_composite for {:?}", mi.id());
    },
    _ => { panic!("creator not implemented for {:?}", mi.id()); }
  };
}
  let col = self.reg_mgm(geom, mi); // must call reg_mgm when not use col
  (if key == "" {
    0 as dBodyID
  }else{
unsafe {
    body = dBodyCreate(world);
    dBodySetMass(body, &*mass);
    dGeomSetBody(geom, body);
    if !self.get_krp(geom).g { dBodyDisable(body); } // care no id
}
    let obg: Obg = Obg::new(key, body, geom, col);
    self.reg_obg(obg)
  }, geom, mass)
}

/// create primitive object as m (register it to show on the ODE space world)
pub fn creator_m(&mut self, key: &str, mi: Box<dyn MetaInf>) ->
  (dBodyID, dGeomID, Box<dMass>) {
  self.creator_dm(key, mi, true)
}

/// create primitive object as dm (register it to show on the ODE space world)
pub fn creator(&mut self, key: &str, mi: Box<dyn MetaInf>) ->
  (dBodyID, dGeomID, Box<dMass>) {
  self.creator_dm(key, mi, false)
}

/// create composite object (register it to show on the ODE space world)
pub fn creator_composite(&mut self, key: &str, mi: Box<dyn MetaInf>) ->
  (dBodyID, dGeomID, Box<dMass>) {
  let gws: &mut Gws = &mut self.gws;
  let world: dWorldID = gws.world();
  let space: dSpaceID = gws.space();
  let body: dBodyID;
  let geom: dGeomID;
  let mut mass: Box<dMass> = Box::new(dMass::new());
  // to keep order
  let mut gto: VecDeque<dGeomID> = vec![].try_into().unwrap(); // no or_else
  // gtrans, {gsub, o}
  let mut gts: HashMap<dGeomID, GeomOffset> = vec![].into_iter().collect();
unsafe {
  body = dBodyCreate(world);
}
  geom = match mi.id() {
    MetaId::Composite => {
      let mc = mi.as_composite();
unsafe {
      for (j, emi) in mc.elems.iter().enumerate() {
        let gtrans: dGeomID = dCreateGeomTransform(space);
        dGeomTransformSetCleanup(gtrans, 1);
        let (_, gsub, mut subm) = self.creator("", emi.dup());
        dGeomTransformSetGeom(gtrans, gsub);
        gto.push_front(gtrans); // first <-> last gto.push_back(gtrans);
        let o = &mc.ofs[j];
        gts.entry(gtrans).or_insert_with(|| GeomOffset{gsub: gsub, o: o});
        dGeomSetPosition(gsub, o[0], o[1], o[2]);
        dMassTranslate(&mut *subm, o[0], o[1], o[2]);
        let q = &mc.qs[j];
        // dQSetIdentity(q.as_ptr_mut());
        // dQFromAxisAndAngle(q.as_ptr_mut(), , , , M_PI / 2.0);
        dGeomSetQuaternion(gsub, q.as_ptr() as *mut dReal);
        dMassRotate(&mut *subm, dMatrix3::from_Q(*q).as_ptr() as *mut dReal);
        dMassAdd(&mut *mass, &*subm);
        self.rgts.entry(gsub).or_insert(gtrans); // reverse geom gtrans
      }
      0 as dGeomID // composite Obg has no geom
}
    },
    _ => { panic!("use creator for {:?}", mi.id()); }
  };
unsafe {
  // adjust from CG != (0, 0, 0)
  for (gtrans, gso) in &gts {
    let o = gso.o;
    dGeomSetPosition(gso.gsub, o[0]-mass.c[0], o[1]-mass.c[1], o[2]-mass.c[2]);
  }
  dMassTranslate(&mut *mass, -mass.c[0], -mass.c[1], -mass.c[2]);
  dBodySetMass(body, &*mass); // CG == (0, 0, 0)
  for gtrans in &gto {
    dGeomSetBody(*gtrans, body);
  }
  if !self.get_krp(gts[&gto[0]].gsub).g { dBodyDisable(body); } // care no id
}
  gts.clear();
  gto.clear();
  let col = mi.get_tcm().col;
  let obg: Obg = Obg::new(key, body, geom, col);
  (self.reg_obg(obg), geom, mass)
}

/// search grand parent body
pub fn get_grand_parent(&self, id: dGeomID) -> dBodyID {
unsafe {
  let mut b: dBodyID = dGeomGetBody(id);
  if b == 0 as dBodyID { // assume sub geom in the GeomTransform
    b = dGeomGetBody(self.rgts[&id]); // reverse geom gtrans (no care or_else)
  }
  b
}
}

/// search bounce (especially support GeomTransform)
pub fn get_bounce(&self, id: dGeomID) -> dReal {
  let gid: dGeomID = match unsafe { dGeomGetClass(id) } {
    dGeomTransformClass => unsafe { dGeomTransformGetGeom(id) },
    _ => id
  }; // GeomTransform is never registered
  match self.get_mgm(gid) {
    Err(_) => KRP100.bounce, // will not be arrived here (check class before)
    Ok(mgm) => mgm.get_krp().bounce
  }
}

/// search Krp (from HashMap)
pub fn get_krp(&self, id: dGeomID) -> &Krp {
  // self.get_mgm(id).unwrap().get_krp() // no care or_else
  match self.get_mgm(id) {
    Err(_) => &KRP100, // or raise panic!
    Ok(mgm) => mgm.get_krp()
  }
}

/// search MetaInf, TCMaterial mut (from HashMap)
pub fn get_mgm_mut(&mut self, id: dGeomID) ->
  Result<&mut Box<dyn MetaInf>, Box<dyn Error>> {
  let mgms: &mut HashMap<dGeomID, Box<dyn MetaInf>> = &mut self.mgms;
  Ok(mgms.get_mut(&id).ok_or(ODEError::no_mgm_id(id))?)
}

/// search MetaInf, TCMaterial (from HashMap)
pub fn get_mgm(&self, id: dGeomID) ->
  Result<&Box<dyn MetaInf>, Box<dyn Error>> {
  let mgms: &HashMap<dGeomID, Box<dyn MetaInf>> = &self.mgms;
  Ok(mgms.get(&id).ok_or(ODEError::no_mgm_id(id))?)
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
  let mgms: &mut HashMap<dGeomID, Box<dyn MetaInf>> = &mut ode_get_mut!(mgms);
  mgms.clear();
  let rgts: &mut HashMap<dGeomID, dGeomID> = &mut ode_get_mut!(rgts);
  rgts.clear();
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
  let ds = ODE::ds_as_ref();
unsafe {
  let sw_viewpoint: &usize = &ode_get!(sw_viewpoint);
  let cams: &mut BTreeMap<usize, Cam> = &mut ode_get_mut!(cams);
  let cam = cams.get_mut(sw_viewpoint).unwrap(); // &mut cams[sw_viewpoint];
  let pos: &mut [f32] = &mut cam.pos;
  let ypr: &mut [f32] = &mut cam.ypr;
  ds.SetViewpoint(pos as *mut [f32] as *mut f32, ypr as *mut [f32] as *mut f32);
}
}

/// get viewpoint (f: true, save to the current viewpoint Cam[sw_viewpoint])
pub fn viewpoint(f: bool) {
  let ds = ODE::ds_as_ref();
unsafe {
  let p: &mut [f32] = &mut vec![0.0; 4];
  let y: &mut [f32] = &mut vec![0.0; 4];
  ds.GetViewpoint(p as *mut [f32] as *mut f32, y as *mut [f32] as *mut f32);
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
  let ds = ODE::ds_as_ref();
unsafe {
  let sim: &mut Option<Box<dyn Sim>> = &mut ode_get_mut!(sim);
  *sim = r_sim;
  let ptt: &mut Option<U8zBuf> = &mut ode_get_mut!(ptt);
  *ptt = Some(U8zBuf::from_u8array(a)); // to keep lifetime
  let mut dsfn: dsFunctions_C = dsFunctions_C{
    version: DS_VERSION,
    start: Some(c_start_callback), // Option<unsafe extern "C" fn()>
    step: Some(c_step_callback), // Option<unsafe extern "C" fn(i32)>
    command: Some(c_command_callback), // Option<unsafe extern "C" fn(i32)>
    stop: Some(c_stop_callback), // Option<unsafe extern "C" fn()>
    path_to_textures: ptt.as_ref().expect("not init").as_i8p()
  };

  let mut cab: CArgsBuf = CArgsBuf::from(&std::env::args().collect());
  ds.SimulationLoop(cab.as_argc(), cab.as_argv_ptr_mut(),
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

  /// set pos and rotation (dMatrix3)
  fn set_pos_R(&mut self, b: dBodyID, p: dVector3, m: dMatrix3) {
    self.super_mut().get_mut(b).expect("no body").set_pos(p).set_rot(m);
  }

  /// set pos and rotation (dQuaternion)
  fn set_pos_Q(&mut self, b: dBodyID, p: dVector3, q: dQuaternion) {
    self.super_mut().get_mut(b).expect("no body").set_pos(p).set_quaternion(q);
  }

  /// draw_geom function
  fn draw_geom(&self, geom: dGeomID,
    pos: Option<*const dReal>, rot: Option<*const dReal>, ws: i32);
  /// draw default function
  fn draw_objects(&mut self);
  /// start default callback function
  fn start_callback(&mut self);
  /// near default callback function
  fn near_callback(&mut self, dat: *mut c_void, o1: dGeomID, o2: dGeomID);
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
  let ds = ODE::ds_as_ref();
unsafe {
  let pos: *const dReal = pos.unwrap_or_else(|| dGeomGetPosition(geom));
  let rot: *const dReal = rot.unwrap_or_else(|| dGeomGetRotation(geom));
  let col: &dVector4 = match self.get_mgm(geom) {
    Err(e) => {
      let obg = self.get(dGeomGetBody(geom)).unwrap(); // must care ok_or
      &obg.col
    },
    Ok(mgm) => {
      let tcm = mgm.get_tcm();
      ds.SetTexture(tcm.tex);
      &tcm.col
    }
  };
  let c: Vec<f32> = col.into_iter().map(|v| *v as f32).collect();
  ds.SetColorAlpha(c[0], c[1], c[2], c[3]);
  let cls = dGeomGetClass(geom);
  match cls {
    dSphereClass => {
      let radius: dReal = dGeomSphereGetRadius(geom);
      ds.DrawSphereD(pos, rot, radius as f32);
    },
    dBoxClass => {
      let mut lxyz: dVector3 = [0.0; 4];
      dGeomBoxGetLengths(geom, lxyz.as_ptr_mut());
      ds.DrawBoxD(pos, rot, lxyz.as_ptr());
    },
    dCapsuleClass => {
      let mut r: dReal = 0.0;
      let mut l: dReal = 0.0;
      dGeomCapsuleGetParams(geom, &mut r as *mut dReal, &mut l as *mut dReal);
      ds.DrawCapsuleD(pos, rot, l as f32, r as f32);
    },
    dCylinderClass => {
      let mut r: dReal = 0.0;
      let mut l: dReal = 0.0;
      dGeomCylinderGetParams(geom, &mut r as *mut dReal, &mut l as *mut dReal);
      ds.DrawCylinderD(pos, rot, l as f32, r as f32);
    },
    dPlaneClass => {
      let mut norm: dVector4 = [0.0; 4];
      dGeomPlaneGetParams(geom, norm.as_ptr_mut());
      // (a Plane is not a Box) dGeomBoxGetLengths
      let lxyz: dVector3 = [10.0, 10.0, 0.05, 0.0]; // ***
      ds.DrawBoxD(pos, rot, lxyz.as_ptr());
    },
    dRayClass => {
      println!("not implemented class: {}", cls);
    },
    dConvexClass => {
      match self.get_mgm(geom) {
        Err(e) => { println!("not found convex {:?} geomID {:?}", e, geom); },
        Ok(mgm) => {
          let fvp: &convexfvp = &*mgm.as_convex().fvp;
          ds.DrawConvexD(pos, rot,
            fvp.faces, fvp.faceCount, fvp.vtx, fvp.vtxCount, fvp.polygons);
        }
      }
    },
    dGeomTransformClass => {
      let gt: dGeomID = dGeomTransformGetGeom(geom);
      let gtpos: *const dReal = dGeomGetPosition(gt);
      let gtrot: *const dReal = dGeomGetRotation(gt);
      let mut rpos = dVector3::multiply0_331_pp(rot, gtpos);
      let ppos = std::slice::from_raw_parts(pos, 4); // must be in unsafe
      for i in 0..4 { rpos[i] += ppos[i]; }
      let rrot = dMatrix3::multiply0_333_pp(rot, gtrot);
      self.draw_geom(gt, Some(rpos.as_ptr()), Some(rrot.as_ptr()), ws);
    },
    dTriMeshClass => {
      match self.get_mgm(geom) {
        Err(e) => { println!("not found trimesh {:?} geomID {:?}", e, geom); },
        Ok(mgm) => {
          let tmv: &trimeshvi = &*mgm.as_trimesh().tmv;
/* (C)
    int is_composite = (dGeomGetSpace(geom) == 0);
    dVector3 tpos = {0.0, 0.0, 0.0};
    dMatrix3 trot;
    dRSetIdentity(trot);
    int triCount = dGeomTriMeshGetTriangleCount(geom);
    for(int i = 0; i < triCount; ++i){
      dVector3 v0, v1, v2;
      dGeomTriMeshGetTriangle(geom, i, &v0, &v1, &v2); // already transformed
      if(!is_composite) dsDrawTriangleD(tpos, trot, v0, v1, v2, ws); // top
      else dsDrawTriangleD(pos, rot, v0, v1, v2, ws); // in the dTransformClass
    }
*/
          let vtx = tmv.as_slice_vtx();
          let p = tmv.as_slice_indices();
          for i in 0..p.len()/3 {
            let idx = [p[i*3] as usize, p[i*3+1] as usize, p[i*3+2] as usize];
            let v: [[dReal; 3]; 3] = [
              [vtx[idx[0] * 3 + 0], vtx[idx[0] * 3 + 1], vtx[idx[0] * 3 + 2]],
              [vtx[idx[1] * 3 + 0], vtx[idx[1] * 3 + 1], vtx[idx[1] * 3 + 2]],
              [vtx[idx[2] * 3 + 0], vtx[idx[2] * 3 + 1], vtx[idx[2] * 3 + 2]]];
            ds.DrawTriangleD(pos, rot,
              v[0].as_ptr(), v[1].as_ptr(), v[2].as_ptr(), ws);
          }
        }
      }
    },
    dHeightfieldClass => {
      println!("not implemented class: {}", cls);
    },
    _ => { println!("unknown class: {}", cls); }
  }
}
}

/// start default callback function
fn start_callback(&mut self) {
  ostatln!("called default start");
  let ds = ODE::ds_as_ref();
  ODE::viewpoint_();
unsafe {
  ds.SetSphereQuality(3); // default sphere 1
  ds.SetCapsuleQuality(3); // default capsule 3
}
}

/// near default callback function
fn near_callback(&mut self, dat: *mut c_void, o1: dGeomID, o2: dGeomID) {
  ostatln!("called default near");
  let gws = &self.gws;
  let world: dWorldID = gws.world();
  let contactgroup: dJointGroupID = gws.contactgroup();
  let ground: dGeomID = gws.ground();
unsafe {
  if dGeomIsSpace(o1) != 0 || dGeomIsSpace(o2) != 0 {
    dSpaceCollide2(o1, o2, dat, Some(c_near_callback));
    if dGeomIsSpace(o1) != 0 {
      dSpaceCollide(o1 as dSpaceID, dat, Some(c_near_callback));
    }
    if dGeomIsSpace(o2) != 0 {
      dSpaceCollide(o2 as dSpaceID, dat, Some(c_near_callback));
    }
    return;
  }
  const num: usize = 40;
  let contacts: &mut Vec<dContact> = &mut vec![dContact::new(); num];
  let sz: i32 = std::mem::size_of::<dContact>() as i32;
  let n: i32 = dCollide(o1, o2, num as i32, &mut contacts[0].geom, sz);
  if ground == o1 || ground == o2 { // vs ground
    let id: dGeomID = if ground == o1 { o2 } else { o1 };
    let bounce: dReal = self.get_bounce(id);
    for i in 0..n as usize {
      let p: &mut dContact = &mut contacts[i];
      p.surface.mode = dContactBounce | dContactSoftERP | dContactSoftCFM;
      p.surface.bounce = bounce; // or 0.0
      p.surface.bounce_vel = 0.01; // or 0.0 minimum velocity for bounce
      p.surface.mu = dInfinity; // or 0.5
      p.surface.soft_erp = 0.2;
      p.surface.soft_cfm = 0.001;
      let c: dJointID = dJointCreateContact(world, contactgroup, p);
      // dJointAttach(c, dGeomGetBody(p.geom.g1), dGeomGetBody(p.geom.g2));
      dJointAttach(c, dGeomGetBody(o1), dGeomGetBody(o2));
    }
  }else{
    let bounce: dReal = self.get_bounce(o1) * self.get_bounce(o2);
    for i in 0..n as usize {
      let p: &mut dContact = &mut contacts[i];
      p.surface.mode = dContactBounce;
      p.surface.bounce = bounce; // or 0.0
      p.surface.bounce_vel = 0.01; // or 0.0 minimum velocity for bounce
      p.surface.mu = dInfinity; // or 0.5
      let c: dJointID = dJointCreateContact(world, contactgroup, p);
      // dJointAttach(c, dGeomGetBody(p.geom.g1), dGeomGetBody(p.geom.g2));
      dJointAttach(c, dGeomGetBody(o1), dGeomGetBody(o2));
    }
  }
}
}

/// step default callback function
fn step_callback(&mut self, pause: i32) {
  ostatln!("called default step");
  let gws = &self.gws;
  let t_delta = &self.t_delta;
  if pause != 1 {
    let mut tmp: HashMap<dBodyID, dVector3> = vec![].into_iter().collect();
    for (id, mi) in &self.mgms {
      if !mi.get_krp().k {
unsafe {
        let b: dBodyID = self.get_grand_parent(*id);
        tmp.entry(b).or_insert(*(dBodyGetPosition(b) as *const dVector3));
}
      }
    }
unsafe {
    dSpaceCollide(gws.space(), 0 as *mut c_void, Some(c_near_callback));
    dWorldStep(gws.world(), *t_delta);
    dJointGroupEmpty(gws.contactgroup());
}
    for (b, p) in &tmp {
unsafe {
      dBodySetPosition(*b, p[0], p[1], p[2]);
}
    }
  }
  ode_sim!(self, draw_objects)
}

/// command default callback function
fn command_callback(&mut self, cmd: i32) {
  ostatln!("called default command");
  let ds = ODE::ds_as_ref();
  match cmd as u8 as char {
    'p' => {
unsafe {
      let polyfill_wireframe: &mut i32 = &mut ode_get_mut!(polyfill_wireframe);
      *polyfill_wireframe = 1 - *polyfill_wireframe;
      ds.SetDrawMode(*polyfill_wireframe);
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
    '?' => {
      println!("{}", KEY_HELP);
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
fn c_near_callback(dat: *mut c_void, o1: dGeomID, o2: dGeomID) {
  let rode: &mut ODE = &mut ode_mut!();
  ode_sim!(rode, near_callback, dat, o1, o2)
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
