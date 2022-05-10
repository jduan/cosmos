/*
 * Copyright (c) 2020, Dive into Systems, LLC
 * https://diveintosystems.org/
 *
 * An example of C pass-by-pointer.
 */
#include <stdio.h>

int arg_modifier(int x, int *y);

int main() {
    int val1, val2, ret;

    printf("Enter an interger: ");
    scanf("%d", &val1);
    printf("Enter anothern interger: ");
    scanf("%d", &val2);

    // pass val1 by value and val2 by pointer:
    printf("before call: val1 = %d val2 = %d\n", val1, val2);
    ret = arg_modifier(val1, &val2);
    printf("after call:  val1 = %d val2 = %d ret = %d\n", val1, val2, ret);

    /*
     * uncomment to test passing val2 by value and val1 by pointer
     */
    printf("before 2nd call: val1 = %d val2 = %d\n", val1, val2);
    ret = arg_modifier(val2, &val1);
    printf("after 2nd call: val1 = %d val2 = %d\n", val1, val2);

    return 0;
}

/*
 * This function illustrates C-style pass by pointer semantics, the
 * argument pointed to by the second parameter, y, will be modified
 * after the call, the value of the first argument, x, will not.
 */
int arg_modifier(int x, int *y) {

    printf("  in arg_modifier:      x = %d *y = %d\n", x, *y);

    /* the location y points to gets (value of what y points to + x) */
    *y = *y + x;

    x = x + 5;

    printf("  leaving arg_modifier: x = %d *y = %d\n", x, *y);

    return x;
}
