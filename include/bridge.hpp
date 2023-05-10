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
  Bridge() : str("_bridge") {}
  /// construct with ptr
  Bridge(char *s) : str(s) {}
  /// destruct
  virtual ~Bridge() {}
  /// assign ptr
  void pset(char *p) { str = p; }
  /// display
  void put();
};

extern "C" {
/// legacy C interface
void bput();
}

#endif // __BRIDGE_H__
