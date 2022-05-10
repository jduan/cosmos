#include <stdio.h>

int myfunc(int param);

int main() {
    int ret = myfunc(32);
    printf("myfunc(32) is %d\n", ret);

    return 0;
}
