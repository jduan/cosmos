#include <stdio.h>

int change_value(int *input);

int main() {
    int *ptr1, *ptr2, x, y;

    x = 8;
    ptr2 = &x;
    ptr1 = NULL;

    *ptr2 = 10;
    printf("x should be 10 at this point: %d\n", x);
    y = *ptr2 + 3;
    printf("y should be 13 at this point: %d\n", y);
    ptr1 = ptr2;
    *ptr1 = 100;
    printf("x should be 100 at this point: %d\n", x);
    ptr1 = &y;
    *ptr1 = 80;
    printf("y should be 80 at this point: %d\n", y);

    // change_value
    int x2 = 30;
    int y2 = change_value(&x2);
    printf("x: %d, y: %d\n", x2, y2);
    x2 = 130;
    y2 = change_value(&x2);
    printf("x: %d, y: %d\n", x2, y2);
}

int change_value(int *input) {
    int val = *input;
    if (val < 100) {
        *input = 100;
    } else {
        *input = val * 2;
    }

    return val;
}
