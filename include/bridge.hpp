/*
  bridge.hpp
*/

#ifndef __BRIDGE_H__
#define __BRIDGE_H__

class Bridge {
protected:
  char *str;
public:
  Bridge();
  Bridge(char *s);
  virtual ~Bridge();
  void pset(char *p);
  void put();
};

extern "C" {
void bput();
}

#endif // __BRIDGE_H__
