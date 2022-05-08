/* Copyright (c) 2020, Dive into Systems, LLC (https://diveintosystems.org/)
 */

#include <stdio.h>

/* function prototypes: */
void printArray(int a[], int size);
int minimum(int a[], int size);

int main() {
    int N = 10;
    // this is a way to statically initialize an array
    // (something that is only occasionally useful):
    int data[] = {5, 8, 9, 1, 10, 12, 4, 3, 7, 13};
    int opposite[N];
    int min, i;
    printf("opposite size: %lu\n", sizeof(opposite));

    for(i = 0; i < N; i++) {
        opposite[i] = -(data[i]);
    }
    printArray(data, N);
    min = minimum(data, N);
    printf("Smallest value in data is: %d\n", min);

    printArray(opposite, N);
    min = minimum(opposite, N);
    printf("Smallest value in opposite is: %d\n", min);

    return 0;
}

/* prints out the contents of an array
 *  a: the array of int values
 *  size: the number of elements in the array
 */
void printArray(int a[], int size) {
    // An example of a function that doesn't return a value.
    int i;

    printf("Array Contents:\n");
    for (i = 0; i < size; i++) {
        printf("%d ", a[i]);
    }
    printf("\n");
}

/* finds the smallest element in the passed array
 *  a: the array of int values
 *  size: the number of elements in the array
 *  returns: the smallest value in the array
 */
int minimum(int a[], int size) {
    int low = a[0];
    for (int i = 1; i < size; i++) {
        if (a[i] < low) {
            low = a[i];
        }
    }

    return low;
}
