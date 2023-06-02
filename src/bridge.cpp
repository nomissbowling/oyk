/// bridge.cpp

#include <iomanip>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <map>

#include <cstring>
#include <cmath>

using namespace std;

#include <bridge.hpp>

/// construct
Bridge::Bridge() : str("_bridge")
{
}

/// construct with ptr
Bridge::Bridge(char *s) : str(s)
{
}

/// destruct
Bridge::~Bridge()
{
}

/// assign ptr
void Bridge::pset(char *p)
{
  str = p;
}

/// display
void Bridge::put()
{
  cout << string(str) << endl;
}

/// legacy C interface
void bput()
{
  Bridge b = Bridge("bridge");
  b.put();
}

// adhoc C macro and static __inline functions

static __inline dReal _dCalcVectorDot3(const dReal *a, const dReal *b, unsigned step_a, unsigned step_b)
{
  return a[0] * b[0] + a[step_a] * b[step_b] + a[2 * step_a] * b[2 * step_b];
}

static __inline dReal dCalcVectorDot3 (const dReal *a, const dReal *b) { return _dCalcVectorDot3(a,b,1,1); }

static __inline void dMultiplyHelper0_331(dReal *res, const dReal *a, const dReal *b)
{
  const dReal res_0 = dCalcVectorDot3(a, b);
  const dReal res_1 = dCalcVectorDot3(a + 4, b);
  const dReal res_2 = dCalcVectorDot3(a + 8, b);

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dMultiply0_331(dReal *res, const dReal *a, const dReal *b)
{
  dMultiplyHelper0_331(res, a, b);
}

/// res = a(&dMatrix3) b(&dVector3)
void dMULTIPLY0_331(dReal *res, const dReal *a, const dReal *b)
{
  dMultiply0_331(res, a, b);
}

static __inline dReal dCalcVectorDot3_41 (const dReal *a, const dReal *b) { return _dCalcVectorDot3(a,b,4,1); }

static __inline void dMultiplyHelper1_331(dReal *res, const dReal *a, const dReal *b)
{
  const dReal res_0 = dCalcVectorDot3_41(a, b);
  const dReal res_1 = dCalcVectorDot3_41(a + 1, b);
  const dReal res_2 = dCalcVectorDot3_41(a + 2, b);

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dMultiplyHelper0_133(dReal *res, const dReal *a, const dReal *b)
{
  dMultiplyHelper1_331(res, b, a);
}

static __inline void dMultiply0_333(dReal *res, const dReal *a, const dReal *b)
{
  dMultiplyHelper0_133(res + 0, a + 0, b);
  dMultiplyHelper0_133(res + 4, a + 4, b);
  dMultiplyHelper0_133(res + 8, a + 8, b);
}

/// res = a(&dMatrix3) b(&dMatrix3)
void dMULTIPLY0_333(dReal *res, const dReal *a, const dReal *b)
{
  dMultiply0_333(res, a, b);
}

// trimeshconvex
static dReal scale_min_limit = 1e-5;

inline int isRescale(dReal sc){
  dReal f = sc - 1.0;
  return (f < -scale_min_limit) || (f > scale_min_limit);
}

void SetScaleLimit(dReal sclim)
{
  scale_min_limit = sclim;
}

// same as void dCROSS(c, =, a, b);
void Cross3(dReal *c, dReal *a, dReal *b) // c[3] = a[3] x b[3]
{
  c[0] = a[1] * b[2] - a[2] * b[1];
  c[1] = a[2] * b[0] - a[0] * b[2];
  c[2] = a[0] * b[1] - a[1] * b[0];
}

void Normal4(dReal *n, dReal *v) // n[4] = normal(v[9])
{
  dReal a[] = {v[3] - v[0], v[4] - v[1], v[5] - v[2]};
  dReal b[] = {v[6] - v[0], v[7] - v[1], v[8] - v[2]};
  Cross3(n, a, b);
  // same as dNormalize3(n);
  dReal r = sqrt(n[0] * n[0] + n[1] * n[1] + n[2] * n[2]);
  for(int j = 0; j < 3; ++j) n[j] /= r;
  // plane n[0]vx + n[1]vy + n[2]vz = n[3]
  n[3] = dDot(n, v, 3);
}

void RecalcFaces(convexfvp *fvp)
{
  // set normal of faces
  dReal *vtx = fvp->vtx;
  unsigned int *p = fvp->polygons;
  for(int i = 0; i < fvp->faceCount; ++i){
    if(*p++ != 3) throw runtime_error("can't recalc not triangle convexfvp");
    unsigned int idx[] = {*p++, *p++, *p++};
    dReal v[] = {
      vtx[idx[0] * 3 + 0], vtx[idx[0] * 3 + 1], vtx[idx[0] * 3 + 2],
      vtx[idx[1] * 3 + 0], vtx[idx[1] * 3 + 1], vtx[idx[1] * 3 + 2],
      vtx[idx[2] * 3 + 0], vtx[idx[2] * 3 + 1], vtx[idx[2] * 3 + 2]};
    Normal4(&fvp->faces[i * 4], v); // get plane
  }
}

void FreeTriMeshVI(trimeshvi *tmv, bool ff)
{
  if(!tmv) return;
  if(ff){
    if(tmv->vtx) delete[] tmv->vtx;
    if(tmv->indices) delete[] tmv->indices;
  }
  delete tmv;
}

void FreeConvexFVP(convexfvp *fvp, bool ff)
{
  if(!fvp) return;
  if(ff){
    if(fvp->faces) delete[] fvp->faces;
    if(fvp->vtx) delete[] fvp->vtx;
    if(fvp->polygons) delete[] fvp->polygons;
  }
  delete fvp;
}

trimeshvi *CvtTriMeshVIFromConvexFVP(convexfvp *fvp, dReal sc)
{
  trimeshvi *tmv = new trimeshvi;
  if(!tmv) throw runtime_error("can't convert to trimeshvi");
  tmv->vtxCount = fvp->vtxCount;
  tmv->vtx = new dReal[3 * tmv->vtxCount];
  if(!tmv->vtx) throw runtime_error("can't convert to trimeshvi.vtx");
  memcpy(tmv->vtx, fvp->vtx, sizeof(dReal) * 3 * tmv->vtxCount);
  if(isRescale(sc)) ScaleTriMeshVI(tmv, sc);
  // get triangles count
  size_t tcnt = 0;
  unsigned int *s = fvp->polygons;
  for(int i = 0; i < fvp->faceCount; ++i){
    unsigned int n = *s++;
    if(n <= 2) throw runtime_error("can't convert to trimeshvi polygon <= 2");
    s += n;
    tcnt += (n - 2); // triangles in the polygon
  }
  tmv->indexCount = 3 * tcnt;
  tmv->indices = new dTriIndex[tmv->indexCount];
  if(!tmv->indices) throw runtime_error("can't convert to trimeshvi.indices");
  // set triangle indices
  dTriIndex *p = tmv->indices;
  s = fvp->polygons;
  for(int i = 0; i < fvp->faceCount; ++i){
    unsigned int n = *s++;
    for(int t = 0; t < (n - 2); ++t){ // triangle number
      *p++ = (dTriIndex)*s++; // [0] [2] [3] [4] ...
      *p++ = (dTriIndex)*s; // [1] [3] [4] [5] ...
      if(!t) *p++ = (dTriIndex)*++s; // [2]
      else *p++ = (dTriIndex)*(s - 2 - t); // [0]
    }
    ++s;
  }
  return tmv;
}

convexfvp *CvtConvexFVPFromTriMeshVI(trimeshvi *tmv, dReal sc)
{
  convexfvp *fvp = new convexfvp;
  if(!fvp) throw runtime_error("can't convert to convexfvp");
  fvp->faceCount = tmv->indexCount / 3;
  fvp->faces = new dReal[4 * fvp->faceCount];
  if(!fvp->faces) throw runtime_error("can't convert to convexfvp.faces");
  fvp->vtxCount = tmv->vtxCount;
  fvp->vtx = new dReal[3 * fvp->vtxCount];
  if(!fvp->vtx) throw runtime_error("can't convert to convexfvp.vtx");
  memcpy(fvp->vtx, tmv->vtx, sizeof(dReal) * 3 * fvp->vtxCount);
  if(isRescale(sc)) ScaleConvexFVP(fvp, sc);
  fvp->polygons = new unsigned int[4 * fvp->faceCount];
  if(!fvp->polygons) throw runtime_error("can't convert to convexfvp.polygons");
  // set triangle indices
  dTriIndex *s = tmv->indices;
  unsigned int *p = fvp->polygons;
  for(int i = 0; i < fvp->faceCount; ++i){
    *p++ = 3;
    for(int j = 0; j < 3; ++j) *p++ = (unsigned int)*s++;
  }
  RecalcFaces(fvp);
  return fvp;
}

trimeshvi *ScaleTriMeshVI(trimeshvi *tmv, dReal sc)
{
  for(int i = 0; i < 3 * tmv->vtxCount; ++i) tmv->vtx[i] *= sc;
  return tmv;
}

trimeshvi *CopyTriMeshVI(trimeshvi *dst, trimeshvi *src, dReal sc)
{
#if 0
  cout << "CopyTriMeshVI: " << (dst ? "copy" : "create") << endl;
  cout << " vtxCount: " << src->vtxCount << endl;
  cout << " indexCount: " << src->indexCount << endl;
#endif
  if(!dst){
    dst = new trimeshvi;
    if(!dst) throw runtime_error("can't create trimeshvi");
    dst->vtx = new dReal[3 * src->vtxCount];
    if(!dst->vtx) throw runtime_error("can't create trimeshvi.vtx");
    dst->indices = new dTriIndex[src->indexCount];
    if(!dst->indices) throw runtime_error("can't create trimeshvi.indices");
  }
  dst->vtxCount = src->vtxCount;
#if 0
  for(int i = 0; i < 3 * dst->vtxCount; ++i) dst->vtx[i] = sc * src->vtx[i];
#else
  memcpy(dst->vtx, src->vtx, sizeof(dReal) * 3 * dst->vtxCount);
  if(isRescale(sc)) ScaleTriMeshVI(dst, sc);
#endif
  dst->indexCount = src->indexCount;
  memcpy(dst->indices, src->indices, sizeof(dTriIndex) * dst->indexCount);
  return dst;
}

convexfvp *ScaleConvexFVP(convexfvp *fvp, dReal sc)
{
  for(int i = 0; i < 3 * fvp->vtxCount; ++i) fvp->vtx[i] *= sc;
  return fvp;
}

convexfvp *CopyConvexFVP(convexfvp *dst, convexfvp *src, dReal sc)
{
  size_t count = 0;
  unsigned int *s = src->polygons;
  for(int i = 0; i < src->faceCount; ++i){
    unsigned int n = *s++;
    s += n;
    count += (1 + n);
  }
#if 0
  cout << "CopyConvexFVP: " << (dst ? "copy" : "create") << endl;
  cout << " faceCount: " << src->faceCount << endl;
  cout << " vtxCount: " << src->vtxCount << endl;
  cout << " indexCount of Polygons: " << count << endl;
#endif
  if(!dst){
    dst = new convexfvp;
    if(!dst) throw runtime_error("can't create convexfvp");
    dst->faces = new dReal[4 * src->faceCount];
    if(!dst->faces) throw runtime_error("can't create convexfvp.faces");
    dst->vtx = new dReal[3 * src->vtxCount];
    if(!dst->vtx) throw runtime_error("can't create convexfvp.vtx");
    dst->polygons = new unsigned int[count];
    if(!dst->polygons) throw runtime_error("can't create convexfvp.polygons");
  }
  dst->faceCount = src->faceCount;
  memcpy(dst->faces, src->faces, sizeof(dReal) * 4 * dst->faceCount);
  dst->vtxCount = src->vtxCount;
#if 0
  for(int i = 0; i < 3 * dst->vtxCount; ++i) dst->vtx[i] = sc * src->vtx[i];
#else
  memcpy(dst->vtx, src->vtx, sizeof(dReal) * 3 * dst->vtxCount);
  if(isRescale(sc)) ScaleConvexFVP(dst, sc);
#endif
  memcpy(dst->polygons, src->polygons, sizeof(unsigned int) * count);
  return dst;
}
