#include <stdio.h>

int main(int argc, char *argv[]) {
    int i = 0;
    while (i < 25) {
        printf("%d ", i);
        i++;
    }
    printf("\n");

    int n = 25;
    while (n > 0) {
        printf("%d ", n);
        n--;
    }

    return 0;
}
