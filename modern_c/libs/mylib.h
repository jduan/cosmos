#ifndef _MYLIB_H
#define _MYLIB_H

// a constant exported by this library
#define MAX_FOO 20

struct foo_struct {
    int x;
    float y;
};

// a global variable exported by this library
extern int total_times;

extern float bigger(float x, float y);

#endif
