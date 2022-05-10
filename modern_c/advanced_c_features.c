#include <stdio.h>
#include <stdlib.h>

void switch_example() {
    int num, new_num = 0;

    printf("Enter a number between 6 and 9: ");
    scanf("%d", &num);

    switch(num) {
        case 6:
            new_num = num + 1;
            break;
        case 7:
            new_num = num;
            break;
        case 8:
            new_num = num - 1;
            break;
        case 9:
            new_num = num + 2;
            break;
        default:
            printf("Hey, %d is not between 6 and 9\n", num);
    }

    printf("num %d  new_num %d\n", num, new_num);
}

/**
 * Because "void *" is a generic pointer type, it cannot be directly
 * dereferenced — the compiler does not know the size of memory that the address
 * points to. For example, the address could refer to an int storage location of
 * four bytes or it could refer to a char storage location in memory of one
 * byte. Therefore, the programmer must explicitly recast the void * pointer to
 * a pointer of a specific type before dereferencing it. Recasting tells the
 * compiler the specific type of pointer variable, allowing the compiler to
 * generate the correct memory access code for pointer dereferences.
 */
void void_pointers() {
    int *array;
    char *str;

    array = (int *) malloc(sizeof(int) * 10);
    str = (char *) malloc(sizeof(char) * 20);

    *array = 10;
    str[0] = 'a';
}

/*
 * When incremented, a pointer points to the next storage location of the type
 * it points to. For example, incrementing an integer pointer (int *) makes it
 * point to the next int storage address (the address four bytes beyond its
 * current value), and incrementing a character pointer makes it point to the
 * next char storage address (the address one byte beyond its current value).
 */
#define N 10
#define M 20
void pointer_arithmetic() {
    char letters[N];
    int numbers[N], i, j;

    char *cptr = NULL;
    int *iptr = NULL;

    // cptr = letters;
    cptr = &(letters[0]);
    iptr = numbers;

    for (i = 0; i < N; i++) {
        *cptr = 'a' + i;
        *iptr = i * 3;

        // advance the pointers by "1"
        cptr++;
        iptr++;
    }

    printf("array values using indexing to access\n");
    for (i = 0; i < N; i++) {
        printf("letter[%d] = %c, numbers[%d] = %d\n", i, letters[i], i, numbers[i]);
    }

    printf("array values using pointer arithmetic to access\n");
    cptr = letters;
    iptr = numbers;
    for (i = 0; i < N; i++) {
        printf("letter[%d] = %c, numbers[%d] = %d\n", i, *cptr, i, *iptr);
        cptr++;
        iptr++;
    }

    // use pointer arithmetic to initialize a 2D array
    int matrix[N][M];
    iptr = &(matrix[0][0]);
    for (i = 0; i < N * M; i++) {
        *iptr = i;
        iptr++;
    }

    printf("2D array values initialized using pointer arithmetic\n");
    for (i = 0; i < N; i++) {
        for (j = 0; j < M; j++) {
            printf("%d ", matrix[i][j]);
        }
        printf("\n");
    }
}

int main() {
    // switch_example();
    pointer_arithmetic();

    return 0;
}
