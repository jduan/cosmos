/*
 * Copyright (c) 2020, Dive into Systems, LLC
 * https://diveintosystems.org/
 */
#include <stdio.h>
#include <stdlib.h>

void foo(int *b, int c, int *arr, int n) ;
void blah(int *r, int s);

int main() {
    int x, y, *arr;

    arr = (int *) malloc(sizeof(int) * 5);
    if (arr == NULL) {
        exit(1);   // should print out nice error msg first
    }

    x = 10;
    y = 20;
    printf("x = %d y = %d\n", x, y);
    foo(&x, y, arr, 5);
    printf("x = %d y = %d arr[0] = %d arr[3] = %d\n",
            x, y, arr[0],arr[3]);
    free(arr);

    return 0;
}

void foo(int *b, int c, int *arr, int n) {
    int i;
    c = 2;
    for (i = 0; i < n; i++) {
        arr[i] = i + c;
    }
    *arr = 13;
    blah(b, c);
}

void blah(int *r, int s) {
    *r = 3;
    s = 4;
    // DRAW STACK HERE
}
