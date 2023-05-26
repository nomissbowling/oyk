/// bridge.cpp

#include <iomanip>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <map>

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
