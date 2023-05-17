//! colors
//!

use crate::ode::*;

use once_cell::sync::Lazy;

/// unsafe static COLORS (reference to preset colors)
pub static COLORS: Lazy<Vec<u32>> = Lazy::new(|| vec![
  0xcccccccc, 0xcc9933cc, 0x33cc99cc, 0x9933cccc,
  0x3399cccc, 0x99cc33cc, 0xcc3399cc, 0x999999cc,
  0x666666cc, 0x996633cc, 0x339966cc, 0x663399cc,
  0x336699cc, 0x669933cc, 0x993366cc, 0x333333cc]);

/// u32 RGBA (little endian) to dVector4 color
pub fn vec4_from_u32(col: u32) -> dVector4 {
  let p: usize = &col as *const u32 as usize;
  mat::v2a((0..4).into_iter().map(|j|
unsafe {
    *((p + (3 - j)) as *const u8) as dReal / 255.0 // little endian
}
  ).collect())
}
