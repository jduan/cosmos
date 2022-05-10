#include <stdio.h>

// Include the library header file if the implementation needs
// any of its definitions (types or constants, for example.)
#include "mylib.h"

int total_times = 0;

float bigger(float x, float y) {
    total_times++;
    if (x > y) {
        return x;
    } else {
        return y;
    }
}
