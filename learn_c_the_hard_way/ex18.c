#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void die(const char *msg) {
    if (errno) {
        perror(msg);
    } else {
        printf("ERROR: %s\n", msg);
    }

    exit(1);
}

/**
 * If you have a function like:
 *
 *  int callme(int a, int b)
 *
 * You can create a "function pointer" for it like this:
 *
 *  typedef int (*pointer_name)(int a, int b)
 *
 * Then "pointer_name" can be used as the "type" for function arguments. When
 * calling such higher-order functions, you can pass in real functions that
 * comply with this "function pointer" signature.
 *
 * Pointers to functions can be used/called like the functions they point to
 * but with a different name.
 */

// typedef for function pointers
typedef int (*compare_cb)(int a, int b);
typedef int *(*sort_cb)(int *arr, int n, compare_cb cmp);

int *bubble_sort(int *numbers, int count, compare_cb cmp) {
    int tmp = 0;
    int i = 0;
    int j = 0;
    int *target = malloc(count * sizeof(int));

    if (!target) {
        die("Memory error");
    }

    memcpy(target, numbers, count * sizeof(int));

    for (i = 0; i < count; i++) {
        for (j = 0; j < count - 1; j++) {
            if (cmp(target[j], target[j + 1]) > 0) {
                tmp = target[j + 1];
                target[j + 1] = target[j];
                target[j] = tmp;
            }
        }
    }

    return target;
}

int *insertion_sort(int *arr, int n, compare_cb cmp) {
    int *target = malloc(n * sizeof(int));

    if (!target) {
        die("Memory error");
    }

    memcpy(target, arr, n * sizeof(int));

    int i, key, j;
    for (i = 1; i < n; i++) {
        key = target[i];
        j = i - 1;

        /* Move elements of target[0..i-1], that are
          greater than key, to one position ahead
          of their current position */
        while (j >= 0 && cmp(target[j], key) > 0) {
            target[j + 1] = target[j];
            j = j - 1;
        }
        target[j + 1] = key;
    }

    return target;
}

int sorted_order(int a, int b) {
    return a - b;
}

int reverse_order(int a, int b) {
    return b - a;
}

// This is a higher-order function that takes two other functions
// * sort_cb is a sorting algorithm
// * compare_cb is a comparison function
void test_sorting(sort_cb sort, int *numbers, int count, compare_cb cmp) {
    int *sorted = sort(numbers, count, cmp);
    printf("sorted: ");
    for (int i = 0; i < count; i++) {
        printf("%d ", sorted[i]);
    }
    printf("\n");

    // This is to show that C has the ability to take one kind of pointer and
    // convert it to another so you can process the data in different ways.
    // Convert the cmp function pointer to a char pointer!
    // unsigned char *data = (unsigned char *)cmp;
    // for (int i = 0; i < 25; i++) {
    //     printf("%02x:", data[i]);
    // }
    // printf("\n");

    free(sorted);
}

int main(int argc, char *argv[]) {
    if (argc < 2) die("Usage: ex18 4 3 1 5 6");

    int count = argc - 1;
    int *numbers = malloc(sizeof(int) * count);
    if (!numbers) die("Memory error");

    for (int i = 1; i < argc; i++) {
        numbers[i - 1] = atoi(argv[i]);
    }
    // pass the "sorted_order" function to the test_sorting function,
    test_sorting(bubble_sort, numbers, count, sorted_order);
    test_sorting(bubble_sort, numbers, count, reverse_order);
    test_sorting(insertion_sort, numbers, count, sorted_order);
    test_sorting(insertion_sort, numbers, count, reverse_order);

    free(numbers);

    return 0;
}

