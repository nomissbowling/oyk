/// bridge.hpp

#ifndef __BRIDGE_H__
#define __BRIDGE_H__

/**
 * Bridge for cpp
 */
class Bridge {
protected:
  char *str;
public:
  /// construct
  Bridge();
  /// construct with ptr
  Bridge(char *s);
  /// destruct
  virtual ~Bridge();
  /// assign ptr
  void pset(char *p);
  /// display
  void put();
};

extern "C" {
/// legacy C interface
void bput();
}

extern "C" {
/// adhoc C macro and static __inline functions

/// dReal as f64 (defined in ode.hpp)
typedef double dReal;
/// dTriIndex as u32 (defined in ode.hpp)
typedef unsigned int dTriIndex;

/// res = a(&dMatrix3) b(&dVector3)
void dMULTIPLY0_331(dReal *res, const dReal *a, const dReal *b);
/// res = a(&dMatrix3) b(&dMatrix3)
void dMULTIPLY0_333(dReal *res, const dReal *a, const dReal *b);

/// res = a(&dMatrix4) b(&dVector4 or &dQuaternion)
void dMULTIPLY0_441(dReal *res, const dReal *a, const dReal *b);
/// res = a(&dMatrix4) b(&dMatrix4)
void dMULTIPLY0_444(dReal *res, const dReal *a, const dReal *b);

/// TriMeshVI
struct trimeshvi {
  /// number of vertices
  unsigned int vtxCount;
  /// vertices
  dReal *vtx;
  /// indices
  dTriIndex *indices;
  /// number of indices (all dTriIndex elements)
  unsigned int indexCount;
};

/// ConvexFVP
struct convexfvp {
  /// number of planes
  unsigned int faceCount;
  /// planes
  dReal *faces;
  /// number of vertices
  unsigned int vtxCount;
  /// vertices
  dReal *vtx;
  /// polygons
  unsigned int *polygons;
};

/// (defined in ode.hpp)
dReal dDot(const dReal *a, const dReal *b, int n);

/// set static in cpp
void SetScaleLimit(dReal sclim);

/// c[3] = a[3] x b[3] same as void dCROSS(c, =, a, b);
void Cross3(dReal *c, dReal *a, dReal *b);
/// n[4] = normal(v[9])
void Normal4(dReal *n, dReal *v);
/// recalc triangle convexfvp (set normal of faces)
void RecalcFaces(convexfvp *fvp);

/// delete vtx, indices when ff is true
void FreeTriMeshVI(trimeshvi *tmv, bool ff);
// void FreeMetaTriMesh(metatrimesh *mt); // to Rust Drop

/// delete faces, vtx, polygons when ff is true
void FreeConvexFVP(convexfvp *fvp, bool ff);
// void FreeMetaConvex(metaconvex *mc); // to Rust Drop

/// always new trimeshvi rescale and return it
trimeshvi *CvtTriMeshVIFromConvexFVP(convexfvp *fvp, dReal sc);
// metatrimesh *CvtMetaTriMeshFromConvex(metaconvex *mc, dReal sc); // to Rust

/// always new convexfvp rescale and return it
convexfvp *CvtConvexFVPFromTriMeshVI(trimeshvi *tmv, dReal sc);
// metaconvex *CvtMetaConvexFromTriMesh(metatrimesh *mt, dReal sc); // to Rust

/// overwrite trimeshvi rescale and return it
trimeshvi *ScaleTriMeshVI(trimeshvi *tmv, dReal sc);
/// (dst is NULL: new, !NULL: overwrite) trimeshvi rescale and return it
trimeshvi *CopyTriMeshVI(trimeshvi *dst, trimeshvi *src, dReal sc);
// metatrimesh *CopyMetaTriMesh(
//   metatrimesh *dst, metatrimesh *src, dReal sc); // to Rust (NewCopy, Copy)

// dGeomID CreateGeomTrimeshFromVI(dSpaceID space, trimeshvi *tmv); // to Rust
// dBodyID CreateTrimeshFromVI(dWorldID world, dSpaceID space,
//   const char *key, metatrimesh *mt); // (merge into above and creator_dm)

/// overwrite convexfvp rescale and return it
convexfvp *ScaleConvexFVP(convexfvp *fvp, dReal sc);
/// (dst is NULL: new, !NULL: overwrite) convexfvp rescale and return it
convexfvp *CopyConvexFVP(convexfvp *dst, convexfvp *src, dReal sc);
// metaconvex *CopyMetaConvex(
//   metaconvex *dst, metaconvex *src, dReal sc); // to Rust (NewCopy, Copy)

// dGeomID CreateGeomConvexFromFVP(dSpaceID space, convexfvp *fvp); // to Rust
// dBodyID CreateConvexFromFVP(dWorldID world, dSpaceID space,
//   const char *key, metaconvex *mc); // (merge into above and creator_dm)

// void _MassSetConvexAsTrimesh(dMass *m, dReal density, dGeomID g); // to Rust
}

#endif // __BRIDGE_H__
