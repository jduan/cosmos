#include <stdio.h>

int main() {
    int x, y;
    x = 1;
    x = x + 2;
    x = x - 14;
    y = x * 100;
    x = x + y * 6;
    printf("x: %d, y: %d\n", x, y);

    return 0;
}
