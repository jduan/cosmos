#include <stdio.h>

int max(int n1, int n2);
int change(int amt);

int g_x; // global variables can be accessed anywhere

int main() {
    int x, result;

    printf("Enter a value: ");
    scanf("%d", &x);
    g_x = 10;

    result = max(g_x, x);
    printf("%d is the largest of %d and %d\n", result, g_x, x);

    result = change(10);
    printf("g_x value was %d and now is %d\n", result, g_x);

    return 0;
}

int max(int n1, int n2) {
    if (n1 > n2) {
        return n1;
    } else {
        return n2;
    }
}

int change(int amt) {
    int val = g_x;
    g_x += amt;
    return val;
}
