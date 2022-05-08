#include <stdio.h>
#include <stdlib.h>

#define COLS (100)
#define N 3
#define M 4

/** For multidimensional array parameters, you must indicate that the parameter
 * is a multidimensional array, but you can leave the size of the first
 * dimension unspecified (for good generic design). The sizes of other
 * dimensions must be fully specified so that the compiler can generate the
 * correct offsets into the array.
 */

void init_matrix(int m[][COLS], int rows) {
    int i, j;
    for (i = 0; i < rows; i++) {
        for (j = 0; j < COLS; j++) {
            m[i][j] = i * j;
        }
    }
}

// method 1: memory-efficient allocation
// In this method, a single call to "malloc" allocates the total number of bytes
// needed to store the N * M array of values.
void method1() {
    int *two_d_array;

    two_d_array = malloc(sizeof(int) * N * M);

    if (two_d_array == NULL) {
        printf("ERROR: malloc failed\n");
        exit(1);
    }

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            /* the C compiler does not know the difference between a 2D or 1D */
            /* array allocation using this method. As a result, the double */
            /* indexing syntax ([i][j]) of statically declared 2D arrays cannot */
            /* be used when allocating a 2D array using this method. Instead, the */
            /* programmer must explicitly compute the offset into the contiguous */
            /* chunk of heap memory using a function of row and column index */
            /* values ([i*M + j], where M is the column dimension). */
            two_d_array[i * M + j] = i * j;
        }
    }

    printf("element: %d\n", two_d_array[2 * M + 3]);
}

/* method 2
 * The second method for dynamically allocating a 2D array stores the array as
 * an array of N 1D arrays (one 1D array per row). It requires N+1 calls to
 * malloc: one malloc for the array of row arrays, and one malloc for each of
 * the N row’s column arrays. As a result, the element locations within a row
 * are contiguous, but elements are not contiguous across rows of the 2D array.
 */
void method2() {
    int **two_d_array;
    two_d_array = malloc(sizeof(int *) * N);
    // error checking is not done here
    for (int i = 0; i < N; i++) {
        two_d_array[i] = malloc(sizeof(int) * M);
    }

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            // you can use "double-indexing" syntax
            two_d_array[i][j] = i * j;
        }
    }

    printf("element: %d\n", two_d_array[2][3]);
}


int main() {
    int matrix[50][COLS];
    int bigger[90][COLS];

    init_matrix(matrix, 50);
    init_matrix(bigger, 90);

    printf("%d\n", matrix[2][3]);

    method1();
    method2();
}
