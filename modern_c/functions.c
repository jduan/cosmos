#include <stdio.h>

int max(int x, int y) {
    int bigger;
    bigger = x;

    if (y > x) {
        bigger = y;
    }

    printf("  in max, before return x: %d, y: %d\n", x, y);
    return bigger;
}

int main() {
    int num1, num2;

    printf("Enter a number: ");
    scanf("%d", &num1);
    printf("Enter another: ");
    scanf("%d", &num2);

    printf("The bigger number is %d\n", max(num1, num2));
}
