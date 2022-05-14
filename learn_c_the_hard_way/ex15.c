#include <stdio.h>

void print1(int *ages, char **names, int count) {
    for (int i = 0; i < count; i++) {
        printf("%s\t has %2d years alive\n", names[i], ages[i]);
    }

    printf("-----\n");
}

void print2(int *ages, char **names, int count) {
    // using pointer arithmetic
    int *cur_age = ages;
    char **cur_name = names;

    for (int i = 0; i < count; i++) {
        printf("%s\t has %2d years alive\n", *(cur_name + i), *(cur_age + i));
    }
    printf("-----\n");
}

void print3(int *ages, char **names, int count) {
    // using indexing
    int *cur_age = ages;
    char **cur_name = names;

    for (int i = 0; i < count; i++) {
        printf("%s\t has %2d years alive\n", cur_name[i], cur_age[i]);
    }
    printf("-----\n");
}

void print4(int *ages, char **names, int count) {
    int *cur_age;
    char **cur_name;

    // keep moving pointers forward
    for (cur_name = names, cur_age = ages; (cur_age - ages) < count; cur_name++, cur_age++) {
        printf("%s\t has %2d years alive\n", *cur_name, *cur_age);
    }

    printf("-----\n");
}

void print5(int *ages, char **names, int count) {
    // iterate backwards
    int *cur_age = &ages[count - 1];
    char **cur_name = &names[count - 1];

    while (1) {
        printf("%s\t has %2d years alive\n", *cur_name, *cur_age);
        if (cur_age == ages) {
            break;
        }
        cur_name--;
        cur_age--;
    }
    printf("-----\n");
}

void print6(int *ages, char **names, int count) {
    // print the addresses of the array elements
    for (int i = 0; i < count; i++) {
        printf("%s\t is at address %p, %2d is at address %p\n",
                names[i], names[i], ages[i], ages + i);
    }
    printf("-----\n");
}

// This program shows 3 ways of using pointers
int main(int argc, char *argv[]) {
    int ages[] = {23, 43, 12, 89, 2};
    char *names[] = {
        "Alan", "Frank", "Mary", "John", "Lisa"
    };

    int count = sizeof(ages) / sizeof(int);
    print1(ages, names, count);
    print2(ages, names, count);
    print3(ages, names, count);
    print4(ages, names, count);
    print5(ages, names, count);
    print6(ages, names, count);

    return 0;
}

