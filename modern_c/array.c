#include <stdio.h>

int basic() {
    int i, size = 0;

    int my_arr[10];

    for (i = 0; i < 10; i++) {
        my_arr[i] = i;
        size++;
    }

    my_arr[3] = 100;

    printf("array of %d items: \n", size);

    for (i = 0; i < 10; i++) {
        printf("%d\n", my_arr[i]);
    }

    return 0;
}

void print_array(int arr[], int size) {
    for (int i = 0; i < size; i++) {
        printf("%d\n", arr[i]);
    }
}

void update_array(int arr[], int size) {
    if (size > 3) {
        arr[3] = 8;
    }
}

int main() {
    int a[] = {1, 2, 3, 4};
    int size = 4;

    print_array(a, size);
    update_array(a, size);
    printf("after calling update_array\n");
    print_array(a, size);
}
