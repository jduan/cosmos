#include <stdio.h>

// swap the values stored in its two arguments
void swap(int *x, int *y) {
    int tmp = *x;
    *x = *y;
    *y = tmp;
}

int main() {
    int x, y;

    x = 10;
    y = 20;
    printf("before swap: x is %d, y is %d\n", x, y);
    swap(&x, &y);
    printf("after swap: x is %d, y is %d\n", x, y);

    return 0;
}
