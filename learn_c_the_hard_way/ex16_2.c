/*
 * This is similar to ex16.c but it creates structs on the stack instead of the
 * heap.
 */
#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct Person {
    char *name;
    int age;
    int height;
    int weight;
};

// this is like a constructor
struct Person create_person(char *name, int age, int height, int weight) {
    struct Person who;

    // Here we make a copy of the string, just to make sure that this structure
    // owns it.
    who.name = strdup(name);
    who.age = age;
    who.height = height;
    who.weight = weight;

    return who;
}

void print_person(struct Person who) {
    printf("struct Person:\n");
    printf("\tName: %s\n", who.name);
    printf("\tAge: %d\n", who.age);
    printf("\tHeight: %d\n", who.height);
    printf("\tWeight: %d\n", who.weight);
}

int main(int argc, char *argv[]) {
    struct Person joe = create_person("Joe Alex", 32, 64, 140);
    struct Person frank = create_person("Frank Blank", 20, 72, 180);

    printf("Joe is at memory location: %p\n", &joe);
    print_person(joe);

    printf("Frank is at memory location: %p\n", &frank);
    print_person(frank);

    // make everyone age 20 years and print them again
    joe.age += 20;
    joe.height -= 2;
    joe.weight += 40;
    print_person(joe);
    frank.age += 20;
    frank.weight += 20;
    print_person(frank);

    return 0;
}

// this is like a constructor
