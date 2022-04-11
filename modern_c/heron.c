//
// Created by Jingjing Duan on 4/4/22.
//

#include <stdlib.h>
#include <stdio.h>

static double const eps1m01 = 1.0 - 0x1P-01;
static double const eps1p01 = 1.0 + 0x1P-01;
static double const eps1m24 = 1.0 - 0x1P-24;
static double const eps1p24 = 1.0 + 0x1P-24;

int main(int argc, char* argv[argc+1]) {
    printf("%g\n", eps1m01);
    printf("%g\n", eps1p01);
    printf("%g\n", eps1m24);
    printf("%g\n", eps1p24);
}