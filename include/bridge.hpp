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
typedef double dReal;
/// res = a(&dMatrix3) b(&dVector3)
void dMULTIPLY0_331(dReal *res, const dReal *a, const dReal *b);
/// res = a(&dMatrix3) b(&dMatrix3)
void dMULTIPLY0_333(dReal *res, const dReal *a, const dReal *b);
}

#endif // __BRIDGE_H__
