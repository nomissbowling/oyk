//! trimeshconvex
//!
//! field               TMVTX   INDICES PLANES  VTX     POLYGONS
//! elems                 (*3)    (*3)    (*4)    (*3)     *(1+n) n=3,4,5,6
//! trimesh_custom                4*3     4*4     4*3     4*(1+3)
//! trimesh_tetra                 4*3     4*4     4*3     4*(1+3)
//! trimesh_cube                 12*3     6*4     8*3     6*(1+4)
//! trimesh_icosahedron          80*3    80*4    42*3    80*(1+3)
//! trimesh_bunny       453*3   902*3   176*4   105*3   176*(1+3)
//! field                               FACES
//! FACES (Normal4) will be reset by RecalcFaces()
//! The number of POLYGONS is same as FACES (but not same number of words)
//!

use crate::ode::*;
use crate::ode::cls::*;
use crate::ode::krp::*;
use crate::ode::meta::*;

/// constructor trimeshvi
pub trait TriMesh {
  /// constructor trimeshvi
  fn new(vtx: &mut Vec<dReal>, indices: &mut Vec<dTriIndex>) -> trimeshvi {
unsafe {
    trimeshvi{
      vtxCount: (vtx.len() / 3) as u32,
      vtx: vtx.as_ptr_mut(),
      indices: indices.as_ptr_mut(),
      indexCount: indices.len() as u32}
}
  }

  /// as slice vtx
  fn as_slice_vtx(&self) -> &[dReal];
  /// as slice indices
  fn as_slice_indices(&self) -> &[dTriIndex];
}

impl TriMesh for trimeshvi {
  /// as slice vtx
  fn as_slice_vtx(&self) -> &[dReal] {
unsafe {
    std::slice::from_raw_parts(self.vtx, self.vtxCount as usize * 3)
}
  }

  /// as slice indices
  fn as_slice_indices(&self) -> &[dTriIndex] {
unsafe {
    std::slice::from_raw_parts(self.indices, self.indexCount as usize)
}
  }
}

/// constructor convexfvp
pub trait Convex {
  /// constructor convexfvp
  fn new(planes: &mut Vec<dReal>, vtx: &mut Vec<dReal>,
    polygons: &mut Vec<u32>) -> convexfvp {
unsafe {
    convexfvp{
      faceCount: (planes.len() / 4) as u32,
      faces: planes.as_ptr_mut(),
      vtxCount: (vtx.len() / 3) as u32,
      vtx: vtx.as_ptr_mut(),
      polygons: polygons.as_ptr_mut()}
}
  }

  /// as slice faces
  fn as_slice_faces(&self) -> &[dReal];
  /// as slice vtx
  fn as_slice_vtx(&self) -> &[dReal];
  /// as slice polygons
  fn as_slice_polygons(&self) -> &[dTriIndex];
}

impl Convex for convexfvp {
  /// as slice faces
  fn as_slice_faces(&self) -> &[dReal] {
unsafe {
    std::slice::from_raw_parts(self.faces, self.faceCount as usize * 4)
}
  }

  /// as slice vtx
  fn as_slice_vtx(&self) -> &[dReal] {
unsafe {
    std::slice::from_raw_parts(self.vtx, self.vtxCount as usize * 3)
}
  }

  /// as slice polygons
  fn as_slice_polygons(&self) -> &[dTriIndex] {
unsafe {
    let mut total: usize = 0;
    for _ in 0..self.faceCount {
      let p = std::slice::from_raw_parts(self.polygons, total + 1);
      total += p[total] as usize + 1;
    }
    std::slice::from_raw_parts(self.polygons, total)
}
  }
}

pub mod trimesh_tetra;
use trimesh_tetra::*;
pub use trimesh_tetra::{tmv_tetra, fvp_tetra};

pub mod trimesh_cube;
use trimesh_cube::*;
pub use trimesh_cube::{tmv_cube, fvp_cube};

pub mod trimesh_icosahedron;
use trimesh_icosahedron::*;
pub use trimesh_icosahedron::{tmv_icosahedron, fvp_icosahedron};

pub mod trimesh_bunny;
use trimesh_bunny::*;
pub use trimesh_bunny::{tmv_bunny, fvp_bunny};

pub mod trimesh_custom;
use trimesh_custom::*;
pub use trimesh_custom::{tmv_custom, fvp_custom};

/// always new MetaTriMesh rescale
pub fn CvtMetaTriMeshFromConvex(mc: &MetaConvex, sc: dReal) ->
  Box<MetaTriMesh> {
  MetaTriMesh::new(true, mc.dm,
    unsafe { CvtTriMeshVIFromConvexFVP(mc.fvp, sc) },
    mc.krp, mc.tcm.tex, mc.tcm.col)
}

/// always new MetaConvex rescale
pub fn CvtMetaConvexFromTriMesh(mt: &MetaTriMesh, sc: dReal) ->
  Box<MetaConvex> {
  MetaConvex::new(true, mt.dm,
    unsafe { CvtConvexFVPFromTriMeshVI(mt.tmv, sc) },
    mt.krp, mt.tcm.tex, mt.tcm.col)
}

/// new and copy MetaTriMesh rescale
pub fn NewCopyMetaTriMesh(src: &MetaTriMesh, sc: dReal) -> Box<MetaTriMesh> {
  MetaTriMesh::new(true, src.dm,
    unsafe { CopyTriMeshVI(0 as *mut trimeshvi, src.tmv, sc) },
    src.krp, src.tcm.tex, src.tcm.col)
}

/// overwrite copy MetaTriMesh rescale
pub fn CopyMetaTriMesh(dst: &mut MetaTriMesh, src: &MetaTriMesh, sc: dReal) {
  // dst.ff = dst.ff;
  dst.dm = src.dm;
  dst.tmv = unsafe { CopyTriMeshVI(dst.tmv, src.tmv, sc) };
  dst.krp = src.krp;
  dst.tcm = src.tcm.clone();
}

/// create geom trimeshvi
pub fn CreateGeomTrimeshFromVI(space: dSpaceID, v: *mut trimeshvi) -> dGeomID {
unsafe {
  let tmv: &trimeshvi = &*v;
  let tmd: dTriMeshDataID = dGeomTriMeshDataCreate();
  dGeomTriMeshDataBuildDouble(tmd,
    tmv.vtx as *const c_void,
    3 * std::mem::size_of::<dReal>() as i32, tmv.vtxCount as i32,
    tmv.indices as *const c_void,
    tmv.indexCount as i32, 3 * std::mem::size_of::<dTriIndex>() as i32);
  dGeomTriMeshDataPreprocess2(tmd,
    (1u32 << dTRIDATAPREPROCESS_BUILD_FACE_ANGLES), 0 as *const i64);
  let geom: dGeomID = dCreateTriMesh(space, tmd, None, None, None);
  // do not release here
  dGeomSetData(geom, tmd as *mut c_void); // if tmd { dGeomTriMeshDataDestroy(tmd); }
  geom
}
}

/// new and copy MetaConvex rescale
pub fn NewCopyMetaConvex(src: &MetaConvex, sc: dReal) -> Box<MetaConvex> {
  MetaConvex::new(true, src.dm,
    unsafe { CopyConvexFVP(0 as *mut convexfvp, src.fvp, sc) },
    src.krp, src.tcm.tex, src.tcm.col)
}

/// overwrite copy MetaConvex rescale
pub fn CopyMetaConvex(dst: &mut MetaConvex, src: &MetaConvex, sc: dReal) {
  // dst.ff = dst.ff;
  dst.dm = src.dm;
  dst.fvp = unsafe { CopyConvexFVP(dst.fvp, src.fvp, sc) };
  dst.krp = src.krp;
  dst.tcm = src.tcm.clone();
}

/// create geom convexfvp
pub fn CreateGeomConvexFromFVP(space: dSpaceID, v: *mut convexfvp) -> dGeomID {
unsafe {
  let fvp: &convexfvp = &*v;
  dCreateConvex(space,
    fvp.faces, fvp.faceCount, fvp.vtx, fvp.vtxCount, fvp.polygons)
}
}

/// set mass calced as Trimesh to the geom that has Convex
pub fn MassSetConvexAsTrimesh(m: &mut dMass, density: dReal,
  fvp: *mut convexfvp) {
unsafe {
  let tmv: *mut trimeshvi = CvtTriMeshVIFromConvexFVP(fvp, 1.0);
  let gt: dGeomID = CreateGeomTrimeshFromVI(0 as dSpaceID, tmv);
  dMassSetTrimesh(m, density, gt);
  dGeomDestroy(gt); // *** inner called dGeomTriMeshDataDestroy() or not ***
  FreeTriMeshVI(tmv, true);
}
}
