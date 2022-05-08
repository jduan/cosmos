#include <string.h>
#include <stdio.h>
#include <stdlib.h>

struct studentT {
    char name[64];
    int age;
    float gpa;
    int grad_yr;
};

void print_student(struct studentT *p) {
    printf("student(name: %s, age: %d, gpa: %g, grad_yr: %d)\n",
            p->name, p->age, p->gpa, p->grad_yr);
}

// a copy of struct studentT is passed when this function is called
int checkID(struct studentT s, int min_age) {
    int ret = 1;

    if (s.age < min_age) {
        ret = 0;

        // let's try changing the student's age
        s.age = min_age + 1;
    }

    printf("inside checkID: %s is %d years old\n", s.name, s.age);

    return ret;
}

void change_name(char *old, char *new) {
    if ((old == NULL) || (new == NULL)) {
        return;
    }

    strcpy(old, new);
}

int basic() {
    struct studentT student1, student2;

    strcpy(student1.name, "Kwame Salter");
    student1.age = 18 + 2;
    student1.gpa = 3.5;
    student1.grad_yr = 2020;

    printf("number of bytes in student struct: %lu\n", sizeof(struct studentT));
    printf("number of bytes in student1: %lu\n", sizeof(student1));

    printf("name: %s, age: %d, gpa: %g, year: %d\n",
            student1.name,
            student1.age,
            student1.gpa,
            student1.grad_yr);

    student2 = student1; // student1's field values are copied to field values of student2

    strcpy(student2.name, "Frances Allen");
    student2.grad_yr = student1.grad_yr + 1;

    printf("name: %s, age: %d, gpa: %g, year: %d\n",
            student2.name,
            student2.age,
            student2.gpa,
            student2.grad_yr);

    int can_vote = checkID(student1, 28);
    if (can_vote) {
        printf("%s is %d years old and can vote.\n",
                student1.name, student1.age);
    } else {
        printf("%s is %d years old and can't vote.\n",
                student1.name, student1.age);
    }

    change_name(student2.name, "Kwame");
    printf("student 2's name is now %s\n", student2.name);

    return 0;
}

void pointers() {
    struct studentT *sptr;
    sptr = malloc(sizeof(struct studentT));
    if (sptr == NULL) {
        printf("Error: malloc failed");
        exit(1);
    }

    // equivalent to: (*sptr).gpa = 3.5
    sptr->gpa = 3.5;
    sptr->grad_yr = 2021;

    print_student(sptr);
    free(sptr);
}

struct personT {
    char *name;
    int age;
};

void print_person(struct personT *p) {
    printf("person(name: %s, age: %d)\n", p->name, p->age);
}

void pointer_fields() {
    struct personT p1, *p2;

    p1.name = malloc(sizeof(char) * 8);
    strcpy(p1.name, "Zhichen");
    p1.age = 22;
    print_person(&p1);

    // first malloc space for the struct
    p2 = malloc(sizeof(struct personT));
    // then malloc space for the name field
    p2->name = malloc(sizeof(char) * 4);
    strcpy(p2->name, "Vic");
    p2->age = 19;
    print_person(p2);

    // free the space
    free(p1.name);
    free(p2->name);
    free(p2);
}

struct studentT *create_student(struct studentT *s,
        char *name, int age, float gpa, int grad_yr) {
    strcpy(s->name, name);
    s->age = age;
    s->gpa = gpa;
    s->grad_yr = grad_yr;

    return s;
}

void update_ages(struct studentT *classroom, int size) {
    for (int i = 0; i < size; i++) {
        classroom[i].age += 1;
    }
}

void array_of_structs() {
    // an array of 40 students on the stack
    struct studentT classroom1[40];
    // for dynamically allocated array in the heap
    struct studentT *classroom2;
    // an array of 40 students (each element stores a student pointer)
    struct studentT *classroom3[40];

    classroom2 = malloc(sizeof(struct studentT) * 3);
    if (classroom2 == NULL) {
        printf("malloc failed");
        exit(1);
    }
    create_student(&classroom2[0], "John", 20, 3.5, 2020);
    create_student(&classroom2[1], "Dave", 20, 3.5, 2020);
    create_student(&classroom2[2], "Smith", 20, 3.5, 2020);

    update_ages(classroom2, 3);
    for (int i = 0; i < 3; i++) {
        print_student(&classroom2[i]);
    }

    // free the space
    free(classroom2);
}

// self-referential structs
struct node {
    int data;
    struct node *next;
};

struct node *create_list() {
    struct node *head, *temp;
    int i;

    head = NULL;

    head = malloc(sizeof(struct node));
    if (head == NULL) {
        printf("Error malloc");
        exit(1);
    }

    head->data = 10;
    head->next = NULL;

    for (i = 0; i < 2; i++) {
        temp = malloc(sizeof(struct node));
        if (temp == NULL) {
            printf("Error malloc");
            exit(1);
        }

        temp->data = i;
        temp->next = head;
        head = temp;
    }

    return head;
}

void print_list(struct node *head) {
    printf("list: ");
    while (head != NULL) {
        printf("%d ", head->data);
        head = head->next;
    }
}

int main() {
    // basic();
    pointers();
    // pointer_fields();
    // array_of_structs();
    struct node *head = create_list();
    print_list(head);
}
