/*
  bridge.cpp
*/

#include <iomanip>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <map>

using namespace std;

#include <bridge.hpp>

Bridge::Bridge() : str("_bridge")
{
}

Bridge::Bridge(char *s) : str(s)
{
}

Bridge::~Bridge()
{
}

void Bridge::pset(char *p)
{
  str = p;
}

void Bridge::put()
{
  cout << string(str) << endl;
}

void bput()
{
  Bridge b = Bridge("bridge");
  b.put();
}
