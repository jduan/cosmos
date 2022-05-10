#include <stdio.h>
#include "mylib.h"

int main() {
    float val1, val2, ret;

    printf("Enter two float numbers: ");
    scanf("%f%f" , &val1, &val2);
    ret = bigger(val1, val2);
    printf("%f is the bigger one\n", ret);

    return 0;
}
