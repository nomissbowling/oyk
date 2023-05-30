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

void FreeTriMeshVI(trimeshvi *tmv);
//void FreeMetaTriMesh(metatrimesh *mt);

void FreeConvexFVP(convexfvp *fvp);
//void FreeMetaConvex(metaconvex *mc);

trimeshvi *CvtTriMeshVIFromConvexFVP(convexfvp *fvp, dReal sc);
//metatrimesh *CvtMetaTriMeshFromConvex(metaconvex *mc, dReal sc);

convexfvp *CvtConvexFVPFromTriMeshVI(trimeshvi *tmv, dReal sc);
//metaconvex *CvtMetaConvexFromTriMesh(metatrimesh *mt, dReal sc);

trimeshvi *ScaleTriMeshVI(trimeshvi *tmv, dReal sc);
trimeshvi *CopyTriMeshVI(trimeshvi *dst, trimeshvi *src, dReal sc);
//metatrimesh *CopyMetaTriMesh(
//  metatrimesh *dst, metatrimesh *src, dReal sc);
/*
dGeomID CreateGeomTrimeshFromVI(dSpaceID space, trimeshvi *tmv);
//dBodyID CreateTrimeshFromVI(dWorldID world, dSpaceID space,
//  const char *key, metatrimesh *mt);
*/
convexfvp *ScaleConvexFVP(convexfvp *fvp, dReal sc);
convexfvp *CopyConvexFVP(convexfvp *dst, convexfvp *src, dReal sc);
//metaconvex *CopyMetaConvex(
//  metaconvex *dst, metaconvex *src, dReal sc);
/*
dGeomID CreateGeomConvexFromFVP(dSpaceID space, convexfvp *fvp);
//dBodyID CreateConvexFromFVP(dWorldID world, dSpaceID space,
//  const char *key, metaconvex *mc);
*/
//void _MassSetConvexAsTrimesh(dMass *m, dReal density, dGeomID g);

}

#endif // __BRIDGE_H__
