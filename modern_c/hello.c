#include <math.h>
#include <stdio.h>

int main() {
    printf("number of bytes in a char: %lu\n", sizeof(char));
    printf("number of bytes in a short: %lu\n", sizeof(short));
    printf("number of bytes in an int: %lu\n", sizeof(int));
    printf("number of bytes in a long: %lu\n", sizeof(long));
    printf("number of bytes in a float: %lu\n", sizeof(float));
    printf("number of bytes in a double: %lu\n", sizeof(double));

    return 0;
}
