// dummy for preprocess header to Rust bindgen

static int DS_VERSION = 0x0002; // as i32 #define DS_VERSION 0x0002

enum DS_TEXTURE_NUMBER {
  DS_NONE = 0,
  DS_WOOD,
  DS_CHECKERED,
  DS_GROUND,
  DS_SKY
};

typedef struct dsFunctions {
  int version;
  void (*start)();
  void (*step)(int pause);
  void (*command)(int cmd);
  void (*stop)();
  const char *path_to_textures;
} dsFunctions;

void dsDebug(const char *msg, ...);
void dsDrawBox(const float pos[3], const float R[12], const float sides[3]);
void dsDrawBoxD(const double pos[3], const double R[12], const double sides[3]);
void dsDrawCapsule(const float pos[3], const float R[12], float length, float radius);
void dsDrawCapsuleD(const double pos[3], const double R[12], float length, float radius);
void dsDrawConvex(const float pos[3], const float R[12], const float *_planes, unsigned int _planecount, const float *_points, unsigned int _pointcount, const unsigned int *_polygons);
void dsDrawConvexD(const double pos[3], const double R[12], const double *_planes, unsigned int _planecount, const double *_points, unsigned int _pointcount, const unsigned int *_polygons);
void dsDrawCylinder(const float pos[3], const float R[12], float length, float radius);
void dsDrawCylinderD(const double pos[3], const double R[12], float length, float radius);
void dsDrawLine(const float pos1[3], const float pos2[3]);
void dsDrawLineD(const double pos1[3], const double pos2[3]);
void dsDrawSphere(const float pos[3], const float R[12], float radius);
void dsDrawSphereD(const double pos[3], const double R[12], const float radius);
void dsDrawTriangle(const float pos[3], const float R[12], const float *v0, const float *v1, const float *v2, int solid);
void dsDrawTriangleD(const double pos[3], const double R[12], const double *v0, const double *v1, const double *v2, int solid);
void dsDrawTriangles(const float pos[3], const float R[12], const float *v, const int n, int solid);
void dsDrawTrianglesD(const double pos[3], const double R[12], const double *v, const int n, int solid);
double dsElapsedTime();
void dsError(const char *msg, ...);
void dsGetViewpoint(float xyz[3], float hpr[3]);
void dsPrint(const char *msg, ...);
void dsSetCapsuleQuality(int n);
void dsSetColor(float red, float green, float blue);
void dsSetColorAlpha(float red, float green, float blue, float alpha);
void dsSetDrawMode(int mode);
void dsSetSphereQuality(int n);
void dsSetTexture(int texture_number);
void dsSetViewpoint(float xyz[3], float hpr[3]);
void dsSimulationLoop(int argc, char **argv, int window_width, int window_height, struct dsFunctions *fn);
void dsStop();
