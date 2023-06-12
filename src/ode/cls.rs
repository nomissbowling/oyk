//! cls
//!

use crate::ode::*;

pub mod obg;

/// GeomOffset
pub struct GeomOffset<'a> {
  /// geom sub
  pub gsub: dGeomID,
  /// offset
  pub o: &'a dVector3
}

/// use for Vec&lt;T&gt;
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
