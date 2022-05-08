#include <stdio.h>
#include <stdlib.h>

// initialize a dynamic allocated array of ints
void init_array(int *arr, int size) {
    for (int i = 0; i < size; i++) {
        arr[i] = i;
    }
}

int main() {
    int *p;

    p = malloc(sizeof(int));

    if (p != NULL) {
        *p = 6;
    }

    printf("the location pointed by p has value: %d\n", *p);

    free(p);
    p = NULL;

    int *arr;
    char *c_arr;

    // allocate an array of 20 ints on the heap
    arr = malloc(sizeof(int) * 20);
    if (arr == NULL) {
        printf("Error: malloc failed\n");
        exit(1);
    }
    init_array(arr, 20);

    printf("the 4th element is: %d\n", arr[3]);

    // allocate an array of 10 ints on the heap
    c_arr = malloc(sizeof(char) * 10);
    if (c_arr == NULL) {
        printf("Error: malloc failed\n");
        exit(1);
    }
}
