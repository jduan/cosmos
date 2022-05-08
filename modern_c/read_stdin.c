#include <stdio.h>

int main() {
    int num1, num2;

    printf("Enter a number: ");
    scanf("%d", &num1);
    printf("Enter another: ");
    scanf("%d", &num2);

    if (num1 > num2) {
        printf("%d is biggest\n", num1);
    } else {
        printf("%d is biggest\n", num2);
    }

    return 0;
}
