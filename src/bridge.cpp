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
