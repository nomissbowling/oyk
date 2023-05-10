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
