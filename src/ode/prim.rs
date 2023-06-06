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

/// static angle 180
pub static PI: dReal = 3.14159265;
/// static angle 360 dual
pub static PId: dReal = PI * 2.0;
/// static angle 90 half
pub static PIh: dReal = PI / 2.0;
/// static angle 60 regular triangle
pub static PIt: dReal = PI / 3.0;
/// static angle 45 quarter
pub static PIq: dReal = PI / 4.0;
/// static angle 30 a sixth
pub static PIx: dReal = PI / 6.0;

/// static angle 270
pub static PIh3: dReal = PIh * 3.0;

/// static angle 120
pub static PIt2: dReal = PIt * 2.0;
/// static angle 240
pub static PIt4: dReal = PIt * 4.0;
/// static angle 300
pub static PIt5: dReal = PIt * 5.0;

/// static angle 135
pub static PIq3: dReal = PIq * 3.0;

/// static angle 150
pub static PIx5: dReal = PIx * 5.0;

/// static dMatrix4 Identity
pub static M4I: dMatrix4 = [
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 1.0, 0.0,
  0.0, 0.0, 0.0, 1.0];

/// static dMatrix3 Identity
pub static M3I: dMatrix3 = [
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 1.0, 0.0];

/// static dQuaternion Identity
pub static QI: dQuaternion = [1.0, 0.0, 0.0, 0.0];

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
  /// axis [x, y, z] should be norm == 1, angle is theta radian
  /// Q(axis, angle) == [cos(t/2), xsin(t/2), ysin(t/2), zsin(t/2)]
  fn from_axis_and_angle(axis: [dReal; 3], angle: dReal) -> dQuaternion {
    let mut q: dQuaternion = [0.0; 4];
unsafe {
    dQFromAxisAndAngle(q.as_ptr_mut(), axis[0], axis[1], axis[2], angle);
}
    q
  }

  /// constructor multiply p: dQuaternion and q: dQuaternion
  fn multiply0(p: dQuaternion, q: dQuaternion) -> dQuaternion {
    dQuaternion::multiply0_pp(p.as_ptr(), q.as_ptr())
  }

  /// constructor multiply p: dQuaternion pointer and q: dQuaternion pointer
  fn multiply0_pp(p: *const dReal, q: *const dReal) -> dQuaternion {
    let mut o: dQuaternion = [0.0; 4];
unsafe {
    dQMultiply0(o.as_ptr_mut(), p as *mut dReal, q as *mut dReal);
}
    o
  }

  /// constructor multiply m: dMatrix3 and v: dVector3
  /// dVector3::multiply0_331 is defined as dQuaternion::multiply0_331
  fn multiply0_331(m: dMatrix3, v: dVector3) -> dVector3 {
    dVector3::multiply0_331_pp(m.as_ptr(), v.as_ptr())
  }

  /// constructor multiply m: dMatrix3 pointer and v: dVector3 pointer
  /// dVector3::multiply0_331_pp is defined as dQuaternion::multiply0_331_pp
  fn multiply0_331_pp(m: *const dReal, v: *const dReal) -> dVector3 {
    let mut o: dVector3 = [0.0; 4];
unsafe {
    dMULTIPLY0_331(o.as_ptr_mut(), m, v);
}
    o
  }

  /// constructor multiply m: dMatrix4 and v: dVector4
  /// dVector4::multiply0_441 is defined as dQuaternion::multiply0_441
  fn multiply0_441(m: dMatrix4, v: dVector4) -> dVector4 {
    dVector4::multiply0_441_pp(m.as_ptr(), v.as_ptr())
  }

  /// constructor multiply m: dMatrix4 pointer and v: dVector4 pointer
  /// dVector4::multiply0_441_pp is defined as dQuaternion::multiply0_441_pp
  fn multiply0_441_pp(m: *const dReal, v: *const dReal) -> dVector4 {
    let mut o: dVector4 = [0.0; 4];
unsafe {
    dMULTIPLY0_441(o.as_ptr_mut(), m, v);
}
    o
  }

  /// conjugate dQuaternion
  fn conjugate(q: dQuaternion) -> dQuaternion {
    dQuaternion::conjugate_ptr(q.as_ptr())
  }

  /// conjugate dQuaternion pointer
  fn conjugate_ptr(q: *const dReal) -> dQuaternion {
unsafe {
    let q = std::slice::from_raw_parts(q, 4);
    [q[0], -q[1], -q[2], -q[3]]
}
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

  /// constructor (transpose illegular order)
  fn t(m: dMatrix3) -> dMatrix3 {
    dMatrix3::t_ptr(m.as_ptr())
  }

  /// constructor (transpose illegular order) pointer
  fn t_ptr(m: *const dReal) -> dMatrix3 {
unsafe {
    let m = std::slice::from_raw_parts(m, 12);
    [m[0], m[4], m[8], m[3],
     m[1], m[5], m[9], m[7],
     m[2], m[6], m[10], m[11]]
}
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

  /// constructor multiply a: dMatrix3 and b: dMatrix3
  fn multiply0_333(a: dMatrix3, b: dMatrix3) -> dMatrix3 {
    dMatrix3::multiply0_333_pp(a.as_ptr(), b.as_ptr())
  }

  /// constructor multiply a: dMatrix3 pointer and b: dMatrix3 pointer
  fn multiply0_333_pp(a: *const dReal, b: *const dReal) -> dMatrix3 {
    let mut m: dMatrix3 = [0.0; 12];
unsafe {
    dMULTIPLY0_333(m.as_ptr_mut(), a, b);
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
  /// ptr mut of dMatrix4
  fn as_ptr_mut(&mut self) -> *mut dReal;
  /// ptr of dMatrix4 (use for converter)
  fn as_ptr(&self) -> *const dReal;

  /// construct as Identity
  fn new() -> dMatrix4 {
    let mut m: dMatrix4 = [0.0; 16];
    for i in 0..4 { m[i * 5] = 1.0; }
    m
  }

  /// constructor (transpose)
  fn t(m: dMatrix4) -> dMatrix4 {
    dMatrix4::t_ptr(m.as_ptr())
  }

  /// constructor (transpose) pointer
  fn t_ptr(m: *const dReal) -> dMatrix4 {
unsafe {
    let m = std::slice::from_raw_parts(m, 16);
    [m[0], m[4], m[8], m[12],
     m[1], m[5], m[9], m[13],
     m[2], m[6], m[10], m[14],
     m[3], m[7], m[11], m[15]]
}
  }

  /// constructor (converter) q: dQuaternion
  fn Hermitian_Q(q: dQuaternion) -> dMatrix4 {
    dMatrix4::t(dMatrix4::from_Q(dQuaternion::conjugate(q)))
  }

  /// constructor (converter) q: dQuaternion pointer
  fn Hermitian_Q_ptr(q: *const dReal) -> dMatrix4 {
    dMatrix4::t(dMatrix4::from_Q(dQuaternion::conjugate_ptr(q)))
  }

  /// constructor (converter) q: dQuaternion
  fn from_Q(q: dQuaternion) -> dMatrix4 {
    dMatrix4::from_Q_ptr(q.as_ptr())
  }

  /// constructor (converter) q: dQuaternion pointer
  fn from_Q_ptr(q: *const dReal) -> dMatrix4 {
unsafe {
    let q = std::slice::from_raw_parts(q, 4);
    [ q[0], -q[1], -q[2], -q[3],
      q[1],  q[0], -q[3],  q[2],
      q[2],  q[3],  q[0], -q[1],
      q[3], -q[2],  q[1],  q[0]]
}
  }

  /// constructor multiply a: dMatrix4 and b: dMatrix4
  fn multiply0_444(a: dMatrix4, b: dMatrix4) -> dMatrix4 {
    dMatrix4::multiply0_444_pp(a.as_ptr(), b.as_ptr())
  }

  /// constructor multiply a: dMatrix4 pointer and b: dMatrix4 pointer
  fn multiply0_444_pp(a: *const dReal, b: *const dReal) -> dMatrix4 {
    let mut m: dMatrix4 = [0.0; 16];
unsafe {
    dMULTIPLY0_444(m.as_ptr_mut(), a, b);
}
    m
  }
}

impl Matrix4 for dMatrix4 {
  /// ptr mut of dMatrix4
  fn as_ptr_mut(&mut self) -> *mut dReal { &mut (*self)[0] as *mut dReal }
  /// ptr of dMatrix4 (use for converter)
  fn as_ptr(&self) -> *const dReal { &(*self)[0] as *const dReal }
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
