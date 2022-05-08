#include <stdio.h>

int power(int base, unsigned int exp) {
    int ret = 1;
    for (int i = 0; i < exp; i++) {
        ret *= base;
    }

    return ret;
}

int main() {
    printf("2^8 is %d\n", power(2, 8));
    printf("2^4 is %d\n", power(2, 4));
}
