#include <stdio.h>

void my_strcpy(char dest[], char src[]) {
    int i;
    for (i = 0; src[i] != '\0'; i++) {
        dest[i] = src[i];
    }

    dest[i] = '\0';
}

int main() {
    char dest[10];
    my_strcpy(dest, "hello\0");
    printf("dest: %s\n", dest);
}
