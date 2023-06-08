//! mat
//!

use crate::ode::*;

use std::fmt;

/// mat as slice
#[derive(Debug)]
pub struct ODEMat<'a> {
  nr: usize,
  nc: usize,
  mat: &'a [dReal]
}

/// mat as slice
impl ODEMat<'_> {
  /// construct
  pub fn from_slice(nr: usize, nc: usize, mat: &[dReal]) -> ODEMat {
    ODEMat{nr: nr, nc: nc, mat: mat}
  }

  /// construct
  pub fn as_vec(mat: &[dReal]) -> ODEMat {
    ODEMat::from_slice(1, 4, mat)
  }

  /// construct
  pub fn as_mat(nr: usize, mat: &[dReal]) -> ODEMat {
    ODEMat::from_slice(nr, 4, mat)
  }

  /// construct
  pub fn from_Q(q: *const dReal) -> ODEMat<'static> {
unsafe {
    ODEMat::as_vec(std::slice::from_raw_parts(q, 4))
}
  }

  /// construct
  pub fn from_Mat3(m: *const dReal) -> ODEMat<'static> {
unsafe {
    ODEMat::as_mat(3, std::slice::from_raw_parts(m, 12))
}
  }

  /// construct
  pub fn from_Mat4(m: *const dReal) -> ODEMat<'static> {
unsafe {
    ODEMat::as_mat(4, std::slice::from_raw_parts(m, 16))
}
  }
}

/// mat formatter
impl fmt::Display for ODEMat<'_> {
  /// mat formatter
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.nr {
      1 => {
        write!(f, "[");
        for (j, col) in self.mat.iter().enumerate() {
          if j != 0 { write!(f, ","); }
          write!(f, "{:17.7}", col);
        }
        write!(f, "]")
      },
      _ => {
        writeln!(f, "[");
        for (i, row) in self.mat.chunks_exact(self.nc).enumerate() {
          write!(f, " {}", ODEMat::as_vec(row));
          if i < self.nr - 1 { writeln!(f, ""); };
        }
        write!(f, "]")
      }
    }
  }
}

// std::convert::TryInto
/// construct array [T; N] (dVector3 dMatrix3 etc) from vec![]
pub fn v2a<T, const N: usize>(v: Vec<T>) -> [T; N] {
  v.try_into().unwrap_or_else(|v: Vec<T>|
    panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
