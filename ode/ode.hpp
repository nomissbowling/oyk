// dummy for preprocess header to Rust bindgen

#include <stdio.h> // FILE __acrt_iob_func
#include <stdarg.h> // va_list
#include <math.h> // sqrt
#include <time.h> // time_t

// #include <sys/types.h> // not define types below

typedef long long dint64;
typedef unsigned long long duint64;
typedef int dint32;
typedef unsigned int duint32;
typedef short dint16;
typedef unsigned short duint16;
typedef signed char dint8;
typedef unsigned char duint8;

typedef dint64 dintptr;
typedef duint64 duintptr;
typedef dint64 ddiffint;
typedef duint64 dsizeint;

typedef double dReal;
typedef duint32 dTriIndex;

extern "C" {

// instead of to Define the dInfinity macro
static dReal dInfinity = 1.0 / 0.0; // INFINITY HUGE_VAL HUGE_VALF

typedef void dMessageFunction (int errnum, const char *msg, va_list ap);

void dSetErrorHandler (dMessageFunction *fn);
void dSetDebugHandler (dMessageFunction *fn);
void dSetMessageHandler (dMessageFunction *fn);

dMessageFunction *dGetErrorHandler(void);
dMessageFunction *dGetDebugHandler(void);
dMessageFunction *dGetMessageHandler(void);

void dError (int num, const char *msg, ...);
void dDebug (int num, const char *msg, ...);
void dMessage (int num, const char *msg, ...);

typedef enum {
    dSA__MIN,

    dSA_X = dSA__MIN,
    dSA_Y,
    dSA_Z,

    dSA__MAX,
} dSpaceAxis;

typedef enum {
    dMD__MIN,

    dMD_LINEAR = dMD__MIN,
    dMD_ANGULAR,

    dMD__MAX,
} dMotionDynamics;

typedef enum {
    dDA__MIN,

    dDA__L_MIN = dDA__MIN + dMD_LINEAR * dSA__MAX,

    dDA_LX = dDA__L_MIN + dSA_X,
    dDA_LY = dDA__L_MIN + dSA_Y,
    dDA_LZ = dDA__L_MIN + dSA_Z,

    dDA__L_MAX = dDA__L_MIN + dSA__MAX,

    dDA__A_MIN = dDA__MIN + dMD_ANGULAR * dSA__MAX,

    dDA_AX = dDA__A_MIN + dSA_X,
    dDA_AY = dDA__A_MIN + dSA_Y,
    dDA_AZ = dDA__A_MIN + dSA_Z,

    dDA__A_MAX = dDA__A_MIN + dSA__MAX,

    dDA__MAX = dDA__MIN + dMD__MAX * dSA__MAX,
} dDynamicsAxis;

typedef enum {
    dV3E__MIN,

    dV3E__AXES_MIN = dV3E__MIN,

    dV3E_X = dV3E__AXES_MIN + dSA_X,
    dV3E_Y = dV3E__AXES_MIN + dSA_Y,
    dV3E_Z = dV3E__AXES_MIN + dSA_Z,

    dV3E__AXES_MAX = dV3E__AXES_MIN + dSA__MAX,

    dV3E_PAD = dV3E__AXES_MAX,

    dV3E__MAX,

    dV3E__AXES_COUNT = dV3E__AXES_MAX - dV3E__AXES_MIN,
} dVec3Element;

typedef enum {
    dV4E__MIN,

    dV4E_X = dV4E__MIN + dSA_X,
    dV4E_Y = dV4E__MIN + dSA_Y,
    dV4E_Z = dV4E__MIN + dSA_Z,
    dV4E_O = dV4E__MIN + dSA__MAX,

    dV4E__MAX,
} dVec4Element;

typedef enum {
    dM3E__MIN,

    dM3E__X_MIN = dM3E__MIN + dSA_X * dV3E__MAX,

    dM3E__X_AXES_MIN = dM3E__X_MIN + dV3E__AXES_MIN,

    dM3E_XX = dM3E__X_MIN + dV3E_X,
    dM3E_XY = dM3E__X_MIN + dV3E_Y,
    dM3E_XZ = dM3E__X_MIN + dV3E_Z,

    dM3E__X_AXES_MAX = dM3E__X_MIN + dV3E__AXES_MAX,

    dM3E_XPAD = dM3E__X_MIN + dV3E_PAD,

    dM3E__X_MAX = dM3E__X_MIN + dV3E__MAX,

    dM3E__Y_MIN = dM3E__MIN + dSA_Y * dV3E__MAX,

    dM3E__Y_AXES_MIN = dM3E__Y_MIN + dV3E__AXES_MIN,

    dM3E_YX = dM3E__Y_MIN + dV3E_X,
    dM3E_YY = dM3E__Y_MIN + dV3E_Y,
    dM3E_YZ = dM3E__Y_MIN + dV3E_Z,

    dM3E__Y_AXES_MAX = dM3E__Y_MIN + dV3E__AXES_MAX,

    dM3E_YPAD = dM3E__Y_MIN + dV3E_PAD,

    dM3E__Y_MAX = dM3E__Y_MIN + dV3E__MAX,

    dM3E__Z_MIN = dM3E__MIN + dSA_Z * dV3E__MAX,

    dM3E__Z_AXES_MIN = dM3E__Z_MIN + dV3E__AXES_MIN,

    dM3E_ZX = dM3E__Z_MIN + dV3E_X,
    dM3E_ZY = dM3E__Z_MIN + dV3E_Y,
    dM3E_ZZ = dM3E__Z_MIN + dV3E_Z,

    dM3E__Z_AXES_MAX = dM3E__Z_MIN + dV3E__AXES_MAX,

    dM3E_ZPAD = dM3E__Z_MIN + dV3E_PAD,

    dM3E__Z_MAX = dM3E__Z_MIN + dV3E__MAX,

    dM3E__MAX = dM3E__MIN + dSA__MAX * dV3E__MAX,
} dMat3Element;

typedef enum {
    dM4E__MIN,

    dM4E__X_MIN = dM4E__MIN + dV4E_X * dV4E__MAX,

    dM4E_XX = dM4E__X_MIN + dV4E_X,
    dM4E_XY = dM4E__X_MIN + dV4E_Y,
    dM4E_XZ = dM4E__X_MIN + dV4E_Z,
    dM4E_XO = dM4E__X_MIN + dV4E_O,

    dM4E__X_MAX = dM4E__X_MIN + dV4E__MAX,

    dM4E__Y_MIN = dM4E__MIN + dV4E_Y * dV4E__MAX,

    dM4E_YX = dM4E__Y_MIN + dV4E_X,
    dM4E_YY = dM4E__Y_MIN + dV4E_Y,
    dM4E_YZ = dM4E__Y_MIN + dV4E_Z,
    dM4E_YO = dM4E__Y_MIN + dV4E_O,

    dM4E__Y_MAX = dM4E__Y_MIN + dV4E__MAX,

    dM4E__Z_MIN = dM4E__MIN + dV4E_Z * dV4E__MAX,

    dM4E_ZX = dM4E__Z_MIN + dV4E_X,
    dM4E_ZY = dM4E__Z_MIN + dV4E_Y,
    dM4E_ZZ = dM4E__Z_MIN + dV4E_Z,
    dM4E_ZO = dM4E__Z_MIN + dV4E_O,

    dM4E__Z_MAX = dM4E__Z_MIN + dV4E__MAX,

    dM4E__O_MIN = dM4E__MIN + dV4E_O * dV4E__MAX,

    dM4E_OX = dM4E__O_MIN + dV4E_X,
    dM4E_OY = dM4E__O_MIN + dV4E_Y,
    dM4E_OZ = dM4E__O_MIN + dV4E_Z,
    dM4E_OO = dM4E__O_MIN + dV4E_O,

    dM4E__O_MAX = dM4E__O_MIN + dV4E__MAX,

    dM4E__MAX = dM4E__MIN + dV4E__MAX * dV4E__MAX,
} dMat4Element;

typedef enum {
    dQUE__MIN,

    dQUE_R = dQUE__MIN,

    dQUE__AXIS_MIN,

    dQUE_I = dQUE__AXIS_MIN + dSA_X,
    dQUE_J = dQUE__AXIS_MIN + dSA_Y,
    dQUE_K = dQUE__AXIS_MIN + dSA_Z,

    dQUE__AXIS_MAX = dQUE__AXIS_MIN + dSA__MAX,

    dQUE__MAX = dQUE__AXIS_MAX,
} dQuatElement;

typedef dReal dVector3[dV3E__MAX];
typedef dReal dVector4[dV4E__MAX];
typedef dReal dMatrix3[dM3E__MAX];
typedef dReal dMatrix4[dM4E__MAX];
typedef dReal dMatrix6[(dMD__MAX * dV3E__MAX) * (dMD__MAX * dSA__MAX)];
typedef dReal dQuaternion[dQUE__MAX];
static __inline dReal dMin(dReal x, dReal y) { return x <= y ? x : y; }
static __inline dReal dMax(dReal x, dReal y) { return x <= y ? y : x; }

struct dxWorld;
struct dxSpace;
struct dxBody;
struct dxGeom;
struct dxJoint;
struct dxJointGroup;

typedef struct dxWorld *dWorldID;
typedef struct dxSpace *dSpaceID;
typedef struct dxBody *dBodyID;
typedef struct dxGeom *dGeomID;
typedef struct dxJoint *dJointID;
typedef struct dxJointGroup *dJointGroupID;

enum {
  d_ERR_UNKNOWN = 0,
  d_ERR_IASSERT,
  d_ERR_UASSERT,
  d_ERR_LCP
};

typedef enum {
  dJointTypeNone = 0,
  dJointTypeBall,
  dJointTypeHinge,
  dJointTypeSlider,
  dJointTypeContact,
  dJointTypeUniversal,
  dJointTypeHinge2,
  dJointTypeFixed,
  dJointTypeNull,
  dJointTypeAMotor,
  dJointTypeLMotor,
  dJointTypePlane2D,
  dJointTypePR,
  dJointTypePU,
  dJointTypePiston,
  dJointTypeDBall,
  dJointTypeDHinge,
  dJointTypeTransmission,
} dJointType;

enum {
  dParamLoStop = 0, dParamHiStop, dParamVel, dParamLoVel, dParamHiVel, dParamFMax, dParamFudgeFactor, dParamBounce, dParamCFM, dParamStopERP, dParamStopCFM, dParamSuspensionERP, dParamSuspensionCFM, dParamERP,
  dParamsInGroup,
  dParamGroup1 = 0x000, dParamLoStop1 = 0x000, dParamHiStop1, dParamVel1, dParamLoVel1, dParamHiVel1, dParamFMax1, dParamFudgeFactor1, dParamBounce1, dParamCFM1, dParamStopERP1, dParamStopCFM1, dParamSuspensionERP1, dParamSuspensionCFM1, dParamERP1,
  dParamGroup2 = 0x100, dParamLoStop2 = 0x100, dParamHiStop2, dParamVel2, dParamLoVel2, dParamHiVel2, dParamFMax2, dParamFudgeFactor2, dParamBounce2, dParamCFM2, dParamStopERP2, dParamStopCFM2, dParamSuspensionERP2, dParamSuspensionCFM2, dParamERP2,
  dParamGroup3 = 0x200, dParamLoStop3 = 0x200, dParamHiStop3, dParamVel3, dParamLoVel3, dParamHiVel3, dParamFMax3, dParamFudgeFactor3, dParamBounce3, dParamCFM3, dParamStopERP3, dParamStopCFM3, dParamSuspensionERP3, dParamSuspensionCFM3, dParamERP3,

  dParamGroup=0x100
};

enum {
  dAMotorUser = 0,
  dAMotorEuler = 1
};

enum {
  dTransmissionParallelAxes = 0,
  dTransmissionIntersectingAxes = 1,
  dTransmissionChainDrive = 2
};

typedef struct dJointFeedback {
  dVector3 f1;
  dVector3 t1;
  dVector3 f2;
  dVector3 t2;
} dJointFeedback;

void dGeomMoved (dGeomID);
dGeomID dGeomGetBodyNext (dGeomID);
const char* dGetConfiguration (void);
int dCheckConfiguration( const char* token );

enum dInitODEFlags {
    dInitFlagManualThreadCleanup = 0x00000001
};

void dInitODE(void);
int dInitODE2(unsigned int uiInitFlags );

enum dAllocateODEDataFlags {
    dAllocateFlagBasicData = 0,

    dAllocateFlagCollisionData = 0x00000001,

    dAllocateMaskAll = ~0
};

int dAllocateODEDataForThread(unsigned int uiAllocateFlags);
void dCleanupODEAllDataForThread();
void dCloseODE(void);

enum {
  dContactMu2 = 0x001,
  dContactAxisDep = 0x001,
  dContactFDir1 = 0x002,
  dContactBounce = 0x004,
  dContactSoftERP = 0x008,
  dContactSoftCFM = 0x010,
  dContactMotion1 = 0x020,
  dContactMotion2 = 0x040,
  dContactMotionN = 0x080,
  dContactSlip1 = 0x100,
  dContactSlip2 = 0x200,
  dContactRolling = 0x400,

  dContactApprox0 = 0x0000,
  dContactApprox1_1 = 0x1000,
  dContactApprox1_2 = 0x2000,
  dContactApprox1_N = 0x4000,
  dContactApprox1 = 0x7000
};

typedef struct dSurfaceParameters {
  int mode;
  dReal mu;

  dReal mu2;
  dReal rho;
  dReal rho2;
  dReal rhoN;
  dReal bounce;
  dReal bounce_vel;
  dReal soft_erp;
  dReal soft_cfm;
  dReal motion1,motion2,motionN;
  dReal slip1,slip2;
} dSurfaceParameters;

typedef struct dContactGeom {
    dVector3 pos;
    dVector3 normal;
    dReal depth;
    dGeomID g1,g2;
    int side1,side2;
} dContactGeom;

typedef struct dContact {
  dSurfaceParameters surface;
  dContactGeom geom;
  dVector3 fdir1;
} dContact;

typedef void * dAllocFunction (dsizeint size);
typedef void * dReallocFunction (void *ptr, dsizeint oldsize, dsizeint newsize);
typedef void dFreeFunction (void *ptr, dsizeint size);

void dSetAllocHandler (dAllocFunction *fn);
void dSetReallocHandler (dReallocFunction *fn);
void dSetFreeHandler (dFreeFunction *fn);

dAllocFunction *dGetAllocHandler (void);
dReallocFunction *dGetReallocHandler (void);
dFreeFunction *dGetFreeHandler (void);

void * dAlloc (dsizeint size);
void * dRealloc (void *ptr, dsizeint oldsize, dsizeint newsize);
void dFree (void *ptr, dsizeint size);

static __inline void dZeroVector3(dVector3 res)
{
    res[dV3E_X] = (0.0);
    res[dV3E_Y] = (0.0);
    res[dV3E_Z] = (0.0);
}

static __inline void dAssignVector3(dVector3 res, dReal x, dReal y, dReal z)
{
    res[dV3E_X] = x;
    res[dV3E_Y] = y;
    res[dV3E_Z] = z;
}

static __inline void dZeroMatrix3(dMatrix3 res)
{
    res[dM3E_XX] = (0.0); res[dM3E_XY] = (0.0); res[dM3E_XZ] = (0.0);
    res[dM3E_YX] = (0.0); res[dM3E_YY] = (0.0); res[dM3E_YZ] = (0.0);
    res[dM3E_ZX] = (0.0); res[dM3E_ZY] = (0.0); res[dM3E_ZZ] = (0.0);
}

static __inline void dZeroMatrix4(dMatrix4 res)
{
    res[dM4E_XX] = (0.0); res[dM4E_XY] = (0.0); res[dM4E_XZ] = (0.0); res[dM4E_XO] = (0.0);
    res[dM4E_YX] = (0.0); res[dM4E_YY] = (0.0); res[dM4E_YZ] = (0.0); res[dM4E_YO] = (0.0);
    res[dM4E_ZX] = (0.0); res[dM4E_ZY] = (0.0); res[dM4E_ZZ] = (0.0); res[dM4E_ZO] = (0.0);
    res[dM4E_OX] = (0.0); res[dM4E_OY] = (0.0); res[dM4E_OZ] = (0.0); res[dM4E_OO] = (0.0);
}

static __inline void dAddVectors3(dReal *res, const dReal *a, const dReal *b)
{
  const dReal res_0 = a[0] + b[0];
  const dReal res_1 = a[1] + b[1];
  const dReal res_2 = a[2] + b[2];

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dSubtractVectors3(dReal *res, const dReal *a, const dReal *b)
{
  const dReal res_0 = a[0] - b[0];
  const dReal res_1 = a[1] - b[1];
  const dReal res_2 = a[2] - b[2];

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dAddVectorScaledVector3(dReal *res, const dReal *a, const dReal *b, dReal b_scale)
{
    const dReal res_0 = a[0] + b_scale * b[0];
    const dReal res_1 = a[1] + b_scale * b[1];
    const dReal res_2 = a[2] + b_scale * b[2];

    res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dAddScaledVectors3(dReal *res, const dReal *a, const dReal *b, dReal a_scale, dReal b_scale)
{
  const dReal res_0 = a_scale * a[0] + b_scale * b[0];
  const dReal res_1 = a_scale * a[1] + b_scale * b[1];
  const dReal res_2 = a_scale * a[2] + b_scale * b[2];

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dAddThreeScaledVectors3(dReal *res, const dReal *a, const dReal *b, const dReal *c, dReal a_scale, dReal b_scale, dReal c_scale)
{
    const dReal res_0 = a_scale * a[0] + b_scale * b[0] + c_scale * c[0];
    const dReal res_1 = a_scale * a[1] + b_scale * b[1] + c_scale * c[1];
    const dReal res_2 = a_scale * a[2] + b_scale * b[2] + c_scale * c[2];

    res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dScaleVector3(dReal *res, dReal nScale)
{
  res[0] *= nScale ;
  res[1] *= nScale ;
  res[2] *= nScale ;
}

static __inline void dNegateVector3(dReal *res)
{
  res[0] = -res[0];
  res[1] = -res[1];
  res[2] = -res[2];
}

static __inline void dCopyVector3(dReal *res, const dReal *a)
{
  const dReal res_0 = a[0];
  const dReal res_1 = a[1];
  const dReal res_2 = a[2];

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dCopyScaledVector3(dReal *res, const dReal *a, dReal nScale)
{
  const dReal res_0 = a[0] * nScale;
  const dReal res_1 = a[1] * nScale;
  const dReal res_2 = a[2] * nScale;

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dCopyNegatedVector3(dReal *res, const dReal *a)
{
  const dReal res_0 = -a[0];
  const dReal res_1 = -a[1];
  const dReal res_2 = -a[2];

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dCopyVector4(dReal *res, const dReal *a)
{
  const dReal res_0 = a[0];
  const dReal res_1 = a[1];
  const dReal res_2 = a[2];
  const dReal res_3 = a[3];

  res[0] = res_0; res[1] = res_1; res[2] = res_2; res[3] = res_3;
}

static __inline void dCopyMatrix4x4(dReal *res, const dReal *a)
{
  dCopyVector4(res + 0, a + 0);
  dCopyVector4(res + 4, a + 4);
  dCopyVector4(res + 8, a + 8);
}

static __inline void dCopyMatrix4x3(dReal *res, const dReal *a)
{
  dCopyVector3(res + 0, a + 0);
  dCopyVector3(res + 4, a + 4);
  dCopyVector3(res + 8, a + 8);
}

static __inline void dGetMatrixColumn3(dReal *res, const dReal *a, unsigned n)
{
  const dReal res_0 = a[n + 0];
  const dReal res_1 = a[n + 4];
  const dReal res_2 = a[n + 8];

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline dReal dCalcVectorLength3(const dReal *a)
{
  return sqrt(a[0] * a[0] + a[1] * a[1] + a[2] * a[2]);
}

static __inline dReal dCalcVectorLengthSquare3(const dReal *a)
{
  return (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]);
}

static __inline dReal dCalcPointDepth3(const dReal *test_p, const dReal *plane_p, const dReal *plane_n)
{
  return (plane_p[0] - test_p[0]) * plane_n[0] + (plane_p[1] - test_p[1]) * plane_n[1] + (plane_p[2] - test_p[2]) * plane_n[2];
}

static __inline dReal _dCalcVectorDot3(const dReal *a, const dReal *b, unsigned step_a, unsigned step_b)
{
  return a[0] * b[0] + a[step_a] * b[step_b] + a[2 * step_a] * b[2 * step_b];
}

static __inline dReal dCalcVectorDot3 (const dReal *a, const dReal *b) { return _dCalcVectorDot3(a,b,1,1); }
static __inline dReal dCalcVectorDot3_13 (const dReal *a, const dReal *b) { return _dCalcVectorDot3(a,b,1,3); }
static __inline dReal dCalcVectorDot3_31 (const dReal *a, const dReal *b) { return _dCalcVectorDot3(a,b,3,1); }
static __inline dReal dCalcVectorDot3_33 (const dReal *a, const dReal *b) { return _dCalcVectorDot3(a,b,3,3); }
static __inline dReal dCalcVectorDot3_14 (const dReal *a, const dReal *b) { return _dCalcVectorDot3(a,b,1,4); }
static __inline dReal dCalcVectorDot3_41 (const dReal *a, const dReal *b) { return _dCalcVectorDot3(a,b,4,1); }
static __inline dReal dCalcVectorDot3_44 (const dReal *a, const dReal *b) { return _dCalcVectorDot3(a,b,4,4); }

static __inline void _dCalcVectorCross3(dReal *res, const dReal *a, const dReal *b, unsigned step_res, unsigned step_a, unsigned step_b)
{
  const dReal res_0 = a[ step_a]*b[2*step_b] - a[2*step_a]*b[ step_b];
  const dReal res_1 = a[2*step_a]*b[ 0] - a[ 0]*b[2*step_b];
  const dReal res_2 = a[ 0]*b[ step_b] - a[ step_a]*b[ 0];

  res[ 0] = res_0;
  res[ step_res] = res_1;
  res[2*step_res] = res_2;
}

static __inline void dCalcVectorCross3 (dReal *res, const dReal *a, const dReal *b) { _dCalcVectorCross3(res, a, b, 1, 1, 1); }
static __inline void dCalcVectorCross3_114(dReal *res, const dReal *a, const dReal *b) { _dCalcVectorCross3(res, a, b, 1, 1, 4); }
static __inline void dCalcVectorCross3_141(dReal *res, const dReal *a, const dReal *b) { _dCalcVectorCross3(res, a, b, 1, 4, 1); }
static __inline void dCalcVectorCross3_144(dReal *res, const dReal *a, const dReal *b) { _dCalcVectorCross3(res, a, b, 1, 4, 4); }
static __inline void dCalcVectorCross3_411(dReal *res, const dReal *a, const dReal *b) { _dCalcVectorCross3(res, a, b, 4, 1, 1); }
static __inline void dCalcVectorCross3_414(dReal *res, const dReal *a, const dReal *b) { _dCalcVectorCross3(res, a, b, 4, 1, 4); }
static __inline void dCalcVectorCross3_441(dReal *res, const dReal *a, const dReal *b) { _dCalcVectorCross3(res, a, b, 4, 4, 1); }
static __inline void dCalcVectorCross3_444(dReal *res, const dReal *a, const dReal *b) { _dCalcVectorCross3(res, a, b, 4, 4, 4); }

static __inline void dAddVectorCross3(dReal *res, const dReal *a, const dReal *b)
{
  dReal tmp[3];
  dCalcVectorCross3(tmp, a, b);
  dAddVectors3(res, res, tmp);
}

static __inline void dSubtractVectorCross3(dReal *res, const dReal *a, const dReal *b)
{
  dReal tmp[3];
  dCalcVectorCross3(tmp, a, b);
  dSubtractVectors3(res, res, tmp);
}

static __inline void dSetCrossMatrixPlus(dReal *res, const dReal *a, unsigned skip)
{
  const dReal a_0 = a[0], a_1 = a[1], a_2 = a[2];
  res[1] = -a_2;
  res[2] = +a_1;
  res[skip+0] = +a_2;
  res[skip+2] = -a_0;
  res[2*skip+0] = -a_1;
  res[2*skip+1] = +a_0;
}

static __inline void dSetCrossMatrixMinus(dReal *res, const dReal *a, unsigned skip)
{
  const dReal a_0 = a[0], a_1 = a[1], a_2 = a[2];
  res[1] = +a_2;
  res[2] = -a_1;
  res[skip+0] = -a_2;
  res[skip+2] = +a_0;
  res[2*skip+0] = +a_1;
  res[2*skip+1] = -a_0;
}

static __inline dReal dCalcPointsDistance3(const dReal *a, const dReal *b)
{
  dReal res;
  dReal tmp[3];
  dSubtractVectors3(tmp, a, b);
  res = dCalcVectorLength3(tmp);
  return res;
}

static __inline void dMultiplyHelper0_331(dReal *res, const dReal *a, const dReal *b)
{
  const dReal res_0 = dCalcVectorDot3(a, b);
  const dReal res_1 = dCalcVectorDot3(a + 4, b);
  const dReal res_2 = dCalcVectorDot3(a + 8, b);

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

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

static __inline void dMultiplyHelper1_133(dReal *res, const dReal *a, const dReal *b)
{
  const dReal res_0 = dCalcVectorDot3_44(a, b);
  const dReal res_1 = dCalcVectorDot3_44(a + 1, b);
  const dReal res_2 = dCalcVectorDot3_44(a + 2, b);

  res[0] = res_0; res[1] = res_1; res[2] = res_2;
}

static __inline void dMultiply0_331(dReal *res, const dReal *a, const dReal *b)
{
  dMultiplyHelper0_331(res, a, b);
}

static __inline void dMultiply1_331(dReal *res, const dReal *a, const dReal *b)
{
  dMultiplyHelper1_331(res, a, b);
}

static __inline void dMultiply0_133(dReal *res, const dReal *a, const dReal *b)
{
  dMultiplyHelper0_133(res, a, b);
}

static __inline void dMultiply0_333(dReal *res, const dReal *a, const dReal *b)
{
  dMultiplyHelper0_133(res + 0, a + 0, b);
  dMultiplyHelper0_133(res + 4, a + 4, b);
  dMultiplyHelper0_133(res + 8, a + 8, b);
}

static __inline void dMultiply1_333(dReal *res, const dReal *a, const dReal *b)
{
  dMultiplyHelper1_133(res + 0, b, a + 0);
  dMultiplyHelper1_133(res + 4, b, a + 1);
  dMultiplyHelper1_133(res + 8, b, a + 2);
}

static __inline void dMultiply2_333(dReal *res, const dReal *a, const dReal *b)
{
  dMultiplyHelper0_331(res + 0, b, a + 0);
  dMultiplyHelper0_331(res + 4, b, a + 4);
  dMultiplyHelper0_331(res + 8, b, a + 8);
}

static __inline void dMultiplyAdd0_331(dReal *res, const dReal *a, const dReal *b)
{
  dReal tmp[3];
  dMultiplyHelper0_331(tmp, a, b);
  dAddVectors3(res, res, tmp);
}

static __inline void dMultiplyAdd1_331(dReal *res, const dReal *a, const dReal *b)
{
  dReal tmp[3];
  dMultiplyHelper1_331(tmp, a, b);
  dAddVectors3(res, res, tmp);
}

static __inline void dMultiplyAdd0_133(dReal *res, const dReal *a, const dReal *b)
{
  dReal tmp[3];
  dMultiplyHelper0_133(tmp, a, b);
  dAddVectors3(res, res, tmp);
}

static __inline void dMultiplyAdd0_333(dReal *res, const dReal *a, const dReal *b)
{
  dReal tmp[3];
  dMultiplyHelper0_133(tmp, a + 0, b);
  dAddVectors3(res+ 0, res + 0, tmp);
  dMultiplyHelper0_133(tmp, a + 4, b);
  dAddVectors3(res + 4, res + 4, tmp);
  dMultiplyHelper0_133(tmp, a + 8, b);
  dAddVectors3(res + 8, res + 8, tmp);
}

static __inline void dMultiplyAdd1_333(dReal *res, const dReal *a, const dReal *b)
{
  dReal tmp[3];
  dMultiplyHelper1_133(tmp, b, a + 0);
  dAddVectors3(res + 0, res + 0, tmp);
  dMultiplyHelper1_133(tmp, b, a + 1);
  dAddVectors3(res + 4, res + 4, tmp);
  dMultiplyHelper1_133(tmp, b, a + 2);
  dAddVectors3(res + 8, res + 8, tmp);
}

static __inline void dMultiplyAdd2_333(dReal *res, const dReal *a, const dReal *b)
{
  dReal tmp[3];
  dMultiplyHelper0_331(tmp, b, a + 0);
  dAddVectors3(res + 0, res + 0, tmp);
  dMultiplyHelper0_331(tmp, b, a + 4);
  dAddVectors3(res + 4, res + 4, tmp);
  dMultiplyHelper0_331(tmp, b, a + 8);
  dAddVectors3(res + 8, res + 8, tmp);
}

static __inline dReal dCalcMatrix3Det( const dReal* mat )
{
    dReal det;

    det = mat[0] * ( mat[5]*mat[10] - mat[9]*mat[6] )
        - mat[1] * ( mat[4]*mat[10] - mat[8]*mat[6] )
        + mat[2] * ( mat[4]*mat[9] - mat[8]*mat[5] );

    return( det );
}

static __inline dReal dInvertMatrix3(dReal *dst, const dReal *ma)
{
    dReal det;
    dReal detRecip;

    det = dCalcMatrix3Det( ma );
    if ( det == 0 )
    {
        return 0;
    }

    detRecip = (1.0/(det));

    dst[0] = ( ma[5]*ma[10] - ma[6]*ma[9] ) * detRecip;
    dst[1] = ( ma[9]*ma[2] - ma[1]*ma[10] ) * detRecip;
    dst[2] = ( ma[1]*ma[6] - ma[5]*ma[2] ) * detRecip;

    dst[4] = ( ma[6]*ma[8] - ma[4]*ma[10] ) * detRecip;
    dst[5] = ( ma[0]*ma[10] - ma[8]*ma[2] ) * detRecip;
    dst[6] = ( ma[4]*ma[2] - ma[0]*ma[6] ) * detRecip;

    dst[8] = ( ma[4]*ma[9] - ma[8]*ma[5] ) * detRecip;
    dst[9] = ( ma[8]*ma[1] - ma[0]*ma[9] ) * detRecip;
    dst[10] = ( ma[0]*ma[5] - ma[1]*ma[4] ) * detRecip;

    return det;
}

int dSafeNormalize3 (dVector3 a);
int dSafeNormalize4 (dVector4 a);
void dNormalize3 (dVector3 a);
void dNormalize4 (dVector4 a);
void dPlaneSpace (const dVector3 n, dVector3 p, dVector3 q);

int dOrthogonalizeR(dMatrix3 m);

void dSetZero (dReal *a, int n);
void dSetValue (dReal *a, int n, dReal value);

dReal dDot (const dReal *a, const dReal *b, int n);
void dMultiply0 (dReal *A, const dReal *B, const dReal *C, int p,int q,int r);
void dMultiply1 (dReal *A, const dReal *B, const dReal *C, int p,int q,int r);
void dMultiply2 (dReal *A, const dReal *B, const dReal *C, int p,int q,int r);

int dFactorCholesky (dReal *A, int n);

void dSolveCholesky (const dReal *L, dReal *b, int n);

int dInvertPDMatrix (const dReal *A, dReal *Ainv, int n);

int dIsPositiveDefinite (const dReal *A, int n);

void dFactorLDLT (dReal *A, dReal *d, int n, int nskip);

void dSolveL1 (const dReal *L, dReal *b, int n, int nskip);

void dSolveL1T (const dReal *L, dReal *b, int n, int nskip);

void dScaleVector (dReal *a, const dReal *d, int n);

void dVectorScale (dReal *a, const dReal *d, int n);

void dSolveLDLT (const dReal *L, const dReal *d, dReal *b, int n, int nskip);

void dLDLTAddTL (dReal *L, dReal *d, const dReal *a, int n, int nskip);

void dLDLTRemove (dReal **A, const int *p, dReal *L, dReal *d,
    int n1, int n2, int r, int nskip);

void dRemoveRowCol (dReal *A, int n, int nskip, int r);

struct dxThreadingImplementation;
typedef struct dxThreadingImplementation *dThreadingImplementationID;

typedef unsigned dmutexindex_t;
struct dxMutexGroup;
typedef struct dxMutexGroup *dMutexGroupID;
typedef dMutexGroupID dMutexGroupAllocFunction (dThreadingImplementationID impl, dmutexindex_t Mutex_count, const char *const *Mutex_names_ptr );
typedef void dMutexGroupFreeFunction (dThreadingImplementationID impl, dMutexGroupID mutex_group);
typedef void dMutexGroupMutexLockFunction (dThreadingImplementationID impl, dMutexGroupID mutex_group, dmutexindex_t mutex_index);
typedef void dMutexGroupMutexUnlockFunction (dThreadingImplementationID impl, dMutexGroupID mutex_group, dmutexindex_t mutex_index);

struct dxCallReleasee;
typedef struct dxCallReleasee *dCallReleaseeID;

struct dxCallWait;
typedef struct dxCallWait *dCallWaitID;

typedef dsizeint ddependencycount_t;
typedef ddiffint ddependencychange_t;
typedef dsizeint dcallindex_t;
typedef int dThreadedCallFunction(void *call_context, dcallindex_t instance_index,
  dCallReleaseeID this_releasee);

typedef struct dxThreadedWaitTime
{
  time_t wait_sec;
  unsigned long wait_nsec;

} dThreadedWaitTime;

typedef dCallWaitID dThreadedCallWaitAllocFunction(dThreadingImplementationID impl);
typedef void dThreadedCallWaitResetFunction(dThreadingImplementationID impl, dCallWaitID call_wait);
typedef void dThreadedCallWaitFreeFunction(dThreadingImplementationID impl, dCallWaitID call_wait);
typedef void dThreadedCallPostFunction(dThreadingImplementationID impl, int *out_summary_fault ,
  dCallReleaseeID *out_post_releasee , ddependencycount_t dependencies_count, dCallReleaseeID dependent_releasee ,
  dCallWaitID call_wait ,
  dThreadedCallFunction *call_func, void *call_context, dcallindex_t instance_index,
  const char *call_name );
typedef void dThreadedCallDependenciesCountAlterFunction(dThreadingImplementationID impl, dCallReleaseeID target_releasee,
  ddependencychange_t dependencies_count_change);
typedef void dThreadedCallWaitFunction(dThreadingImplementationID impl, int *out_wait_status ,
  dCallWaitID call_wait, const dThreadedWaitTime *timeout_time_ptr ,
  const char *wait_name );
typedef unsigned dThreadingImplThreadCountRetrieveFunction(dThreadingImplementationID impl);
typedef int dThreadingImplResourcesForCallsPreallocateFunction(dThreadingImplementationID impl,
  ddependencycount_t max_simultaneous_calls_estimate);

typedef struct dxThreadingFunctionsInfo
{
  unsigned struct_size;

  dMutexGroupAllocFunction *alloc_mutex_group;
  dMutexGroupFreeFunction *free_mutex_group;
  dMutexGroupMutexLockFunction *lock_group_mutex;
  dMutexGroupMutexUnlockFunction *unlock_group_mutex;

  dThreadedCallWaitAllocFunction *alloc_call_wait;
  dThreadedCallWaitResetFunction *reset_call_wait;
  dThreadedCallWaitFreeFunction *free_call_wait;

  dThreadedCallPostFunction *post_call;
  dThreadedCallDependenciesCountAlterFunction *alter_call_dependencies_count;
  dThreadedCallWaitFunction *wait_call;

  dThreadingImplThreadCountRetrieveFunction *retrieve_thread_count;
  dThreadingImplResourcesForCallsPreallocateFunction *preallocate_resources_for_calls;
} dThreadingFunctionsInfo;

struct dxCooperative;
struct dxResourceRequirements;
struct dxResourceContainer;
typedef struct dxCooperative *dCooperativeID;
typedef struct dxResourceRequirements *dResourceRequirementsID;
typedef struct dxResourceContainer *dResourceContainerID;

dCooperativeID dCooperativeCreate(const dThreadingFunctionsInfo *functionInfo , dThreadingImplementationID threadingImpl );
void dCooperativeDestroy(dCooperativeID cooperative);
dResourceRequirementsID dResourceRequirementsCreate(dCooperativeID cooperative);
void dResourceRequirementsDestroy(dResourceRequirementsID requirements);
dResourceRequirementsID dResourceRequirementsClone( dResourceRequirementsID requirements);
void dResourceRequirementsMergeIn(dResourceRequirementsID summaryRequirements, dResourceRequirementsID extraRequirements);
dResourceContainerID dResourceContainerAcquire( dResourceRequirementsID requirements);
void dResourceContainerDestroy(dResourceContainerID resources);

void dEstimateCooperativelyFactorLDLTResourceRequirements(dResourceRequirementsID requirements,
    unsigned maximalAllowedThreadCount, unsigned maximalRowCount);
void dCooperativelyFactorLDLT(dResourceContainerID resources, unsigned allowedThreadCount,
    dReal *A, dReal *d, unsigned rowCount, unsigned rowSkip);
void dEstimateCooperativelySolveLDLTResourceRequirements(dResourceRequirementsID requirements,
    unsigned maximalAllowedThreadCount, unsigned maximalRowCount);
void dCooperativelySolveLDLT(dResourceContainerID resources, unsigned allowedThreadCount,
    const dReal *L, const dReal *d, dReal *b, unsigned rowCount, unsigned rowSkip);
void dEstimateCooperativelySolveL1StraightResourceRequirements(dResourceRequirementsID requirements,
    unsigned maximalAllowedThreadCount, unsigned maximalRowCount);
void dCooperativelySolveL1Straight(dResourceContainerID resources, unsigned allowedThreadCount,
    const dReal *L, dReal *b, unsigned rowCount, unsigned rowSkip);
void dEstimateCooperativelySolveL1TransposedResourceRequirements(dResourceRequirementsID requirements,
    unsigned maximalAllowedThreadCount, unsigned maximalRowCount);
void dCooperativelySolveL1Transposed(dResourceContainerID resources, unsigned allowedThreadCount,
    const dReal *L, dReal *b, unsigned rowCount, unsigned rowSkip);
void dEstimateCooperativelyScaleVectorResourceRequirements(dResourceRequirementsID requirements,
    unsigned maximalAllowedThreadCount, unsigned maximalElementCount);
void dCooperativelyScaleVector(dResourceContainerID resources, unsigned allowedThreadCount,
    dReal *dataVector, const dReal *scaleVector, unsigned elementCount);

typedef struct dStopwatch {
  double time;
  unsigned long cc[2];
} dStopwatch;

void dStopwatchReset (dStopwatch *);
void dStopwatchStart (dStopwatch *);
void dStopwatchStop (dStopwatch *);
double dStopwatchTime (dStopwatch *);

void dTimerStart (const char *description);
void dTimerNow (const char *description);
void dTimerEnd(void);

void dTimerReport (FILE *fout, int average);

double dTimerTicksPerSecond(void);

double dTimerResolution(void);

void dRSetIdentity (dMatrix3 R);

void dRFromAxisAndAngle (dMatrix3 R, dReal ax, dReal ay, dReal az,
    dReal angle);

void dRFromEulerAngles (dMatrix3 R, dReal phi, dReal theta, dReal psi);

void dRFrom2Axes (dMatrix3 R, dReal ax, dReal ay, dReal az,
    dReal bx, dReal by, dReal bz);

void dRFromZAxis (dMatrix3 R, dReal ax, dReal ay, dReal az);

void dQSetIdentity (dQuaternion q);

void dQFromAxisAndAngle (dQuaternion q, dReal ax, dReal ay, dReal az,
    dReal angle);

void dQMultiply0 (dQuaternion qa, const dQuaternion qb, const dQuaternion qc);

void dQMultiply1 (dQuaternion qa, const dQuaternion qb, const dQuaternion qc);

void dQMultiply2 (dQuaternion qa, const dQuaternion qb, const dQuaternion qc);

void dQMultiply3 (dQuaternion qa, const dQuaternion qb, const dQuaternion qc);

void dRfromQ (dMatrix3 R, const dQuaternion q);
void dQfromR (dQuaternion q, const dMatrix3 R);
void dDQfromW (dReal dq[4], const dVector3 w, const dQuaternion q);

struct dMass;
typedef struct dMass dMass;
int dMassCheck(const dMass *m);

void dMassSetZero (dMass *);

void dMassSetParameters (dMass *, dReal themass,
    dReal cgx, dReal cgy, dReal cgz,
    dReal I11, dReal I22, dReal I33,
    dReal I12, dReal I13, dReal I23);

void dMassSetSphere (dMass *, dReal density, dReal radius);
void dMassSetSphereTotal (dMass *, dReal total_mass, dReal radius);

void dMassSetCapsule (dMass *, dReal density, int direction,
     dReal radius, dReal length);
void dMassSetCapsuleTotal (dMass *, dReal total_mass, int direction,
   dReal radius, dReal length);

void dMassSetCylinder (dMass *, dReal density, int direction,
         dReal radius, dReal length);
void dMassSetCylinderTotal (dMass *, dReal total_mass, int direction,
       dReal radius, dReal length);

void dMassSetBox (dMass *, dReal density,
    dReal lx, dReal ly, dReal lz);
void dMassSetBoxTotal (dMass *, dReal total_mass,
         dReal lx, dReal ly, dReal lz);

void dMassSetTrimesh (dMass *, dReal density, dGeomID g);

void dMassSetTrimeshTotal (dMass *m, dReal total_mass, dGeomID g);

void dMassAdjust (dMass *, dReal newmass);

void dMassTranslate (dMass *, dReal x, dReal y, dReal z);

void dMassRotate (dMass *, const dMatrix3 R);

void dMassAdd (dMass *a, const dMass *b);

void dMassSetCappedCylinder(dMass *a, dReal b, int c, dReal d, dReal e);
void dMassSetCappedCylinderTotal(dMass *a, dReal b, int c, dReal d, dReal e);

struct dMass {
  dReal mass;
  dVector3 c;
  dMatrix3 I;

  dMass()
    { dMassSetZero (this); }
  void setZero()
    { dMassSetZero (this); }
  void setParameters (dReal themass, dReal cgx, dReal cgy, dReal cgz,
        dReal I11, dReal I22, dReal I33,
        dReal I12, dReal I13, dReal I23)
    { dMassSetParameters (this,themass,cgx,cgy,cgz,I11,I22,I33,I12,I13,I23); }

  void setSphere (dReal density, dReal radius)
    { dMassSetSphere (this,density,radius); }
  void setSphereTotal (dReal total, dReal radius)
    { dMassSetSphereTotal (this,total,radius); }

  void setCapsule (dReal density, int direction, dReal radius, dReal length)
    { dMassSetCapsule (this,density,direction,radius,length); }
  void setCapsuleTotal (dReal total, int direction, dReal radius, dReal length)
    { dMassSetCapsule (this,total,direction,radius,length); }

  void setCylinder(dReal density, int direction, dReal radius, dReal length)
    { dMassSetCylinder (this,density,direction,radius,length); }
  void setCylinderTotal(dReal total, int direction, dReal radius, dReal length)
    { dMassSetCylinderTotal (this,total,direction,radius,length); }

  void setBox (dReal density, dReal lx, dReal ly, dReal lz)
    { dMassSetBox (this,density,lx,ly,lz); }
  void setBoxTotal (dReal total, dReal lx, dReal ly, dReal lz)
    { dMassSetBoxTotal (this,total,lx,ly,lz); }

  void setTrimesh(dReal density, dGeomID g)
    { dMassSetTrimesh (this, density, g); }
  void setTrimeshTotal(dReal total, dGeomID g)
    { dMassSetTrimeshTotal (this, total, g); }

  void adjust (dReal newmass)
    { dMassAdjust (this,newmass); }
  void translate (dReal x, dReal y, dReal z)
    { dMassTranslate (this,x,y,z); }
  void rotate (const dMatrix3 R)
    { dMassRotate (this,R); }
  void add (const dMass *b)
    { dMassAdd (this,b); }
};

int dTestRand(void);

unsigned long dRand(void);

unsigned long dRandGetSeed(void);
void dRandSetSeed (unsigned long s);

int dRandInt (int n);

dReal dRandReal(void);

void dPrintMatrix (const dReal *A, int n, int m, const char *fmt, FILE *f);

void dMakeRandomVector (dReal *A, int n, dReal range);

void dMakeRandomMatrix (dReal *A, int n, int m, dReal range);

void dClearUpperTriangle (dReal *A, int n);

dReal dMaxDifference (const dReal *A, const dReal *B, int n, int m);

dReal dMaxDifferenceLowerTriangle (const dReal *A, const dReal *B, int n);

}

static inline void dPrintMatrix (const dReal *A, int n, int m, const char *fmt="%10.4f ") {
  dPrintMatrix(A, n, m, fmt, (__acrt_iob_func(1)));
}

extern "C" {

dWorldID dWorldCreate(void);
void dWorldDestroy (dWorldID world);
void dWorldSetData (dWorldID world, void* data);
void* dWorldGetData (dWorldID world);
void dWorldSetGravity (dWorldID, dReal x, dReal y, dReal z);

void dWorldGetGravity (dWorldID, dVector3 gravity);
void dWorldSetERP (dWorldID, dReal erp);

dReal dWorldGetERP (dWorldID);
void dWorldSetCFM (dWorldID, dReal cfm);

dReal dWorldGetCFM (dWorldID);
void dWorldSetStepIslandsProcessingMaxThreadCount(dWorldID w, unsigned count);
unsigned dWorldGetStepIslandsProcessingMaxThreadCount(dWorldID w);
int dWorldUseSharedWorkingMemory(dWorldID w, dWorldID from_world );
void dWorldCleanupWorkingMemory(dWorldID w);

typedef struct
{
  unsigned struct_size;
  float reserve_factor;
  unsigned reserve_minimum;

} dWorldStepReserveInfo;

int dWorldSetStepMemoryReservationPolicy(dWorldID w, const dWorldStepReserveInfo *policyinfo );

typedef struct
{
  unsigned struct_size;
  void *(*alloc_block)(dsizeint block_size);
  void *(*shrink_block)(void *block_pointer, dsizeint block_current_size, dsizeint block_smaller_size);
  void (*free_block)(void *block_pointer, dsizeint block_current_size);

} dWorldStepMemoryFunctionsInfo;

int dWorldSetStepMemoryManager(dWorldID w, const dWorldStepMemoryFunctionsInfo *memfuncs);
void dWorldSetStepThreadingImplementation(dWorldID w, const dThreadingFunctionsInfo *functions_info, dThreadingImplementationID threading_impl);
int dWorldStep (dWorldID w, dReal stepsize);
int dWorldQuickStep (dWorldID w, dReal stepsize);
void dWorldImpulseToForce(dWorldID, dReal stepsize,
 dReal ix, dReal iy, dReal iz, dVector3 force);
void dWorldSetQuickStepNumIterations (dWorldID, int num);
int dWorldGetQuickStepNumIterations (dWorldID);

void dWorldSetQuickStepW (dWorldID, dReal over_relaxation);

dReal dWorldGetQuickStepW (dWorldID);
void dWorldSetContactMaxCorrectingVel (dWorldID, dReal vel);

dReal dWorldGetContactMaxCorrectingVel (dWorldID);
void dWorldSetContactSurfaceLayer (dWorldID, dReal depth);

dReal dWorldGetContactSurfaceLayer (dWorldID);
dReal dWorldGetAutoDisableLinearThreshold (dWorldID);

void dWorldSetAutoDisableLinearThreshold (dWorldID, dReal linear_average_threshold);

dReal dWorldGetAutoDisableAngularThreshold (dWorldID);

void dWorldSetAutoDisableAngularThreshold (dWorldID, dReal angular_average_threshold);

int dWorldGetAutoDisableAverageSamplesCount (dWorldID);

void dWorldSetAutoDisableAverageSamplesCount (dWorldID, unsigned int average_samples_count );

int dWorldGetAutoDisableSteps (dWorldID);

void dWorldSetAutoDisableSteps (dWorldID, int steps);

dReal dWorldGetAutoDisableTime (dWorldID);

void dWorldSetAutoDisableTime (dWorldID, dReal time);

int dWorldGetAutoDisableFlag (dWorldID);

void dWorldSetAutoDisableFlag (dWorldID, int do_auto_disable);
dReal dWorldGetLinearDampingThreshold (dWorldID w);

void dWorldSetLinearDampingThreshold(dWorldID w, dReal threshold);

dReal dWorldGetAngularDampingThreshold (dWorldID w);

void dWorldSetAngularDampingThreshold(dWorldID w, dReal threshold);

dReal dWorldGetLinearDamping (dWorldID w);

void dWorldSetLinearDamping (dWorldID w, dReal scale);

dReal dWorldGetAngularDamping (dWorldID w);

void dWorldSetAngularDamping(dWorldID w, dReal scale);

void dWorldSetDamping(dWorldID w, dReal linear_scale, dReal angular_scale);

dReal dWorldGetMaxAngularSpeed (dWorldID w);

void dWorldSetMaxAngularSpeed (dWorldID w, dReal max_speed);
dReal dBodyGetAutoDisableLinearThreshold (dBodyID);

void dBodySetAutoDisableLinearThreshold (dBodyID, dReal linear_average_threshold);

dReal dBodyGetAutoDisableAngularThreshold (dBodyID);

void dBodySetAutoDisableAngularThreshold (dBodyID, dReal angular_average_threshold);

int dBodyGetAutoDisableAverageSamplesCount (dBodyID);

void dBodySetAutoDisableAverageSamplesCount (dBodyID, unsigned int average_samples_count);

int dBodyGetAutoDisableSteps (dBodyID);

void dBodySetAutoDisableSteps (dBodyID, int steps);

dReal dBodyGetAutoDisableTime (dBodyID);

void dBodySetAutoDisableTime (dBodyID, dReal time);

int dBodyGetAutoDisableFlag (dBodyID);

void dBodySetAutoDisableFlag (dBodyID, int do_auto_disable);

void dBodySetAutoDisableDefaults (dBodyID);
dWorldID dBodyGetWorld (dBodyID);

dBodyID dBodyCreate (dWorldID);
void dBodyDestroy (dBodyID);

void dBodySetData (dBodyID, void *data);

void *dBodyGetData (dBodyID);
void dBodySetPosition (dBodyID, dReal x, dReal y, dReal z);
void dBodySetRotation (dBodyID, const dMatrix3 R);
void dBodySetQuaternion (dBodyID, const dQuaternion q);

void dBodySetLinearVel (dBodyID, dReal x, dReal y, dReal z);

void dBodySetAngularVel (dBodyID, dReal x, dReal y, dReal z);
const dReal * dBodyGetPosition (dBodyID);
void dBodyCopyPosition (dBodyID body, dVector3 pos);

const dReal * dBodyGetRotation (dBodyID);
void dBodyCopyRotation (dBodyID, dMatrix3 R);

const dReal * dBodyGetQuaternion (dBodyID);
void dBodyCopyQuaternion(dBodyID body, dQuaternion quat);

const dReal * dBodyGetLinearVel (dBodyID);

const dReal * dBodyGetAngularVel (dBodyID);

void dBodySetMass (dBodyID, const dMass *mass);

void dBodyGetMass (dBodyID, dMass *mass);

void dBodyAddForce (dBodyID, dReal fx, dReal fy, dReal fz);

void dBodyAddTorque (dBodyID, dReal fx, dReal fy, dReal fz);

void dBodyAddRelForce (dBodyID, dReal fx, dReal fy, dReal fz);

void dBodyAddRelTorque (dBodyID, dReal fx, dReal fy, dReal fz);

void dBodyAddForceAtPos (dBodyID, dReal fx, dReal fy, dReal fz,
                   dReal px, dReal py, dReal pz);

void dBodyAddForceAtRelPos (dBodyID, dReal fx, dReal fy, dReal fz,
                   dReal px, dReal py, dReal pz);

void dBodyAddRelForceAtPos (dBodyID, dReal fx, dReal fy, dReal fz,
                   dReal px, dReal py, dReal pz);

void dBodyAddRelForceAtRelPos (dBodyID, dReal fx, dReal fy, dReal fz,
                   dReal px, dReal py, dReal pz);

const dReal * dBodyGetForce (dBodyID);
const dReal * dBodyGetTorque (dBodyID);
void dBodySetForce (dBodyID b, dReal x, dReal y, dReal z);
void dBodySetTorque (dBodyID b, dReal x, dReal y, dReal z);

void dBodyGetRelPointPos(dBodyID, dReal px, dReal py, dReal pz,
  dVector3 result);

void dBodyGetRelPointVel(dBodyID, dReal px, dReal py, dReal pz,
  dVector3 result);

void dBodyGetPointVel(dBodyID, dReal px, dReal py, dReal pz,
  dVector3 result);

void dBodyGetPosRelPoint(dBodyID, dReal px, dReal py, dReal pz,
  dVector3 result);

void dBodyVectorToWorld(dBodyID, dReal px, dReal py, dReal pz,
  dVector3 result);

void dBodyVectorFromWorld(dBodyID, dReal px, dReal py, dReal pz,
  dVector3 result);

void dBodySetFiniteRotationMode (dBodyID, int mode);
void dBodySetFiniteRotationAxis (dBodyID, dReal x, dReal y, dReal z);

int dBodyGetFiniteRotationMode (dBodyID);

void dBodyGetFiniteRotationAxis (dBodyID, dVector3 result);

int dBodyGetNumJoints (dBodyID b);

dJointID dBodyGetJoint (dBodyID, int index);
void dBodySetDynamic (dBodyID);
void dBodySetKinematic (dBodyID);

int dBodyIsKinematic (dBodyID);

void dBodyEnable (dBodyID);
void dBodyDisable (dBodyID);

int dBodyIsEnabled (dBodyID);
void dBodySetGravityMode (dBodyID b, int mode);

int dBodyGetGravityMode (dBodyID b);
void dBodySetMovedCallback(dBodyID b, void (*callback)(dBodyID));
dGeomID dBodyGetFirstGeom (dBodyID b);
dGeomID dBodyGetNextGeom (dGeomID g);

void dBodySetDampingDefaults(dBodyID b);

dReal dBodyGetLinearDamping (dBodyID b);
void dBodySetLinearDamping(dBodyID b, dReal scale);

dReal dBodyGetAngularDamping (dBodyID b);
void dBodySetAngularDamping(dBodyID b, dReal scale);
void dBodySetDamping(dBodyID b, dReal linear_scale, dReal angular_scale);

dReal dBodyGetLinearDampingThreshold (dBodyID b);

void dBodySetLinearDampingThreshold(dBodyID b, dReal threshold);

dReal dBodyGetAngularDampingThreshold (dBodyID b);

void dBodySetAngularDampingThreshold(dBodyID b, dReal threshold);

dReal dBodyGetMaxAngularSpeed (dBodyID b);
void dBodySetMaxAngularSpeed(dBodyID b, dReal max_speed);
int dBodyGetGyroscopicMode(dBodyID b);
void dBodySetGyroscopicMode(dBodyID b, int enabled);
dJointID dJointCreateBall (dWorldID, dJointGroupID);

dJointID dJointCreateHinge (dWorldID, dJointGroupID);

dJointID dJointCreateSlider (dWorldID, dJointGroupID);

dJointID dJointCreateContact (dWorldID, dJointGroupID, const dContact *);

dJointID dJointCreateHinge2 (dWorldID, dJointGroupID);

dJointID dJointCreateUniversal (dWorldID, dJointGroupID);

dJointID dJointCreatePR (dWorldID, dJointGroupID);

dJointID dJointCreatePU (dWorldID, dJointGroupID);
dJointID dJointCreatePiston (dWorldID, dJointGroupID);

dJointID dJointCreateFixed (dWorldID, dJointGroupID);

dJointID dJointCreateNull (dWorldID, dJointGroupID);

dJointID dJointCreateAMotor (dWorldID, dJointGroupID);

dJointID dJointCreateLMotor (dWorldID, dJointGroupID);

dJointID dJointCreatePlane2D (dWorldID, dJointGroupID);

dJointID dJointCreateDBall (dWorldID, dJointGroupID);

dJointID dJointCreateDHinge (dWorldID, dJointGroupID);

dJointID dJointCreateTransmission (dWorldID, dJointGroupID);
void dJointDestroy (dJointID);

dJointGroupID dJointGroupCreate (int max_size);

void dJointGroupDestroy (dJointGroupID);
void dJointGroupEmpty (dJointGroupID);

int dJointGetNumBodies(dJointID);
void dJointAttach (dJointID, dBodyID body1, dBodyID body2);

void dJointEnable (dJointID);
void dJointDisable (dJointID);

int dJointIsEnabled (dJointID);

void dJointSetData (dJointID, void *data);

void *dJointGetData (dJointID);
dJointType dJointGetType (dJointID);
dBodyID dJointGetBody (dJointID, int index);
void dJointSetFeedback (dJointID, dJointFeedback *);

dJointFeedback *dJointGetFeedback (dJointID);
void dJointSetBallAnchor (dJointID, dReal x, dReal y, dReal z);

void dJointSetBallAnchor2 (dJointID, dReal x, dReal y, dReal z);

void dJointSetBallParam (dJointID, int parameter, dReal value);

void dJointSetHingeAnchor (dJointID, dReal x, dReal y, dReal z);

void dJointSetHingeAnchorDelta (dJointID, dReal x, dReal y, dReal z, dReal ax, dReal ay, dReal az);

void dJointSetHingeAxis (dJointID, dReal x, dReal y, dReal z);
void dJointSetHingeAxisOffset (dJointID j, dReal x, dReal y, dReal z, dReal angle);

void dJointSetHingeParam (dJointID, int parameter, dReal value);
void dJointAddHingeTorque(dJointID joint, dReal torque);

void dJointSetSliderAxis (dJointID, dReal x, dReal y, dReal z);

void dJointSetSliderAxisDelta (dJointID, dReal x, dReal y, dReal z, dReal ax, dReal ay, dReal az);

void dJointSetSliderParam (dJointID, int parameter, dReal value);
void dJointAddSliderForce(dJointID joint, dReal force);

void dJointSetHinge2Anchor (dJointID, dReal x, dReal y, dReal z);
void dJointSetHinge2Axes (dJointID j, const dReal *axis1 , const dReal *axis2 );
void dJointSetHinge2Axis1 (dJointID j, dReal x, dReal y, dReal z);
void dJointSetHinge2Axis2 (dJointID j, dReal x, dReal y, dReal z);

void dJointSetHinge2Param (dJointID, int parameter, dReal value);

void dJointAddHinge2Torques(dJointID joint, dReal torque1, dReal torque2);

void dJointSetUniversalAnchor (dJointID, dReal x, dReal y, dReal z);

void dJointSetUniversalAxis1 (dJointID, dReal x, dReal y, dReal z);
void dJointSetUniversalAxis1Offset (dJointID, dReal x, dReal y, dReal z,
                                            dReal offset1, dReal offset2);

void dJointSetUniversalAxis2 (dJointID, dReal x, dReal y, dReal z);
void dJointSetUniversalAxis2Offset (dJointID, dReal x, dReal y, dReal z,
                                            dReal offset1, dReal offset2);

void dJointSetUniversalParam (dJointID, int parameter, dReal value);

void dJointAddUniversalTorques(dJointID joint, dReal torque1, dReal torque2);

void dJointSetPRAnchor (dJointID, dReal x, dReal y, dReal z);

void dJointSetPRAxis1 (dJointID, dReal x, dReal y, dReal z);

void dJointSetPRAxis2 (dJointID, dReal x, dReal y, dReal z);

void dJointSetPRParam (dJointID, int parameter, dReal value);
void dJointAddPRTorque (dJointID j, dReal torque);

void dJointSetPUAnchor (dJointID, dReal x, dReal y, dReal z);

void dJointSetPUAnchorDelta (dJointID, dReal x, dReal y, dReal z,
                                     dReal dx, dReal dy, dReal dz);
void dJointSetPUAnchorOffset (dJointID, dReal x, dReal y, dReal z,
                                     dReal dx, dReal dy, dReal dz);

void dJointSetPUAxis1 (dJointID, dReal x, dReal y, dReal z);

void dJointSetPUAxis2 (dJointID, dReal x, dReal y, dReal z);

void dJointSetPUAxis3 (dJointID, dReal x, dReal y, dReal z);

void dJointSetPUAxisP (dJointID id, dReal x, dReal y, dReal z);
void dJointSetPUParam (dJointID, int parameter, dReal value);
void dJointAddPUTorque (dJointID j, dReal torque);
void dJointSetPistonAnchor (dJointID, dReal x, dReal y, dReal z);
void dJointSetPistonAnchorOffset(dJointID j, dReal x, dReal y, dReal z,
                                         dReal dx, dReal dy, dReal dz);

void dJointSetPistonAxis (dJointID, dReal x, dReal y, dReal z);
void dJointSetPistonAxisDelta (dJointID j, dReal x, dReal y, dReal z, dReal ax, dReal ay, dReal az);

void dJointSetPistonParam (dJointID, int parameter, dReal value);
void dJointAddPistonForce (dJointID joint, dReal force);
void dJointSetFixed (dJointID);

void dJointSetFixedParam (dJointID, int parameter, dReal value);

void dJointSetAMotorNumAxes (dJointID, int num);

void dJointSetAMotorAxis (dJointID, int anum, int rel,
     dReal x, dReal y, dReal z);
void dJointSetAMotorAngle (dJointID, int anum, dReal angle);

void dJointSetAMotorParam (dJointID, int parameter, dReal value);

void dJointSetAMotorMode (dJointID, int mode);
void dJointAddAMotorTorques (dJointID, dReal torque1, dReal torque2, dReal torque3);

void dJointSetLMotorNumAxes (dJointID, int num);
void dJointSetLMotorAxis (dJointID, int anum, int rel, dReal x, dReal y, dReal z);

void dJointSetLMotorParam (dJointID, int parameter, dReal value);

void dJointSetPlane2DXParam (dJointID, int parameter, dReal value);

void dJointSetPlane2DYParam (dJointID, int parameter, dReal value);

void dJointSetPlane2DAngleParam (dJointID, int parameter, dReal value);

void dJointGetBallAnchor (dJointID, dVector3 result);
void dJointGetBallAnchor2 (dJointID, dVector3 result);
dReal dJointGetBallParam (dJointID, int parameter);
void dJointGetHingeAnchor (dJointID, dVector3 result);
void dJointGetHingeAnchor2 (dJointID, dVector3 result);
void dJointGetHingeAxis (dJointID, dVector3 result);
dReal dJointGetHingeParam (dJointID, int parameter);
dReal dJointGetHingeAngle (dJointID);
dReal dJointGetHingeAngleRate (dJointID);
dReal dJointGetSliderPosition (dJointID);
dReal dJointGetSliderPositionRate (dJointID);
void dJointGetSliderAxis (dJointID, dVector3 result);
dReal dJointGetSliderParam (dJointID, int parameter);
void dJointGetHinge2Anchor (dJointID, dVector3 result);
void dJointGetHinge2Anchor2 (dJointID, dVector3 result);
void dJointGetHinge2Axis1 (dJointID, dVector3 result);
void dJointGetHinge2Axis2 (dJointID, dVector3 result);
dReal dJointGetHinge2Param (dJointID, int parameter);
dReal dJointGetHinge2Angle1 (dJointID);
dReal dJointGetHinge2Angle2 (dJointID);
dReal dJointGetHinge2Angle1Rate (dJointID);
dReal dJointGetHinge2Angle2Rate (dJointID);
void dJointGetUniversalAnchor (dJointID, dVector3 result);
void dJointGetUniversalAnchor2 (dJointID, dVector3 result);
void dJointGetUniversalAxis1 (dJointID, dVector3 result);
void dJointGetUniversalAxis2 (dJointID, dVector3 result);
dReal dJointGetUniversalParam (dJointID, int parameter);
void dJointGetUniversalAngles (dJointID, dReal *angle1, dReal *angle2);
dReal dJointGetUniversalAngle1 (dJointID);
dReal dJointGetUniversalAngle2 (dJointID);
dReal dJointGetUniversalAngle1Rate (dJointID);
dReal dJointGetUniversalAngle2Rate (dJointID);
void dJointGetPRAnchor (dJointID, dVector3 result);
dReal dJointGetPRPosition (dJointID);
dReal dJointGetPRPositionRate (dJointID);
dReal dJointGetPRAngle (dJointID);
dReal dJointGetPRAngleRate (dJointID);
void dJointGetPRAxis1 (dJointID, dVector3 result);
void dJointGetPRAxis2 (dJointID, dVector3 result);
dReal dJointGetPRParam (dJointID, int parameter);
void dJointGetPUAnchor (dJointID, dVector3 result);
dReal dJointGetPUPosition (dJointID);
dReal dJointGetPUPositionRate (dJointID);
void dJointGetPUAxis1 (dJointID, dVector3 result);
void dJointGetPUAxis2 (dJointID, dVector3 result);
void dJointGetPUAxis3 (dJointID, dVector3 result);
void dJointGetPUAxisP (dJointID id, dVector3 result);
void dJointGetPUAngles (dJointID, dReal *angle1, dReal *angle2);
dReal dJointGetPUAngle1 (dJointID);
dReal dJointGetPUAngle1Rate (dJointID);
dReal dJointGetPUAngle2 (dJointID);
dReal dJointGetPUAngle2Rate (dJointID);
dReal dJointGetPUParam (dJointID, int parameter);
dReal dJointGetPistonPosition (dJointID);
dReal dJointGetPistonPositionRate (dJointID);
dReal dJointGetPistonAngle (dJointID);
dReal dJointGetPistonAngleRate (dJointID);
void dJointGetPistonAnchor (dJointID, dVector3 result);
void dJointGetPistonAnchor2 (dJointID, dVector3 result);
void dJointGetPistonAxis (dJointID, dVector3 result);
dReal dJointGetPistonParam (dJointID, int parameter);
int dJointGetAMotorNumAxes (dJointID);
void dJointGetAMotorAxis (dJointID, int anum, dVector3 result);
int dJointGetAMotorAxisRel (dJointID, int anum);
dReal dJointGetAMotorAngle (dJointID, int anum);
dReal dJointGetAMotorAngleRate (dJointID, int anum);
dReal dJointGetAMotorParam (dJointID, int parameter);
int dJointGetAMotorMode (dJointID);
int dJointGetLMotorNumAxes (dJointID);
void dJointGetLMotorAxis (dJointID, int anum, dVector3 result);
dReal dJointGetLMotorParam (dJointID, int parameter);
dReal dJointGetFixedParam (dJointID, int parameter);
void dJointGetTransmissionContactPoint1(dJointID, dVector3 result);
void dJointGetTransmissionContactPoint2(dJointID, dVector3 result);
void dJointSetTransmissionAxis1(dJointID, dReal x, dReal y, dReal z);
void dJointGetTransmissionAxis1(dJointID, dVector3 result);
void dJointSetTransmissionAxis2(dJointID, dReal x, dReal y, dReal z);
void dJointGetTransmissionAxis2(dJointID, dVector3 result);
void dJointSetTransmissionAnchor1(dJointID, dReal x, dReal y, dReal z);
void dJointGetTransmissionAnchor1(dJointID, dVector3 result);
void dJointSetTransmissionAnchor2(dJointID, dReal x, dReal y, dReal z);
void dJointGetTransmissionAnchor2(dJointID, dVector3 result);
void dJointSetTransmissionParam(dJointID, int parameter, dReal value);
dReal dJointGetTransmissionParam(dJointID, int parameter);
void dJointSetTransmissionMode( dJointID j, int mode );
int dJointGetTransmissionMode( dJointID j );
void dJointSetTransmissionRatio( dJointID j, dReal ratio );
dReal dJointGetTransmissionRatio( dJointID j );
void dJointSetTransmissionAxis( dJointID j, dReal x, dReal y, dReal z );
void dJointGetTransmissionAxis( dJointID j, dVector3 result );
dReal dJointGetTransmissionAngle1( dJointID j );
dReal dJointGetTransmissionAngle2( dJointID j );
dReal dJointGetTransmissionRadius1( dJointID j );
dReal dJointGetTransmissionRadius2( dJointID j );
void dJointSetTransmissionRadius1( dJointID j, dReal radius );
void dJointSetTransmissionRadius2( dJointID j, dReal radius );
dReal dJointGetTransmissionBacklash( dJointID j );
void dJointSetTransmissionBacklash( dJointID j, dReal backlash );
void dJointSetDBallAnchor1(dJointID, dReal x, dReal y, dReal z);
void dJointSetDBallAnchor2(dJointID, dReal x, dReal y, dReal z);
void dJointGetDBallAnchor1(dJointID, dVector3 result);
void dJointGetDBallAnchor2(dJointID, dVector3 result);
dReal dJointGetDBallDistance(dJointID);
void dJointSetDBallDistance(dJointID, dReal dist);
void dJointSetDBallParam(dJointID, int parameter, dReal value);
dReal dJointGetDBallParam(dJointID, int parameter);
void dJointSetDHingeAxis(dJointID, dReal x, dReal y, dReal z);
void dJointGetDHingeAxis(dJointID, dVector3 result);
void dJointSetDHingeAnchor1(dJointID, dReal x, dReal y, dReal z);
void dJointSetDHingeAnchor2(dJointID, dReal x, dReal y, dReal z);
void dJointGetDHingeAnchor1(dJointID, dVector3 result);
void dJointGetDHingeAnchor2(dJointID, dVector3 result);
dReal dJointGetDHingeDistance(dJointID);
void dJointSetDHingeParam(dJointID, int parameter, dReal value);
dReal dJointGetDHingeParam(dJointID, int parameter);

dJointID dConnectingJoint (dBodyID, dBodyID);
int dConnectingJointList (dBodyID, dBodyID, dJointID*);
int dAreConnected (dBodyID, dBodyID);
int dAreConnectedExcluding (dBodyID body1, dBodyID body2, int joint_type);

struct dContactGeom;
typedef void dNearCallback (void *data, dGeomID o1, dGeomID o2);

dSpaceID dSimpleSpaceCreate (dSpaceID space);
dSpaceID dHashSpaceCreate (dSpaceID space);
dSpaceID dQuadTreeSpaceCreate (dSpaceID space, const dVector3 Center, const dVector3 Extents, int Depth);
dSpaceID dSweepAndPruneSpaceCreate( dSpaceID space, int axisorder );

void dSpaceDestroy (dSpaceID);

void dHashSpaceSetLevels (dSpaceID space, int minlevel, int maxlevel);
void dHashSpaceGetLevels (dSpaceID space, int *minlevel, int *maxlevel);

void dSpaceSetCleanup (dSpaceID space, int mode);
int dSpaceGetCleanup (dSpaceID space);
void dSpaceSetSublevel (dSpaceID space, int sublevel);
int dSpaceGetSublevel (dSpaceID space);
void dSpaceSetManualCleanup (dSpaceID space, int mode);
int dSpaceGetManualCleanup (dSpaceID space);

void dSpaceAdd (dSpaceID, dGeomID);
void dSpaceRemove (dSpaceID, dGeomID);
int dSpaceQuery (dSpaceID, dGeomID);
void dSpaceClean (dSpaceID);
int dSpaceGetNumGeoms (dSpaceID);
dGeomID dSpaceGetGeom (dSpaceID, int i);
int dSpaceGetClass(dSpaceID space);

void dGeomDestroy (dGeomID geom);
void dGeomSetData (dGeomID geom, void* data);
void *dGeomGetData (dGeomID geom);
void dGeomSetBody (dGeomID geom, dBodyID body);
dBodyID dGeomGetBody (dGeomID geom);
void dGeomSetPosition (dGeomID geom, dReal x, dReal y, dReal z);
void dGeomSetRotation (dGeomID geom, const dMatrix3 R);
void dGeomSetQuaternion (dGeomID geom, const dQuaternion Q);
const dReal * dGeomGetPosition (dGeomID geom);
void dGeomCopyPosition (dGeomID geom, dVector3 pos);
const dReal * dGeomGetRotation (dGeomID geom);
void dGeomCopyRotation(dGeomID geom, dMatrix3 R);
void dGeomGetQuaternion (dGeomID geom, dQuaternion result);
void dGeomGetAABB (dGeomID geom, dReal aabb[6]);
int dGeomIsSpace (dGeomID geom);
dSpaceID dGeomGetSpace (dGeomID);
int dGeomGetClass (dGeomID geom);
void dGeomSetCategoryBits (dGeomID geom, unsigned long bits);
void dGeomSetCollideBits (dGeomID geom, unsigned long bits);
unsigned long dGeomGetCategoryBits (dGeomID);
unsigned long dGeomGetCollideBits (dGeomID);
void dGeomEnable (dGeomID geom);
void dGeomDisable (dGeomID geom);
int dGeomIsEnabled (dGeomID geom);

enum {
 dGeomCommonControlClass = 0,
 dGeomColliderControlClass = 1
};

enum {
 dGeomCommonAnyControlCode = 0,

 dGeomColliderSetMergeSphereContactsControlCode = 1,
 dGeomColliderGetMergeSphereContactsControlCode = 2
};

enum {
 dGeomColliderMergeContactsValue__Default = 0,
 dGeomColliderMergeContactsValue_None = 1,
 dGeomColliderMergeContactsValue_Normals = 2,
 dGeomColliderMergeContactsValue_Full = 3
};

int dGeomLowLevelControl (dGeomID geom, int controlClass, int controlCode, void *dataValue, int *dataSize);
void dGeomGetRelPointPos(dGeomID geom, dReal px, dReal py, dReal pz,
  dVector3 result);
void dGeomGetPosRelPoint(dGeomID geom, dReal px, dReal py, dReal pz,
  dVector3 result);
void dGeomVectorToWorld(dGeomID geom, dReal px, dReal py, dReal pz,
  dVector3 result);
void dGeomVectorFromWorld(dGeomID geom, dReal px, dReal py, dReal pz,
  dVector3 result);
void dGeomSetOffsetPosition (dGeomID geom, dReal x, dReal y, dReal z);
void dGeomSetOffsetRotation (dGeomID geom, const dMatrix3 R);
void dGeomSetOffsetQuaternion (dGeomID geom, const dQuaternion Q);
void dGeomSetOffsetWorldPosition (dGeomID geom, dReal x, dReal y, dReal z);
void dGeomSetOffsetWorldRotation (dGeomID geom, const dMatrix3 R);
void dGeomSetOffsetWorldQuaternion (dGeomID geom, const dQuaternion);
void dGeomClearOffset(dGeomID geom);
int dGeomIsOffset(dGeomID geom);
const dReal * dGeomGetOffsetPosition (dGeomID geom);
void dGeomCopyOffsetPosition (dGeomID geom, dVector3 pos);
const dReal * dGeomGetOffsetRotation (dGeomID geom);
void dGeomCopyOffsetRotation (dGeomID geom, dMatrix3 R);
void dGeomGetOffsetQuaternion (dGeomID geom, dQuaternion result);
int dCollide (dGeomID o1, dGeomID o2, int flags, dContactGeom *contact,
       int skip);
void dSpaceCollide (dSpaceID space, void *data, dNearCallback *callback);
void dSpaceCollide2 (dGeomID space1, dGeomID space2, void *data, dNearCallback *callback);

enum {
  dMaxUserClasses = 4
};

enum {
  dSphereClass = 0,
  dBoxClass,
  dCapsuleClass,
  dCylinderClass,
  dPlaneClass,
  dRayClass,
  dConvexClass,
  dGeomTransformClass,
  dTriMeshClass,
  dHeightfieldClass,

  dFirstSpaceClass,
  dSimpleSpaceClass = dFirstSpaceClass,
  dHashSpaceClass,
  dSweepAndPruneSpaceClass,
  dQuadTreeSpaceClass,
  dLastSpaceClass = dQuadTreeSpaceClass,

  dFirstUserClass,
  dLastUserClass = dFirstUserClass + dMaxUserClasses - 1,
  dGeomNumClasses
};

dGeomID dCreateSphere (dSpaceID space, dReal radius);
void dGeomSphereSetRadius (dGeomID sphere, dReal radius);
dReal dGeomSphereGetRadius (dGeomID sphere);
dReal dGeomSpherePointDepth (dGeomID sphere, dReal x, dReal y, dReal z);

dGeomID dCreateConvex (dSpaceID space,
          const dReal *_planes,
          unsigned int _planecount,
          const dReal *_points,
          unsigned int _pointcount,
                   const unsigned int *_polygons);

void dGeomSetConvex (dGeomID g,
        const dReal *_planes,
        unsigned int _count,
        const dReal *_points,
        unsigned int _pointcount,
                 const unsigned int *_polygons);

dGeomID dCreateBox (dSpaceID space, dReal lx, dReal ly, dReal lz);
void dGeomBoxSetLengths (dGeomID box, dReal lx, dReal ly, dReal lz);
void dGeomBoxGetLengths (dGeomID box, dVector3 result);
dReal dGeomBoxPointDepth (dGeomID box, dReal x, dReal y, dReal z);

dGeomID dCreatePlane (dSpaceID space, dReal a, dReal b, dReal c, dReal d);
void dGeomPlaneSetParams (dGeomID plane, dReal a, dReal b, dReal c, dReal d);
void dGeomPlaneGetParams (dGeomID plane, dVector4 result);
dReal dGeomPlanePointDepth (dGeomID plane, dReal x, dReal y, dReal z);

dGeomID dCreateCapsule (dSpaceID space, dReal radius, dReal length);
void dGeomCapsuleSetParams (dGeomID ccylinder, dReal radius, dReal length);
void dGeomCapsuleGetParams (dGeomID ccylinder, dReal *radius, dReal *length);
dReal dGeomCapsulePointDepth (dGeomID ccylinder, dReal x, dReal y, dReal z);
dGeomID dCreateCylinder (dSpaceID space, dReal radius, dReal length);
void dGeomCylinderSetParams (dGeomID cylinder, dReal radius, dReal length);
void dGeomCylinderGetParams (dGeomID cylinder, dReal *radius, dReal *length);

dGeomID dCreateRay (dSpaceID space, dReal length);
void dGeomRaySetLength (dGeomID ray, dReal length);
dReal dGeomRayGetLength (dGeomID ray);
void dGeomRaySet (dGeomID ray, dReal px, dReal py, dReal pz,
    dReal dx, dReal dy, dReal dz);
void dGeomRayGet (dGeomID ray, dVector3 start, dVector3 dir);

void dGeomRaySetParams (dGeomID g, int FirstContact, int BackfaceCull);
void dGeomRayGetParams (dGeomID g, int *FirstContact, int *BackfaceCull);
void dGeomRaySetFirstContact (dGeomID g, int firstContact);
int dGeomRayGetFirstContact (dGeomID g);
void dGeomRaySetBackfaceCull (dGeomID g, int backfaceCull);
int dGeomRayGetBackfaceCull (dGeomID g);
void dGeomRaySetClosestHit (dGeomID g, int closestHit);
int dGeomRayGetClosestHit (dGeomID g);

struct dxTriMeshData;
typedef struct dxTriMeshData* dTriMeshDataID;

typedef enum {
    dMTV__MIN,

    dMTV_FIRST = dMTV__MIN,
    dMTV_SECOND,
    dMTV_THIRD,

    dMTV__MAX,
} dMeshTriangleVertex;

dTriMeshDataID dGeomTriMeshDataCreate(void);
void dGeomTriMeshDataDestroy(dTriMeshDataID g);

enum {
    dTRIMESHDATA__MIN,

    dTRIMESHDATA_FACE_NORMALS = dTRIMESHDATA__MIN,
    dTRIMESHDATA_USE_FLAGS,

    dTRIMESHDATA__MAX,

    TRIMESH_FACE_NORMALS = dTRIMESHDATA_FACE_NORMALS,
};

enum {
    dMESHDATAUSE_EDGE1 = 0x01,
    dMESHDATAUSE_EDGE2 = 0x02,
    dMESHDATAUSE_EDGE3 = 0x04,
    dMESHDATAUSE_VERTEX1 = 0x08,
    dMESHDATAUSE_VERTEX2 = 0x10,
    dMESHDATAUSE_VERTEX3 = 0x20,
};

void dGeomTriMeshDataSet(dTriMeshDataID g, int data_id, void *in_data);
void *dGeomTriMeshDataGet(dTriMeshDataID g, int data_id);
void *dGeomTriMeshDataGet2(dTriMeshDataID g, int data_id, dsizeint *pout_size );
void dGeomTriMeshSetLastTransform( dGeomID g, const dMatrix4 last_trans );
const dReal* dGeomTriMeshGetLastTransform( dGeomID g );

void dGeomTriMeshDataBuildSingle(dTriMeshDataID g,
                                 const void* Vertices, int VertexStride, int VertexCount,
                                 const void* Indices, int IndexCount, int TriStride);

void dGeomTriMeshDataBuildSingle1(dTriMeshDataID g,
                                  const void* Vertices, int VertexStride, int VertexCount,
                                  const void* Indices, int IndexCount, int TriStride,
                                  const void* Normals);

void dGeomTriMeshDataBuildDouble(dTriMeshDataID g,
                                 const void* Vertices, int VertexStride, int VertexCount,
                                 const void* Indices, int IndexCount, int TriStride);

void dGeomTriMeshDataBuildDouble1(dTriMeshDataID g,
                                  const void* Vertices, int VertexStride, int VertexCount,
                                  const void* Indices, int IndexCount, int TriStride,
                                  const void* Normals);

void dGeomTriMeshDataBuildSimple(dTriMeshDataID g,
                                 const dReal* Vertices, int VertexCount,
                                 const dTriIndex* Indices, int IndexCount);

void dGeomTriMeshDataBuildSimple1(dTriMeshDataID g,
                                  const dReal* Vertices, int VertexCount,
                                  const dTriIndex* Indices, int IndexCount,
                                  const int* Normals);

enum {
    dTRIDATAPREPROCESS_BUILD__MIN,

    dTRIDATAPREPROCESS_BUILD_CONCAVE_EDGES = dTRIDATAPREPROCESS_BUILD__MIN,
    dTRIDATAPREPROCESS_BUILD_FACE_ANGLES,

    dTRIDATAPREPROCESS_BUILD__MAX,
};

enum {
    dTRIDATAPREPROCESS_FACE_ANGLES_EXTRA__MIN,

    dTRIDATAPREPROCESS_FACE_ANGLES_EXTRA_BYTE_POSITIVE = dTRIDATAPREPROCESS_FACE_ANGLES_EXTRA__MIN,
    dTRIDATAPREPROCESS_FACE_ANGLES_EXTRA_BYTE_ALL,
    dTRIDATAPREPROCESS_FACE_ANGLES_EXTRA_WORD_ALL,

    dTRIDATAPREPROCESS_FACE_ANGLES_EXTRA__MAX,

    dTRIDATAPREPROCESS_FACE_ANGLES_EXTRA__DEFAULT = dTRIDATAPREPROCESS_FACE_ANGLES_EXTRA_BYTE_POSITIVE,
};

int dGeomTriMeshDataPreprocess2(dTriMeshDataID g, unsigned int buildRequestFlags, const dintptr *requestExtraData );

int dGeomTriMeshDataPreprocess(dTriMeshDataID g);

void dGeomTriMeshDataGetBuffer(dTriMeshDataID g, unsigned char** buf, int* bufLen);
void dGeomTriMeshDataSetBuffer(dTriMeshDataID g, unsigned char* buf);

typedef int dTriCallback(dGeomID TriMesh, dGeomID RefObject, int TriangleIndex);
void dGeomTriMeshSetCallback(dGeomID g, dTriCallback* Callback);
dTriCallback* dGeomTriMeshGetCallback(dGeomID g);

typedef void dTriArrayCallback(dGeomID TriMesh, dGeomID RefObject, const int* TriIndices, int TriCount);
void dGeomTriMeshSetArrayCallback(dGeomID g, dTriArrayCallback* ArrayCallback);
dTriArrayCallback* dGeomTriMeshGetArrayCallback(dGeomID g);

typedef int dTriRayCallback(dGeomID TriMesh, dGeomID Ray, int TriangleIndex, dReal u, dReal v);
void dGeomTriMeshSetRayCallback(dGeomID g, dTriRayCallback* Callback);
dTriRayCallback* dGeomTriMeshGetRayCallback(dGeomID g);
typedef int dTriTriMergeCallback(dGeomID TriMesh, int FirstTriangleIndex, int SecondTriangleIndex);
void dGeomTriMeshSetTriMergeCallback(dGeomID g, dTriTriMergeCallback* Callback);
dTriTriMergeCallback* dGeomTriMeshGetTriMergeCallback(dGeomID g);

dGeomID dCreateTriMesh(dSpaceID space, dTriMeshDataID Data, dTriCallback* Callback, dTriArrayCallback* ArrayCallback, dTriRayCallback* RayCallback);

void dGeomTriMeshSetData(dGeomID g, dTriMeshDataID Data);
dTriMeshDataID dGeomTriMeshGetData(dGeomID g);

void dGeomTriMeshEnableTC(dGeomID g, int geomClass, int enable);
int dGeomTriMeshIsTCEnabled(dGeomID g, int geomClass);

void dGeomTriMeshClearTCCache(dGeomID g);

dTriMeshDataID dGeomTriMeshGetTriMeshDataID(dGeomID g);

void dGeomTriMeshGetTriangle(dGeomID g, int Index, dVector3* v0, dVector3* v1, dVector3* v2);

void dGeomTriMeshGetPoint(dGeomID g, int Index, dReal u, dReal v, dVector3 Out);
int dGeomTriMeshGetTriangleCount (dGeomID g);

void dGeomTriMeshDataUpdate(dTriMeshDataID g);

dGeomID dCreateGeomTransform (dSpaceID space);
void dGeomTransformSetGeom (dGeomID g, dGeomID obj);
dGeomID dGeomTransformGetGeom (dGeomID g);
void dGeomTransformSetCleanup (dGeomID g, int mode);
int dGeomTransformGetCleanup (dGeomID g);
void dGeomTransformSetInfo (dGeomID g, int mode);
int dGeomTransformGetInfo (dGeomID g);

struct dxHeightfieldData;
typedef struct dxHeightfieldData* dHeightfieldDataID;
typedef dReal dHeightfieldGetHeight( void* p_user_data, int x, int z );
dGeomID dCreateHeightfield( dSpaceID space,
     dHeightfieldDataID data, int bPlaceable );
dHeightfieldDataID dGeomHeightfieldDataCreate(void);
void dGeomHeightfieldDataDestroy( dHeightfieldDataID d );
void dGeomHeightfieldDataBuildCallback( dHeightfieldDataID d,
    void* pUserData, dHeightfieldGetHeight* pCallback,
    dReal width, dReal depth, int widthSamples, int depthSamples,
    dReal scale, dReal offset, dReal thickness, int bWrap );
void dGeomHeightfieldDataBuildByte( dHeightfieldDataID d,
    const unsigned char* pHeightData, int bCopyHeightData,
    dReal width, dReal depth, int widthSamples, int depthSamples,
    dReal scale, dReal offset, dReal thickness, int bWrap );
void dGeomHeightfieldDataBuildShort( dHeightfieldDataID d,
    const short* pHeightData, int bCopyHeightData,
    dReal width, dReal depth, int widthSamples, int depthSamples,
    dReal scale, dReal offset, dReal thickness, int bWrap );
void dGeomHeightfieldDataBuildSingle( dHeightfieldDataID d,
    const float* pHeightData, int bCopyHeightData,
    dReal width, dReal depth, int widthSamples, int depthSamples,
    dReal scale, dReal offset, dReal thickness, int bWrap );
void dGeomHeightfieldDataBuildDouble( dHeightfieldDataID d,
    const double* pHeightData, int bCopyHeightData,
    dReal width, dReal depth, int widthSamples, int depthSamples,
    dReal scale, dReal offset, dReal thickness, int bWrap );
void dGeomHeightfieldDataSetBounds( dHeightfieldDataID d,
    dReal minHeight, dReal maxHeight );
void dGeomHeightfieldSetHeightfieldData( dGeomID g, dHeightfieldDataID d );
dHeightfieldDataID dGeomHeightfieldGetHeightfieldData( dGeomID g );

void dClosestLineSegmentPoints (const dVector3 a1, const dVector3 a2,
    const dVector3 b1, const dVector3 b2,
    dVector3 cp1, dVector3 cp2);

int dBoxTouchesBox (const dVector3 _p1, const dMatrix3 R1,
      const dVector3 side1, const dVector3 _p2,
      const dMatrix3 R2, const dVector3 side2);

int dBoxBox (const dVector3 p1, const dMatrix3 R1,
      const dVector3 side1, const dVector3 p2,
      const dMatrix3 R2, const dVector3 side2,
      dVector3 normal, dReal *depth, int *return_code,
      int flags, dContactGeom *contact, int skip);

void dInfiniteAABB (dGeomID geom, dReal aabb[6]);

typedef void dGetAABBFn (dGeomID, dReal aabb[6]);
typedef int dColliderFn (dGeomID o1, dGeomID o2,
    int flags, dContactGeom *contact, int skip);
typedef dColliderFn * dGetColliderFnFn (int num);
typedef void dGeomDtorFn (dGeomID o);
typedef int dAABBTestFn (dGeomID o1, dGeomID o2, dReal aabb[6]);

typedef struct dGeomClass {
  int bytes;
  dGetColliderFnFn *collider;
  dGetAABBFn *aabb;
  dAABBTestFn *aabb_test;
  dGeomDtorFn *dtor;
} dGeomClass;

int dCreateGeomClass (const dGeomClass *classptr);
void * dGeomGetClassData (dGeomID);
dGeomID dCreateGeom (int classnum);
void dSetColliderOverride (int i, int j, dColliderFn *fn);

struct dxThreadingThreadPool;
typedef struct dxThreadingThreadPool *dThreadingThreadPoolID;
dThreadingImplementationID dThreadingAllocateSelfThreadedImplementation();
dThreadingImplementationID dThreadingAllocateMultiThreadedImplementation();
const dThreadingFunctionsInfo *dThreadingImplementationGetFunctions(dThreadingImplementationID impl);
void dThreadingImplementationShutdownProcessing(dThreadingImplementationID impl);
void dThreadingImplementationCleanupForRestart(dThreadingImplementationID impl);
void dThreadingFreeImplementation(dThreadingImplementationID impl);

typedef void (dThreadReadyToServeCallback)(void *callback_context);
void dExternalThreadingServeMultiThreadedImplementation(dThreadingImplementationID impl,
dThreadReadyToServeCallback *readiness_callback , void *callback_context );
dThreadingThreadPoolID dThreadingAllocateThreadPool(unsigned thread_count,
  dsizeint stack_size, unsigned int ode_data_allocate_flags, void *reserved );
void dThreadingThreadPoolServeMultiThreadedImplementation(dThreadingThreadPoolID pool, dThreadingImplementationID impl);
void dThreadingThreadPoolWaitIdleState(dThreadingThreadPoolID pool);
void dThreadingFreeThreadPool(dThreadingThreadPoolID pool);

void dWorldExportDIF (dWorldID w, FILE *file, const char *world_name);

}

class dWorldSimpleIDContainer {
protected:
 dWorldID _id;

 dWorldSimpleIDContainer(): _id(0) {}
 ~dWorldSimpleIDContainer() { destroy(); }

 void destroy() {
  if (_id) {
   dWorldDestroy(_id);
   _id = 0;
  }
 }
};

class dWorldDynamicIDContainer: public dWorldSimpleIDContainer {
protected:
 virtual ~dWorldDynamicIDContainer() {}
};

template <class dWorldTemplateBase>
class dWorldTemplate: public dWorldTemplateBase {

  dWorldTemplate (const dWorldTemplate<dWorldTemplateBase> &);
  void operator= (const dWorldTemplate<dWorldTemplateBase> &);

protected:
  dWorldID get_id() const { return dWorldTemplateBase::_id; }
  void set_id(dWorldID value) { dWorldTemplateBase::_id = value; }

public:
  dWorldTemplate()
    { set_id(dWorldCreate()); }

  dWorldID id() const
    { return get_id(); }
  operator dWorldID() const
    { return get_id(); }

  void setGravity (dReal x, dReal y, dReal z)
    { dWorldSetGravity (get_id(), x, y, z); }
  void setGravity (const dVector3 g)
    { setGravity (g[0], g[1], g[2]); }
  void getGravity (dVector3 g) const
    { dWorldGetGravity (get_id(), g); }

  void setERP (dReal erp)
    { dWorldSetERP(get_id(), erp); }
  dReal getERP() const
    { return dWorldGetERP(get_id()); }

  void setCFM (dReal cfm)
    { dWorldSetCFM(get_id(), cfm); }
  dReal getCFM() const
    { return dWorldGetCFM(get_id()); }

  void step (dReal stepsize)
    { dWorldStep (get_id(), stepsize); }

  void quickStep(dReal stepsize)
    { dWorldQuickStep (get_id(), stepsize); }
  void setQuickStepNumIterations(int num)
    { dWorldSetQuickStepNumIterations (get_id(), num); }
  int getQuickStepNumIterations() const
    { return dWorldGetQuickStepNumIterations (get_id()); }
  void setQuickStepW(dReal over_relaxation)
    { dWorldSetQuickStepW (get_id(), over_relaxation); }
  dReal getQuickStepW() const
    { return dWorldGetQuickStepW (get_id()); }

  void setAutoDisableLinearThreshold (dReal threshold)
    { dWorldSetAutoDisableLinearThreshold (get_id(), threshold); }
  dReal getAutoDisableLinearThreshold() const
    { return dWorldGetAutoDisableLinearThreshold (get_id()); }
  void setAutoDisableAngularThreshold (dReal threshold)
    { dWorldSetAutoDisableAngularThreshold (get_id(), threshold); }
  dReal getAutoDisableAngularThreshold() const
    { return dWorldGetAutoDisableAngularThreshold (get_id()); }
  void setAutoDisableSteps (int steps)
    { dWorldSetAutoDisableSteps (get_id(), steps); }
  int getAutoDisableSteps() const
    { return dWorldGetAutoDisableSteps (get_id()); }
  void setAutoDisableTime (dReal time)
    { dWorldSetAutoDisableTime (get_id(), time); }
  dReal getAutoDisableTime() const
    { return dWorldGetAutoDisableTime (get_id()); }
  void setAutoDisableFlag (int do_auto_disable)
    { dWorldSetAutoDisableFlag (get_id(), do_auto_disable); }
  int getAutoDisableFlag() const
    { return dWorldGetAutoDisableFlag (get_id()); }

  dReal getLinearDampingThreshold() const
    { return dWorldGetLinearDampingThreshold(get_id()); }
  void setLinearDampingThreshold(dReal threshold)
    { dWorldSetLinearDampingThreshold(get_id(), threshold); }
  dReal getAngularDampingThreshold() const
    { return dWorldGetAngularDampingThreshold(get_id()); }
  void setAngularDampingThreshold(dReal threshold)
    { dWorldSetAngularDampingThreshold(get_id(), threshold); }
  dReal getLinearDamping() const
    { return dWorldGetLinearDamping(get_id()); }
  void setLinearDamping(dReal scale)
    { dWorldSetLinearDamping(get_id(), scale); }
  dReal getAngularDamping() const
    { return dWorldGetAngularDamping(get_id()); }
  void setAngularDamping(dReal scale)
    { dWorldSetAngularDamping(get_id(), scale); }
  void setDamping(dReal linear_scale, dReal angular_scale)
    { dWorldSetDamping(get_id(), linear_scale, angular_scale); }

  dReal getMaxAngularSpeed() const
    { return dWorldGetMaxAngularSpeed(get_id()); }
  void setMaxAngularSpeed(dReal max_speed)
    { dWorldSetMaxAngularSpeed(get_id(), max_speed); }

  void setContactSurfaceLayer(dReal depth)
    { dWorldSetContactSurfaceLayer (get_id(), depth); }
  dReal getContactSurfaceLayer() const
    { return dWorldGetContactSurfaceLayer (get_id()); }

  void impulseToForce (dReal stepsize, dReal ix, dReal iy, dReal iz,
         dVector3 force)
    { dWorldImpulseToForce (get_id(), stepsize, ix, iy, iz, force); }
};


class dBodySimpleIDContainer {
protected:
 dBodyID _id;

 dBodySimpleIDContainer(): _id(0) {}
 ~dBodySimpleIDContainer() { destroy(); }

 void destroy() {
  if (_id) {
   dBodyDestroy(_id);
   _id = 0;
  }
 }
};

class dBodyDynamicIDContainer: public dBodySimpleIDContainer {
protected:
 virtual ~dBodyDynamicIDContainer() {}
};

template <class dBodyTemplateBase, class dWorldTemplateBase>
class dBodyTemplate: public dBodyTemplateBase {

  dBodyTemplate (const dBodyTemplate<dBodyTemplateBase, dWorldTemplateBase> &);
  void operator= (const dBodyTemplate<dBodyTemplateBase, dWorldTemplateBase> &);

protected:
  dBodyID get_id() const { return dBodyTemplateBase::_id; }
  void set_id(dBodyID value) { dBodyTemplateBase::_id = value; }

  void destroy() { dBodyTemplateBase::destroy(); }

public:
  dBodyTemplate()
    { }
  dBodyTemplate (dWorldID world)
    { set_id(dBodyCreate(world)); }
  dBodyTemplate (dWorldTemplate<dWorldTemplateBase>& world)
    { set_id(dBodyCreate(world.id())); }

  void create (dWorldID world) {
    destroy();
    set_id(dBodyCreate(world));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world) {
    create(world.id());
  }

  dBodyID id() const
    { return get_id(); }
  operator dBodyID() const
    { return get_id(); }

  void setData (void *data)
    { dBodySetData (get_id(), data); }
  void *getData() const
    { return dBodyGetData (get_id()); }

  void setPosition (dReal x, dReal y, dReal z)
    { dBodySetPosition (get_id(), x, y, z); }
  void setPosition (const dVector3 p)
    { setPosition(p[0], p[1], p[2]); }

  void setRotation (const dMatrix3 R)
    { dBodySetRotation (get_id(), R); }
  void setQuaternion (const dQuaternion q)
    { dBodySetQuaternion (get_id(), q); }
  void setLinearVel (dReal x, dReal y, dReal z)
    { dBodySetLinearVel (get_id(), x, y, z); }
  void setLinearVel (const dVector3 v)
    { setLinearVel(v[0], v[1], v[2]); }
  void setAngularVel (dReal x, dReal y, dReal z)
    { dBodySetAngularVel (get_id(), x, y, z); }
  void setAngularVel (const dVector3 v)
    { setAngularVel (v[0], v[1], v[2]); }

  const dReal * getPosition() const
    { return dBodyGetPosition (get_id()); }
  const dReal * getRotation() const
    { return dBodyGetRotation (get_id()); }
  const dReal * getQuaternion() const
    { return dBodyGetQuaternion (get_id()); }
  const dReal * getLinearVel() const
    { return dBodyGetLinearVel (get_id()); }
  const dReal * getAngularVel() const
    { return dBodyGetAngularVel (get_id()); }

  void setMass (const dMass *mass)
    { dBodySetMass (get_id(), mass); }
  void setMass (const dMass &mass)
    { setMass (&mass); }
  dMass getMass () const
    { dMass mass; dBodyGetMass (get_id(), &mass); return mass; }

  void addForce (dReal fx, dReal fy, dReal fz)
    { dBodyAddForce (get_id(), fx, fy, fz); }
  void addForce (const dVector3 f)
    { addForce (f[0], f[1], f[2]); }
  void addTorque (dReal fx, dReal fy, dReal fz)
    { dBodyAddTorque (get_id(), fx, fy, fz); }
  void addTorque (const dVector3 t)
    { addTorque(t[0], t[1], t[2]); }

  void addRelForce (dReal fx, dReal fy, dReal fz)
    { dBodyAddRelForce (get_id(), fx, fy, fz); }
  void addRelForce (const dVector3 f)
    { addRelForce (f[0], f[1], f[2]); }
  void addRelTorque (dReal fx, dReal fy, dReal fz)
    { dBodyAddRelTorque (get_id(), fx, fy, fz); }
  void addRelTorque (const dVector3 t)
    { addRelTorque (t[0], t[1], t[2]); }

  void addForceAtPos (dReal fx, dReal fy, dReal fz,
        dReal px, dReal py, dReal pz)
    { dBodyAddForceAtPos (get_id(), fx, fy, fz, px, py, pz); }
  void addForceAtPos (const dVector3 f, const dVector3 p)
    { addForceAtPos (f[0], f[1], f[2], p[0], p[1], p[2]); }

  void addForceAtRelPos (dReal fx, dReal fy, dReal fz,
                         dReal px, dReal py, dReal pz)
    { dBodyAddForceAtRelPos (get_id(), fx, fy, fz, px, py, pz); }
  void addForceAtRelPos (const dVector3 f, const dVector3 p)
    { addForceAtRelPos (f[0], f[1], f[2], p[0], p[1], p[2]); }

  void addRelForceAtPos (dReal fx, dReal fy, dReal fz,
    dReal px, dReal py, dReal pz)
    { dBodyAddRelForceAtPos (get_id(), fx, fy, fz, px, py, pz); }
  void addRelForceAtPos (const dVector3 f, const dVector3 p)
    { addRelForceAtPos (f[0], f[1], f[2], p[0], p[1], p[2]); }

  void addRelForceAtRelPos (dReal fx, dReal fy, dReal fz,
       dReal px, dReal py, dReal pz)
    { dBodyAddRelForceAtRelPos (get_id(), fx, fy, fz, px, py, pz); }
  void addRelForceAtRelPos (const dVector3 f, const dVector3 p)
    { addRelForceAtRelPos (f[0], f[1], f[2], p[0], p[1], p[2]); }

  const dReal * getForce() const
    { return dBodyGetForce(get_id()); }
  const dReal * getTorque() const
    { return dBodyGetTorque(get_id()); }
  void setForce (dReal x, dReal y, dReal z)
    { dBodySetForce (get_id(), x, y, z); }
  void setForce (const dVector3 f)
    { setForce (f[0], f[1], f[2]); }
  void setTorque (dReal x, dReal y, dReal z)
    { dBodySetTorque (get_id(), x, y, z); }
  void setTorque (const dVector3 t)
  { setTorque (t[0], t[1], t[2]); }

  void setDynamic()
    { dBodySetDynamic (get_id()); }
  void setKinematic()
    { dBodySetKinematic (get_id()); }
  bool isKinematic() const
    { return dBodyIsKinematic (get_id()) != 0; }

  void enable()
    { dBodyEnable (get_id()); }
  void disable()
    { dBodyDisable (get_id()); }
  bool isEnabled() const
    { return dBodyIsEnabled (get_id()) != 0; }

  void getRelPointPos (dReal px, dReal py, dReal pz, dVector3 result) const
    { dBodyGetRelPointPos (get_id(), px, py, pz, result); }
  void getRelPointPos (const dVector3 p, dVector3 result) const
    { getRelPointPos (p[0], p[1], p[2], result); }

  void getRelPointVel (dReal px, dReal py, dReal pz, dVector3 result) const
    { dBodyGetRelPointVel (get_id(), px, py, pz, result); }
  void getRelPointVel (const dVector3 p, dVector3 result) const
    { getRelPointVel (p[0], p[1], p[2], result); }

  void getPointVel (dReal px, dReal py, dReal pz, dVector3 result) const
    { dBodyGetPointVel (get_id(), px, py, pz, result); }
  void getPointVel (const dVector3 p, dVector3 result) const
    { getPointVel (p[0], p[1], p[2], result); }

  void getPosRelPoint (dReal px, dReal py, dReal pz, dVector3 result) const
    { dBodyGetPosRelPoint (get_id(), px, py, pz, result); }
  void getPosRelPoint (const dVector3 p, dVector3 result) const
    { getPosRelPoint (p[0], p[1], p[2], result); }

  void vectorToWorld (dReal px, dReal py, dReal pz, dVector3 result) const
    { dBodyVectorToWorld (get_id(), px, py, pz, result); }
  void vectorToWorld (const dVector3 p, dVector3 result) const
    { vectorToWorld (p[0], p[1], p[2], result); }

  void vectorFromWorld (dReal px, dReal py, dReal pz, dVector3 result) const
    { dBodyVectorFromWorld (get_id(), px, py, pz, result); }
  void vectorFromWorld (const dVector3 p, dVector3 result) const
    { vectorFromWorld (p[0], p[1], p[2], result); }

  void setFiniteRotationMode (bool mode)
    { dBodySetFiniteRotationMode (get_id(), mode); }

  void setFiniteRotationAxis (dReal x, dReal y, dReal z)
    { dBodySetFiniteRotationAxis (get_id(), x, y, z); }
  void setFiniteRotationAxis (const dVector3 a)
    { setFiniteRotationAxis (a[0], a[1], a[2]); }

  bool getFiniteRotationMode() const
    { return dBodyGetFiniteRotationMode (get_id()) != 0; }
  void getFiniteRotationAxis (dVector3 result) const
    { dBodyGetFiniteRotationAxis (get_id(), result); }

  int getNumJoints() const
    { return dBodyGetNumJoints (get_id()); }
  dJointID getJoint (int index) const
    { return dBodyGetJoint (get_id(), index); }

  void setGravityMode (bool mode)
    { dBodySetGravityMode (get_id(), mode); }
  bool getGravityMode() const
    { return dBodyGetGravityMode (get_id()) != 0; }

  bool isConnectedTo (dBodyID body) const
    { return dAreConnected (get_id(), body) != 0; }

  void setAutoDisableLinearThreshold (dReal threshold)
    { dBodySetAutoDisableLinearThreshold (get_id(), threshold); }
  dReal getAutoDisableLinearThreshold() const
    { return dBodyGetAutoDisableLinearThreshold (get_id()); }
  void setAutoDisableAngularThreshold (dReal threshold)
    { dBodySetAutoDisableAngularThreshold (get_id(), threshold); }
  dReal getAutoDisableAngularThreshold() const
    { return dBodyGetAutoDisableAngularThreshold (get_id()); }
  void setAutoDisableSteps (int steps)
    { dBodySetAutoDisableSteps (get_id(), steps); }
  int getAutoDisableSteps() const
    { return dBodyGetAutoDisableSteps (get_id()); }
  void setAutoDisableTime (dReal time)
    { dBodySetAutoDisableTime (get_id(), time); }
  dReal getAutoDisableTime() const
    { return dBodyGetAutoDisableTime (get_id()); }
  void setAutoDisableFlag (bool do_auto_disable)
    { dBodySetAutoDisableFlag (get_id(), do_auto_disable); }
  bool getAutoDisableFlag() const
    { return dBodyGetAutoDisableFlag (get_id()) != 0; }

  dReal getLinearDamping() const
    { return dBodyGetLinearDamping(get_id()); }
  void setLinearDamping(dReal scale)
    { dBodySetLinearDamping(get_id(), scale); }
  dReal getAngularDamping() const
    { return dBodyGetAngularDamping(get_id()); }
  void setAngularDamping(dReal scale)
    { dBodySetAngularDamping(get_id(), scale); }
  void setDamping(dReal linear_scale, dReal angular_scale)
    { dBodySetDamping(get_id(), linear_scale, angular_scale); }
  dReal getLinearDampingThreshold() const
    { return dBodyGetLinearDampingThreshold(get_id()); }
  void setLinearDampingThreshold(dReal threshold) const
    { dBodySetLinearDampingThreshold(get_id(), threshold); }
  dReal getAngularDampingThreshold() const
    { return dBodyGetAngularDampingThreshold(get_id()); }
  void setAngularDampingThreshold(dReal threshold)
    { dBodySetAngularDampingThreshold(get_id(), threshold); }
  void setDampingDefaults()
    { dBodySetDampingDefaults(get_id()); }

  dReal getMaxAngularSpeed() const
    { return dBodyGetMaxAngularSpeed(get_id()); }
  void setMaxAngularSpeed(dReal max_speed)
    { dBodySetMaxAngularSpeed(get_id(), max_speed); }

  bool getGyroscopicMode() const
    { return dBodyGetGyroscopicMode(get_id()) != 0; }
  void setGyroscopicMode(bool mode)
    { dBodySetGyroscopicMode(get_id(), mode); }

};


class dJointGroupSimpleIDContainer {
protected:
 dJointGroupID _id;

 dJointGroupSimpleIDContainer(): _id(0) {}
 ~dJointGroupSimpleIDContainer() { destroy(); }

 void destroy() {
  if (_id) {
   dJointGroupDestroy(_id);
   _id = 0;
  }
 }
};

class dJointGroupDynamicIDContainer: public dJointGroupSimpleIDContainer {
protected:
 virtual ~dJointGroupDynamicIDContainer() {}
};

template <class dJointGroupTemplateBase>
class dJointGroupTemplate: public dJointGroupTemplateBase {

  dJointGroupTemplate (const dJointGroupTemplate<dJointGroupTemplateBase> &);
  void operator= (const dJointGroupTemplate<dJointGroupTemplateBase> &);

protected:
  dJointGroupID get_id() const { return dJointGroupTemplateBase::_id; }
  void set_id(dJointGroupID value) { dJointGroupTemplateBase::_id = value; }

  void destroy() { dJointGroupTemplateBase::destroy(); }

public:
  dJointGroupTemplate ()
    { set_id(dJointGroupCreate(0)); }

  void create () {
    destroy();
    set_id(dJointGroupCreate(0));
  }

  dJointGroupID id() const
    { return get_id(); }
  operator dJointGroupID() const
    { return get_id(); }

  void empty()
    { dJointGroupEmpty (get_id()); }
  void clear()
    { empty(); }
};


class dJointSimpleIDContainer {
protected:
 dJointID _id;

 dJointSimpleIDContainer(): _id(0) {}
 ~dJointSimpleIDContainer() { destroy(); }

 void destroy() {
  if (_id) {
   dJointDestroy (_id);
   _id = 0;
  }
 }
};

class dJointDynamicIDContainer: public dJointSimpleIDContainer {
protected:
 virtual ~dJointDynamicIDContainer() {}
};

template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dJointTemplate: public dJointTemplateBase {
private:

  dJointTemplate (const dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &) ;
  void operator= (const dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  dJointID get_id() const { return dJointTemplateBase::_id; }
  void set_id(dJointID value) { dJointTemplateBase::_id = value; }

  void destroy() { dJointTemplateBase::destroy(); }

protected:
  dJointTemplate()
    { }

public:
  dJointID id() const
    { return get_id(); }
  operator dJointID() const
    { return get_id(); }

  int getNumBodies() const
    { return dJointGetNumBodies(get_id()); }

  void attach (dBodyID body1, dBodyID body2)
    { dJointAttach (get_id(), body1, body2); }
  void attach (dBodyTemplate<dBodyTemplateBase, dWorldTemplateBase>& body1, dBodyTemplate<dBodyTemplateBase, dWorldTemplateBase>& body2)
    { attach(body1.id(), body2.id()); }

  void enable()
    { dJointEnable (get_id()); }
  void disable()
    { dJointDisable (get_id()); }
  bool isEnabled() const
    { return dJointIsEnabled (get_id()) != 0; }

  void setData (void *data)
    { dJointSetData (get_id(), data); }
  void *getData() const
    { return dJointGetData (get_id()); }

  dJointType getType() const
    { return dJointGetType (get_id()); }

  dBodyID getBody (int index) const
    { return dJointGetBody (get_id(), index); }

  void setFeedback(dJointFeedback *fb)
    { dJointSetFeedback(get_id(), fb); }
  dJointFeedback *getFeedback() const
    { return dJointGetFeedback(get_id()); }


  virtual void setParam (int, dReal) {};
  virtual dReal getParam (int) const { return 0; }
};


template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dBallJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> {
private:

  dBallJointTemplate (const dBallJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator= (const dBallJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dBallJointTemplate() { }
  dBallJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreateBall(world, group)); }
  dBallJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreateBall(world.id(), group)); }

  void create (dWorldID world, dJointGroupID group=0) {
    destroy();
    set_id(dJointCreateBall(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }

  void setAnchor (dReal x, dReal y, dReal z)
    { dJointSetBallAnchor (get_id(), x, y, z); }
  void setAnchor (const dVector3 a)
    { setAnchor (a[0], a[1], a[2]); }
  void getAnchor (dVector3 result) const
    { dJointGetBallAnchor (get_id(), result); }
  void getAnchor2 (dVector3 result) const
    { dJointGetBallAnchor2 (get_id(), result); }
  virtual void setParam (int parameter, dReal value)
    { dJointSetBallParam (get_id(), parameter, value); }
  virtual dReal getParam (int parameter) const
    { return dJointGetBallParam (get_id(), parameter); }

} ;


template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dHingeJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> {
private:

  dHingeJointTemplate (const dHingeJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dHingeJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dHingeJointTemplate() { }
  dHingeJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreateHinge(world, group)); }
  dHingeJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreateHinge(world.id(), group)); }

  void create (dWorldID world, dJointGroupID group=0) {
    destroy();
    set_id(dJointCreateHinge (world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }

  void setAnchor (dReal x, dReal y, dReal z)
    { dJointSetHingeAnchor (get_id(), x, y, z); }
  void setAnchor (const dVector3 a)
    { setAnchor (a[0], a[1], a[2]); }
  void getAnchor (dVector3 result) const
    { dJointGetHingeAnchor (get_id(), result); }
  void getAnchor2 (dVector3 result) const
    { dJointGetHingeAnchor2 (get_id(), result); }

  void setAxis (dReal x, dReal y, dReal z)
    { dJointSetHingeAxis (get_id(), x, y, z); }
  void setAxis (const dVector3 a)
    { setAxis(a[0], a[1], a[2]); }
  void getAxis (dVector3 result) const
    { dJointGetHingeAxis (get_id(), result); }

  dReal getAngle() const
    { return dJointGetHingeAngle (get_id()); }
  dReal getAngleRate() const
    { return dJointGetHingeAngleRate (get_id()); }

  virtual void setParam (int parameter, dReal value)
    { dJointSetHingeParam (get_id(), parameter, value); }
  virtual dReal getParam (int parameter) const
    { return dJointGetHingeParam (get_id(), parameter); }


  void addTorque (dReal torque)
 { dJointAddHingeTorque(get_id(), torque); }
};


template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dSliderJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> {
private:

  dSliderJointTemplate (const dSliderJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dSliderJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dSliderJointTemplate() { }
  dSliderJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreateSlider(world, group)); }
  dSliderJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreateSlider(world.id(), group)); }

  void create (dWorldID world, dJointGroupID group=0) {
    destroy();
    set_id(dJointCreateSlider(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }

  void setAxis (dReal x, dReal y, dReal z)
    { dJointSetSliderAxis (get_id(), x, y, z); }
  void setAxis (const dVector3 a)
    { setAxis (a[0], a[1], a[2]); }
  void getAxis (dVector3 result) const
    { dJointGetSliderAxis (get_id(), result); }

  dReal getPosition() const
    { return dJointGetSliderPosition (get_id()); }
  dReal getPositionRate() const
    { return dJointGetSliderPositionRate (get_id()); }

  virtual void setParam (int parameter, dReal value)
    { dJointSetSliderParam (get_id(), parameter, value); }
  virtual dReal getParam (int parameter) const
    { return dJointGetSliderParam (get_id(), parameter); }


  void addForce (dReal force)
 { dJointAddSliderForce(get_id(), force); }
};


template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dUniversalJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> {
private:

  dUniversalJointTemplate (const dUniversalJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dUniversalJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dUniversalJointTemplate() { }
  dUniversalJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreateUniversal(world, group)); }
  dUniversalJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreateUniversal(world.id(), group)); }

  void create (dWorldID world, dJointGroupID group=0) {
    destroy();
    set_id(dJointCreateUniversal(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }

  void setAnchor (dReal x, dReal y, dReal z)
    { dJointSetUniversalAnchor (get_id(), x, y, z); }
  void setAnchor (const dVector3 a)
    { setAnchor(a[0], a[1], a[2]); }
  void setAxis1 (dReal x, dReal y, dReal z)
    { dJointSetUniversalAxis1 (get_id(), x, y, z); }
  void setAxis1 (const dVector3 a)
    { setAxis1 (a[0], a[1], a[2]); }
  void setAxis2 (dReal x, dReal y, dReal z)
    { dJointSetUniversalAxis2 (get_id(), x, y, z); }
  void setAxis2 (const dVector3 a)
    { setAxis2 (a[0], a[1], a[2]); }

  void getAnchor (dVector3 result) const
    { dJointGetUniversalAnchor (get_id(), result); }
  void getAnchor2 (dVector3 result) const
    { dJointGetUniversalAnchor2 (get_id(), result); }
  void getAxis1 (dVector3 result) const
    { dJointGetUniversalAxis1 (get_id(), result); }
  void getAxis2 (dVector3 result) const
    { dJointGetUniversalAxis2 (get_id(), result); }

  virtual void setParam (int parameter, dReal value)
    { dJointSetUniversalParam (get_id(), parameter, value); }
  virtual dReal getParam (int parameter) const
    { return dJointGetUniversalParam (get_id(), parameter); }


  void getAngles(dReal *angle1, dReal *angle2) const
    { dJointGetUniversalAngles (get_id(), angle1, angle2); }

  dReal getAngle1() const
    { return dJointGetUniversalAngle1 (get_id()); }
  dReal getAngle1Rate() const
    { return dJointGetUniversalAngle1Rate (get_id()); }
  dReal getAngle2() const
    { return dJointGetUniversalAngle2 (get_id()); }
  dReal getAngle2Rate() const
    { return dJointGetUniversalAngle2Rate (get_id()); }

  void addTorques (dReal torque1, dReal torque2)
 { dJointAddUniversalTorques(get_id(), torque1, torque2); }
};


template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dHinge2JointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> {
private:

  dHinge2JointTemplate (const dHinge2JointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dHinge2JointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dHinge2JointTemplate() { }
  dHinge2JointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreateHinge2(world, group)); }
  dHinge2JointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreateHinge2(world.id(), group)); }

  void create (dWorldID world, dJointGroupID group=0) {
    destroy();
    set_id(dJointCreateHinge2(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }

  void setAnchor (dReal x, dReal y, dReal z)
    { dJointSetHinge2Anchor (get_id(), x, y, z); }
  void setAnchor (const dVector3 a)
    { setAnchor(a[0], a[1], a[2]); }
  void setAxes (const dReal *axis1 , const dReal *axis2 )
    { dJointSetHinge2Axes (get_id(), axis1, axis2); }
  void setAxis1 (dReal x, dReal y, dReal z)
    { dVector3 a = { x, y, z }; dJointSetHinge2Axes (get_id(), a,
                                                                 __null
                                                                     ); }
  void setAxis1 (const dVector3 a)
    { dJointSetHinge2Axes (get_id(), a,
                                       __null
                                           ); }
  void setAxis2 (dReal x, dReal y, dReal z)
    { dVector3 a = { x, y, z }; dJointSetHinge2Axes (get_id(),
                                                              __null
                                                                  , a); }
  void setAxis2 (const dVector3 a)
    { dJointSetHinge2Axes (get_id(),
                                    __null
                                        , a); }

  void getAnchor (dVector3 result) const
    { dJointGetHinge2Anchor (get_id(), result); }
  void getAnchor2 (dVector3 result) const
    { dJointGetHinge2Anchor2 (get_id(), result); }
  void getAxis1 (dVector3 result) const
    { dJointGetHinge2Axis1 (get_id(), result); }
  void getAxis2 (dVector3 result) const
    { dJointGetHinge2Axis2 (get_id(), result); }

  dReal getAngle1() const
    { return dJointGetHinge2Angle1 (get_id()); }
  dReal getAngle1Rate() const
    { return dJointGetHinge2Angle1Rate (get_id()); }
  dReal getAngle2Rate() const
    { return dJointGetHinge2Angle2Rate (get_id()); }

  virtual void setParam (int parameter, dReal value)
    { dJointSetHinge2Param (get_id(), parameter, value); }
  virtual dReal getParam (int parameter) const
    { return dJointGetHinge2Param (get_id(), parameter); }


  void addTorques(dReal torque1, dReal torque2)
 { dJointAddHinge2Torques(get_id(), torque1, torque2); }
};


template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dPRJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> {
private:

  dPRJointTemplate (const dPRJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dPRJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dPRJointTemplate() { }
  dPRJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreatePR(world, group)); }
  dPRJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreatePR(world.id(), group)); }

  void create (dWorldID world, dJointGroupID group=0) {
    destroy();
    set_id(dJointCreatePR(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }

  void setAnchor (dReal x, dReal y, dReal z)
    { dJointSetPRAnchor (get_id(), x, y, z); }
  void setAnchor (const dVector3 a)
    { setAnchor (a[0], a[1], a[2]); }
  void setAxis1 (dReal x, dReal y, dReal z)
    { dJointSetPRAxis1 (get_id(), x, y, z); }
  void setAxis1 (const dVector3 a)
    { setAxis1(a[0], a[1], a[2]); }
  void setAxis2 (dReal x, dReal y, dReal z)
    { dJointSetPRAxis2 (get_id(), x, y, z); }
  void setAxis2 (const dVector3 a)
    { setAxis2(a[0], a[1], a[2]); }

  void getAnchor (dVector3 result) const
    { dJointGetPRAnchor (get_id(), result); }
  void getAxis1 (dVector3 result) const
    { dJointGetPRAxis1 (get_id(), result); }
  void getAxis2 (dVector3 result) const
    { dJointGetPRAxis2 (get_id(), result); }

  dReal getPosition() const
    { return dJointGetPRPosition (get_id()); }
  dReal getPositionRate() const
    { return dJointGetPRPositionRate (get_id()); }

  dReal getAngle() const
    { return dJointGetPRAngle (get_id()); }
  dReal getAngleRate() const
    { return dJointGetPRAngleRate (get_id()); }

  virtual void setParam (int parameter, dReal value)
    { dJointSetPRParam (get_id(), parameter, value); }
  virtual dReal getParam (int parameter) const
    { return dJointGetPRParam (get_id(), parameter); }
};



template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dPUJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase>
{
private:

  dPUJointTemplate (const dPUJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dPUJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dPUJointTemplate() { }
  dPUJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreatePU(world, group)); }
  dPUJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreatePU(world.id(), group)); }

  void create (dWorldID world, dJointGroupID group=0)
  {
    destroy();
    set_id(dJointCreatePU(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
  { create(world.id(), group); }

  void setAnchor (dReal x, dReal y, dReal z)
    { dJointSetPUAnchor (get_id(), x, y, z); }
  void setAnchor (const dVector3 a)
    { setAnchor (a[0], a[1], a[2]); }
  void setAxis1 (dReal x, dReal y, dReal z)
    { dJointSetPUAxis1 (get_id(), x, y, z); }
  void setAxis1 (const dVector3 a)
    { setAxis1(a[0], a[1], a[2]); }
  void setAxis2 (dReal x, dReal y, dReal z)
  { dJointSetPUAxis2 (get_id(), x, y, z); }
  void setAxis3 (dReal x, dReal y, dReal z)
  { dJointSetPUAxis3 (get_id(), x, y, z); }
  void setAxis3 (const dVector3 a)
    { setAxis3(a[0], a[1], a[2]); }
  void setAxisP (dReal x, dReal y, dReal z)
  { dJointSetPUAxis3 (get_id(), x, y, z); }
  void setAxisP (const dVector3 a)
    { setAxisP(a[0], a[1], a[2]); }

  virtual void getAnchor (dVector3 result) const
    { dJointGetPUAnchor (get_id(), result); }
  void getAxis1 (dVector3 result) const
    { dJointGetPUAxis1 (get_id(), result); }
  void getAxis2 (dVector3 result) const
    { dJointGetPUAxis2 (get_id(), result); }
  void getAxis3 (dVector3 result) const
    { dJointGetPUAxis3 (get_id(), result); }
  void getAxisP (dVector3 result) const
    { dJointGetPUAxis3 (get_id(), result); }

  dReal getAngle1() const
    { return dJointGetPUAngle1 (get_id()); }
  dReal getAngle1Rate() const
    { return dJointGetPUAngle1Rate (get_id()); }
  dReal getAngle2() const
    { return dJointGetPUAngle2 (get_id()); }
  dReal getAngle2Rate() const
    { return dJointGetPUAngle2Rate (get_id()); }

  dReal getPosition() const
    { return dJointGetPUPosition (get_id()); }
  dReal getPositionRate() const
    { return dJointGetPUPositionRate (get_id()); }

  virtual void setParam (int parameter, dReal value)
  { dJointSetPUParam (get_id(), parameter, value); }
  virtual dReal getParam (int parameter) const
    { return dJointGetPUParam (get_id(), parameter); }

};





template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dPistonJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase>
{
private:

  dPistonJointTemplate (const dPistonJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dPistonJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dPistonJointTemplate() { }
  dPistonJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreatePiston(world, group)); }
  dPistonJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreatePiston(world, group)); }

  void create (dWorldID world, dJointGroupID group=0)
  {
    destroy();
    set_id(dJointCreatePiston(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }

  void setAnchor (dReal x, dReal y, dReal z)
    { dJointSetPistonAnchor (get_id(), x, y, z); }
  void setAnchor (const dVector3 a)
    { setAnchor (a[0], a[1], a[2]); }
  void getAnchor (dVector3 result) const
    { dJointGetPistonAnchor (get_id(), result); }
  void getAnchor2 (dVector3 result) const
    { dJointGetPistonAnchor2 (get_id(), result); }

  void setAxis (dReal x, dReal y, dReal z)
    { dJointSetPistonAxis (get_id(), x, y, z); }
  void setAxis (const dVector3 a)
    { setAxis(a[0], a[1], a[2]); }
  void getAxis (dVector3 result) const
    { dJointGetPistonAxis (get_id(), result); }

  dReal getPosition() const
    { return dJointGetPistonPosition (get_id()); }
  dReal getPositionRate() const
    { return dJointGetPistonPositionRate (get_id()); }

  virtual void setParam (int parameter, dReal value)
  { dJointSetPistonParam (get_id(), parameter, value); }
  virtual dReal getParam (int parameter) const
    { return dJointGetPistonParam (get_id(), parameter); }


  void addForce (dReal force)
  { dJointAddPistonForce (get_id(), force); }
};



template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dFixedJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase>
{
private:

  dFixedJointTemplate (const dFixedJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dFixedJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dFixedJointTemplate() { }
  dFixedJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreateFixed(world, group)); }
  dFixedJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreateFixed(world, group)); }

  void create (dWorldID world, dJointGroupID group=0) {
    destroy();
    set_id(dJointCreateFixed(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }

  void set()
    { dJointSetFixed (get_id()); }

  virtual void setParam (int parameter, dReal value)
    { dJointSetFixedParam (get_id(), parameter, value); }

  virtual dReal getParam (int parameter) const
    { return dJointGetFixedParam (get_id(), parameter); }

};


template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dContactJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> {
private:

  dContactJointTemplate (const dContactJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dContactJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dContactJointTemplate() { }
  dContactJointTemplate (dWorldID world, dJointGroupID group, dContact *contact)
    { set_id(dJointCreateContact(world, group, contact)); }
  dContactJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group, dContact *contact)
    { set_id(dJointCreateContact(world.id(), group, contact)); }

  void create (dWorldID world, dJointGroupID group, dContact *contact) {
    destroy();
    set_id(dJointCreateContact(world, group, contact));
  }

  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group, dContact *contact)
    { create(world.id(), group, contact); }
};


template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dNullJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> {
private:

  dNullJointTemplate (const dNullJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dNullJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dNullJointTemplate() { }
  dNullJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreateNull(world, group)); }
  dNullJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreateNull (world.id(), group)); }

  void create (dWorldID world, dJointGroupID group=0) {
    destroy();
    set_id(dJointCreateNull(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }
};


template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dAMotorJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> {
private:

  dAMotorJointTemplate (const dAMotorJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dAMotorJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dAMotorJointTemplate() { }
  dAMotorJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreateAMotor(world, group)); }
  dAMotorJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreateAMotor(world.id(), group)); }

  void create (dWorldID world, dJointGroupID group=0) {
    destroy();
    set_id(dJointCreateAMotor(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }

  void setMode (int mode)
    { dJointSetAMotorMode (get_id(), mode); }
  int getMode() const
    { return dJointGetAMotorMode (get_id()); }

  void setNumAxes (int num)
    { dJointSetAMotorNumAxes (get_id(), num); }
  int getNumAxes() const
    { return dJointGetAMotorNumAxes (get_id()); }

  void setAxis (int anum, int rel, dReal x, dReal y, dReal z)
    { dJointSetAMotorAxis (get_id(), anum, rel, x, y, z); }
  void setAxis (int anum, int rel, const dVector3 a)
    { setAxis(anum, rel, a[0], a[1], a[2]); }
  void getAxis (int anum, dVector3 result) const
    { dJointGetAMotorAxis (get_id(), anum, result); }
  int getAxisRel (int anum) const
    { return dJointGetAMotorAxisRel (get_id(), anum); }

  void setAngle (int anum, dReal angle)
    { dJointSetAMotorAngle (get_id(), anum, angle); }
  dReal getAngle (int anum) const
    { return dJointGetAMotorAngle (get_id(), anum); }
  dReal getAngleRate (int anum)
    { return dJointGetAMotorAngleRate (get_id(), anum); }

  void setParam (int parameter, dReal value)
    { dJointSetAMotorParam (get_id(), parameter, value); }
  dReal getParam (int parameter) const
    { return dJointGetAMotorParam (get_id(), parameter); }


  void addTorques(dReal torque1, dReal torque2, dReal torque3)
 { dJointAddAMotorTorques(get_id(), torque1, torque2, torque3); }
};


template <class dJointTemplateBase, class dWorldTemplateBase, class dBodyTemplateBase>
class dLMotorJointTemplate : public dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> {
private:

  dLMotorJointTemplate (const dLMotorJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);
  void operator = (const dLMotorJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> &);

protected:
  typedef dJointTemplate<dJointTemplateBase, dWorldTemplateBase, dBodyTemplateBase> dBaseTemplate;

  dJointID get_id() const { return dBaseTemplate::get_id(); }
  void set_id(dJointID value) { dBaseTemplate::set_id(value); }

  void destroy() { dBaseTemplate::destroy(); }

public:
  dLMotorJointTemplate() { }
  dLMotorJointTemplate (dWorldID world, dJointGroupID group=0)
    { set_id(dJointCreateLMotor(world, group)); }
  dLMotorJointTemplate (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { set_id(dJointCreateLMotor(world.id(), group)); }

  void create (dWorldID world, dJointGroupID group=0) {
    destroy();
    set_id(dJointCreateLMotor(world, group));
  }
  void create (dWorldTemplate<dWorldTemplateBase>& world, dJointGroupID group=0)
    { create(world.id(), group); }

  void setNumAxes (int num)
    { dJointSetLMotorNumAxes (get_id(), num); }
  int getNumAxes() const
    { return dJointGetLMotorNumAxes (get_id()); }

  void setAxis (int anum, int rel, dReal x, dReal y, dReal z)
    { dJointSetLMotorAxis (get_id(), anum, rel, x, y, z); }
  void setAxis (int anum, int rel, const dVector3 a)
    { setAxis(anum, rel, a[0], a[1], a[2]); }
  void getAxis (int anum, dVector3 result) const
    { dJointGetLMotorAxis (get_id(), anum, result); }

  void setParam (int parameter, dReal value)
    { dJointSetLMotorParam (get_id(), parameter, value); }
  dReal getParam (int parameter) const
    { return dJointGetLMotorParam (get_id(), parameter); }

};
typedef dWorldTemplate<dWorldDynamicIDContainer> dWorld;
typedef dBodyTemplate<dBodyDynamicIDContainer, dWorldDynamicIDContainer> dBody;
typedef dJointGroupTemplate<dJointGroupDynamicIDContainer> dJointGroup;
typedef dJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dJoint;
typedef dBallJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dBallJoint;
typedef dHingeJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dHingeJoint;
typedef dSliderJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dSliderJoint;
typedef dUniversalJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dUniversalJoint;
typedef dHinge2JointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dHinge2Joint;
typedef dPRJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dPRJoint;
typedef dPUJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dPUJoint;
typedef dPistonJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dPistonJoint;
typedef dFixedJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dFixedJoint;
typedef dContactJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dContactJoint;
typedef dNullJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dNullJoint;
typedef dAMotorJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dAMotorJoint;
typedef dLMotorJointTemplate<dJointDynamicIDContainer, dWorldDynamicIDContainer, dBodyDynamicIDContainer> dLMotorJoint;
class dGeom {

  dGeom (dGeom &);
  void operator= (dGeom &);

protected:
  dGeomID _id;

  dGeom()
    { _id = 0; }
public:
  ~dGeom()
    { if (_id) dGeomDestroy (_id); }

  dGeomID id() const
    { return _id; }
  operator dGeomID() const
    { return _id; }

  void destroy() {
    if (_id) dGeomDestroy (_id);
    _id = 0;
  }

  int getClass() const
    { return dGeomGetClass (_id); }

  dSpaceID getSpace() const
    { return dGeomGetSpace (_id); }

  void setData (void *data)
    { dGeomSetData (_id,data); }
  void *getData() const
    { return dGeomGetData (_id); }

  void setBody (dBodyID b)
    { dGeomSetBody (_id,b); }
  dBodyID getBody() const
    { return dGeomGetBody (_id); }

  void setPosition (dReal x, dReal y, dReal z)
    { dGeomSetPosition (_id,x,y,z); }
  const dReal * getPosition() const
    { return dGeomGetPosition (_id); }

  void setRotation (const dMatrix3 R)
    { dGeomSetRotation (_id,R); }
  const dReal * getRotation() const
    { return dGeomGetRotation (_id); }

  void setQuaternion (const dQuaternion quat)
    { dGeomSetQuaternion (_id,quat); }
  void getQuaternion (dQuaternion quat) const
    { dGeomGetQuaternion (_id,quat); }

  void getAABB (dReal aabb[6]) const
    { dGeomGetAABB (_id, aabb); }

  int isSpace()
    { return dGeomIsSpace (_id); }

  void setCategoryBits (unsigned long bits)
    { dGeomSetCategoryBits (_id, bits); }
  void setCollideBits (unsigned long bits)
    { dGeomSetCollideBits (_id, bits); }
  unsigned long getCategoryBits()
    { return dGeomGetCategoryBits (_id); }
  unsigned long getCollideBits()
    { return dGeomGetCollideBits (_id); }

  void enable()
    { dGeomEnable (_id); }
  void disable()
    { dGeomDisable (_id); }
  int isEnabled()
    { return dGeomIsEnabled (_id); }

  void getRelPointPos (dReal px, dReal py, dReal pz, dVector3 result) const
    { dGeomGetRelPointPos (_id, px, py, pz, result); }
  void getRelPointPos (const dVector3 p, dVector3 result) const
    { getRelPointPos (p[0], p[1], p[2], result); }

  void getPosRelPoint (dReal px, dReal py, dReal pz, dVector3 result) const
    { dGeomGetPosRelPoint (_id, px, py, pz, result); }
  void getPosRelPoint (const dVector3 p, dVector3 result) const
    { getPosRelPoint (p[0], p[1], p[2], result); }

  void vectorToWorld (dReal px, dReal py, dReal pz, dVector3 result) const
    { dGeomVectorToWorld (_id, px, py, pz, result); }
  void vectorToWorld (const dVector3 p, dVector3 result) const
    { vectorToWorld (p[0], p[1], p[2], result); }

  void vectorFromWorld (dReal px, dReal py, dReal pz, dVector3 result) const
    { dGeomVectorFromWorld (_id, px, py, pz, result); }
  void vectorFromWorld (const dVector3 p, dVector3 result) const
    { vectorFromWorld (p[0], p[1], p[2], result); }

  void collide2 (dGeomID g, void *data, dNearCallback *callback)
    { dSpaceCollide2 (_id,g,data,callback); }
};


class dSpace : public dGeom {

  dSpace (dSpace &);
  void operator= (dSpace &);

protected:



  dSpace () { _id = 0; }

public:
  dSpaceID id() const
    { return (dSpaceID) _id; }
  operator dSpaceID() const
    { return (dSpaceID) _id; }

  void setCleanup (int mode)
    { dSpaceSetCleanup (id(), mode); }
  int getCleanup()
    { return dSpaceGetCleanup (id()); }

  void add (dGeomID x)
    { dSpaceAdd (id(), x); }
  void remove (dGeomID x)
    { dSpaceRemove (id(), x); }
  int query (dGeomID x)
    { return dSpaceQuery (id(),x); }

  int getNumGeoms()
    { return dSpaceGetNumGeoms (id()); }
  dGeomID getGeom (int i)
    { return dSpaceGetGeom (id(),i); }

  void collide (void *data, dNearCallback *callback)
    { dSpaceCollide (id(),data,callback); }
};


class dSimpleSpace : public dSpace {

  dSimpleSpace (dSimpleSpace &);
  void operator= (dSimpleSpace &);

public:
  dSimpleSpace ()
    { _id = (dGeomID) dSimpleSpaceCreate (0); }
  dSimpleSpace (dSpace &space)
    { _id = (dGeomID) dSimpleSpaceCreate (space.id()); }
  dSimpleSpace (dSpaceID space)
    { _id = (dGeomID) dSimpleSpaceCreate (space); }
};


class dHashSpace : public dSpace {

  dHashSpace (dHashSpace &);
  void operator= (dHashSpace &);

public:
  dHashSpace ()
    { _id = (dGeomID) dHashSpaceCreate (0); }
  dHashSpace (dSpace &space)
    { _id = (dGeomID) dHashSpaceCreate (space.id()); }
  dHashSpace (dSpaceID space)
    { _id = (dGeomID) dHashSpaceCreate (space); }

  void setLevels (int minlevel, int maxlevel)
    { dHashSpaceSetLevels (id(),minlevel,maxlevel); }
};


class dQuadTreeSpace : public dSpace {

  dQuadTreeSpace (dQuadTreeSpace &);
  void operator= (dQuadTreeSpace &);

public:
  dQuadTreeSpace (const dVector3 center, const dVector3 extents, int depth)
    { _id = (dGeomID) dQuadTreeSpaceCreate (0,center,extents,depth); }
  dQuadTreeSpace (dSpace &space, const dVector3 center, const dVector3 extents, int depth)
    { _id = (dGeomID) dQuadTreeSpaceCreate (space.id(),center,extents,depth); }
  dQuadTreeSpace (dSpaceID space, const dVector3 center, const dVector3 extents, int depth)
    { _id = (dGeomID) dQuadTreeSpaceCreate (space,center,extents,depth); }
};


class dSphere : public dGeom {

  dSphere (dSphere &);
  void operator= (dSphere &);

public:
  dSphere () { }
  dSphere (dReal radius)
    { _id = dCreateSphere (0, radius); }
  dSphere (dSpace &space, dReal radius)
    { _id = dCreateSphere (space.id(), radius); }
  dSphere (dSpaceID space, dReal radius)
    { _id = dCreateSphere (space, radius); }

  void create (dSpaceID space, dReal radius) {
    if (_id) dGeomDestroy (_id);
    _id = dCreateSphere (space, radius);
  }

  void setRadius (dReal radius)
    { dGeomSphereSetRadius (_id, radius); }
  dReal getRadius() const
    { return dGeomSphereGetRadius (_id); }
};


class dBox : public dGeom {

  dBox (dBox &);
  void operator= (dBox &);

public:
  dBox () { }
  dBox (dReal lx, dReal ly, dReal lz)
    { _id = dCreateBox (0,lx,ly,lz); }
  dBox (dSpace &space, dReal lx, dReal ly, dReal lz)
    { _id = dCreateBox (space,lx,ly,lz); }
  dBox (dSpaceID space, dReal lx, dReal ly, dReal lz)
    { _id = dCreateBox (space,lx,ly,lz); }

  void create (dSpaceID space, dReal lx, dReal ly, dReal lz) {
    if (_id) dGeomDestroy (_id);
    _id = dCreateBox (space,lx,ly,lz);
  }

  void setLengths (dReal lx, dReal ly, dReal lz)
    { dGeomBoxSetLengths (_id, lx, ly, lz); }
  void getLengths (dVector3 result) const
    { dGeomBoxGetLengths (_id,result); }
};


class dPlane : public dGeom {

  dPlane (dPlane &);
  void operator= (dPlane &);

public:
  dPlane() { }
  dPlane (dReal a, dReal b, dReal c, dReal d)
    { _id = dCreatePlane (0,a,b,c,d); }
  dPlane (dSpace &space, dReal a, dReal b, dReal c, dReal d)
    { _id = dCreatePlane (space.id(),a,b,c,d); }
  dPlane (dSpaceID space, dReal a, dReal b, dReal c, dReal d)
    { _id = dCreatePlane (space,a,b,c,d); }

  void create (dSpaceID space, dReal a, dReal b, dReal c, dReal d) {
    if (_id) dGeomDestroy (_id);
    _id = dCreatePlane (space,a,b,c,d);
  }

  void setParams (dReal a, dReal b, dReal c, dReal d)
    { dGeomPlaneSetParams (_id, a, b, c, d); }
  void getParams (dVector4 result) const
    { dGeomPlaneGetParams (_id,result); }
};


class dCapsule : public dGeom {

  dCapsule (dCapsule &);
  void operator= (dCapsule &);

public:
  dCapsule() { }
  dCapsule (dReal radius, dReal length)
    { _id = dCreateCapsule (0,radius,length); }
  dCapsule (dSpace &space, dReal radius, dReal length)
    { _id = dCreateCapsule (space.id(),radius,length); }
  dCapsule (dSpaceID space, dReal radius, dReal length)
    { _id = dCreateCapsule (space,radius,length); }

  void create (dSpaceID space, dReal radius, dReal length) {
    if (_id) dGeomDestroy (_id);
    _id = dCreateCapsule (space,radius,length);
  }

  void setParams (dReal radius, dReal length)
    { dGeomCapsuleSetParams (_id, radius, length); }
  void getParams (dReal *radius, dReal *length) const
    { dGeomCapsuleGetParams (_id,radius,length); }
};


class dCylinder : public dGeom {

  dCylinder (dCylinder &);
  void operator= (dCylinder &);

public:
  dCylinder() { }
  dCylinder (dReal radius, dReal length)
    { _id = dCreateCylinder (0,radius,length); }
  dCylinder (dSpace &space, dReal radius, dReal length)
    { _id = dCreateCylinder (space.id(),radius,length); }
  dCylinder (dSpaceID space, dReal radius, dReal length)
    { _id = dCreateCylinder (space,radius,length); }

  void create (dSpaceID space, dReal radius, dReal length) {
    if (_id) dGeomDestroy (_id);
    _id = dCreateCylinder (space,radius,length);
  }

  void setParams (dReal radius, dReal length)
    { dGeomCylinderSetParams (_id, radius, length); }
  void getParams (dReal *radius, dReal *length) const
    { dGeomCylinderGetParams (_id,radius,length); }
};


class dRay : public dGeom {

  dRay (dRay &);
  void operator= (dRay &);

public:
  dRay() { }
  dRay (dReal length)
    { _id = dCreateRay (0,length); }
  dRay (dSpace &space, dReal length)
    { _id = dCreateRay (space.id(),length); }
  dRay (dSpaceID space, dReal length)
    { _id = dCreateRay (space,length); }

  void create (dSpaceID space, dReal length) {
    if (_id) dGeomDestroy (_id);
    _id = dCreateRay (space,length);
  }

  void setLength (dReal length)
    { dGeomRaySetLength (_id, length); }
  dReal getLength()
    { return dGeomRayGetLength (_id); }

  void set (dReal px, dReal py, dReal pz, dReal dx, dReal dy, dReal dz)
    { dGeomRaySet (_id, px, py, pz, dx, dy, dz); }
  void get (dVector3 start, dVector3 dir)
    { dGeomRayGet (_id, start, dir); }






  void setParams (int firstContact, int backfaceCull)
    { dGeomRaySetParams (_id, firstContact, backfaceCull); }

  void getParams (int *firstContact, int *backfaceCull)
    { dGeomRayGetParams (_id, firstContact, backfaceCull); }




  void setBackfaceCull (int backfaceCull)
    { dGeomRaySetBackfaceCull (_id, backfaceCull); }
  int getBackfaceCull()
    { return dGeomRayGetBackfaceCull (_id); }

  void setFirstContact (int firstContact)
    { dGeomRaySetFirstContact (_id, firstContact); }
  int getFirstContact()
    { return dGeomRayGetFirstContact (_id); }

  void setClosestHit (int closestHit)
    { dGeomRaySetClosestHit (_id, closestHit); }
  int getClosestHit()
    { return dGeomRayGetClosestHit (_id); }
};







class dGeomTransform : public dGeom {

  dGeomTransform (dGeomTransform &);
  void operator= (dGeomTransform &);

public:
  dGeomTransform() { }
  dGeomTransform (dSpace &space)
    { _id = dCreateGeomTransform (space.id()); }
  dGeomTransform (dSpaceID space)
    { _id = dCreateGeomTransform (space); }

  void create (dSpaceID space=0) {
    if (_id) dGeomDestroy (_id);
    _id = dCreateGeomTransform (space);
  }

  void setGeom (dGeomID geom)
    { dGeomTransformSetGeom (_id, geom); }
  dGeomID getGeom() const
    { return dGeomTransformGetGeom (_id); }

  void setCleanup (int mode)
    { dGeomTransformSetCleanup (_id,mode); }
  int getCleanup ()
    { return dGeomTransformGetCleanup (_id); }

  void setInfo (int mode)
    { dGeomTransformSetInfo (_id,mode); }
  int getInfo()
    { return dGeomTransformGetInfo (_id); }
};
