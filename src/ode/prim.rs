//! prim
//!

use crate::ode::*;

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

/// angles
pub static PI: dReal = 3.14159265; // 180
pub static PId: dReal = PI * 2.0; // 360 dual
pub static PIh: dReal = PI / 2.0; // 90 half
pub static PIt: dReal = PI / 3.0; // 60 regular triangle
pub static PIq: dReal = PI / 4.0; // 45 quarter
pub static PIx: dReal = PI / 6.0; // 30 a sixth

pub static PIh3: dReal = PIh * 3.0; // 270

pub static PIt2: dReal = PIt * 2.0; // 120
pub static PIt4: dReal = PIt * 4.0; // 240
pub static PIt5: dReal = PIt * 5.0; // 300

pub static PIq3: dReal = PIq * 3.0; // 135

/// constructor and converter for primitive type
pub trait Quaternion {
  /// ptr mut of dQuaternion
  fn as_ptr_mut(&mut self) -> *mut dReal;
  /// ptr of dQuaternion (use for converter)
  fn as_ptr(&self) -> *const dReal;

  /// construct as Identity
  fn new() -> dQuaternion {
    let mut q: dQuaternion = [0.0; 4];
unsafe {
    dQSetIdentity(q.as_ptr_mut());
}
    q
  }

  /// constructor (converter)
  fn from_R(m: dMatrix3) -> dQuaternion {
    let mut q: dQuaternion = [0.0; 4];
unsafe {
    dQfromR(q.as_ptr_mut(), m.as_ptr() as *mut dReal);
}
    q
  }

  /// converter (like as dMatrix3::from_Q(*self))
  fn to_R(&self) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRfromQ(m.as_ptr_mut(), self.as_ptr() as *mut dReal);
}
    m
  }

  /// constructor
  fn from_axis_and_angle(axis: [dReal; 3], angle: dReal) -> dQuaternion {
    let mut q: dQuaternion = [0.0; 4];
unsafe {
    dQFromAxisAndAngle(q.as_ptr_mut(), axis[0], axis[1], axis[2], angle);
}
    q
  }
}

impl Quaternion for dQuaternion {
  /// ptr mut of dQuaternion
  fn as_ptr_mut(&mut self) -> *mut dReal { &mut (*self)[0] as *mut dReal }
  /// ptr of dQuaternion (use for converter)
  fn as_ptr(&self) -> *const dReal { &(*self)[0] as *const dReal }
}

/// constructor and converter for primitive type
pub trait Matrix3 {
  /// ptr mut of dMatrix3
  fn as_ptr_mut(&mut self) -> *mut dReal;
  /// ptr of dMatrix3 (use for converter)
  fn as_ptr(&self) -> *const dReal;

  /// construct as Identity
  fn new() -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRSetIdentity(m.as_ptr_mut());
}
    m
  }

  /// constructor (converter)
  fn from_Q(q: dQuaternion) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRfromQ(m.as_ptr_mut(), q.as_ptr() as *mut dReal);
}
    m
  }

  /// converter (like as dQuaternion::from_R(*self))
  fn to_Q(&self) -> dQuaternion {
    let mut q: dQuaternion = [0.0; 4];
unsafe {
    dQfromR(q.as_ptr_mut(), self.as_ptr() as *mut dReal);
}
    q
  }

  /// constructor
  fn from_axis_and_angle(axis: [dReal; 3], angle: dReal) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRFromAxisAndAngle(m.as_ptr_mut(), axis[0], axis[1], axis[2], angle);
}
    m
  }

  /// constructor
  fn from_euler_angles(phi: dReal, theta: dReal, psi: dReal) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRFromEulerAngles(m.as_ptr_mut(), phi, theta, psi);
}
    m
  }

  /// constructor
  fn from_2_axes(e0: [dReal; 3], e1: [dReal; 3]) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRFrom2Axes(m.as_ptr_mut(), e0[0], e0[1], e0[2], e1[0], e1[1], e1[2]);
}
    m
  }

  /// constructor
  fn from_z_axis(e: [dReal; 3]) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dRFromZAxis(m.as_ptr_mut(), e[0], e[1], e[2]);
}
    m
  }
}

impl Matrix3 for dMatrix3 {
  /// ptr mut of dMatrix3
  fn as_ptr_mut(&mut self) -> *mut dReal { &mut (*self)[0] as *mut dReal }
  /// ptr of dMatrix3 (use for converter)
  fn as_ptr(&self) -> *const dReal { &(*self)[0] as *const dReal }
}

/// constructor and converter for primitive type
pub trait Matrix4 {
  /// construct as Identity
  fn new() -> dMatrix4 {
    let mut m: dMatrix4 = [0.0; 16];
    for i in 0..4 { m[i * 5] = 1.0; }
    m
  }
}

impl Matrix4 for dMatrix4 {
}

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
