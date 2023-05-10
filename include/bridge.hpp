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

#endif // __BRIDGE_H__
