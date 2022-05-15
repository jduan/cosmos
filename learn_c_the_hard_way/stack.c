#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Element {
    char *value;
    struct Element *next;
};

struct Stack {
    struct Element *head;
};

void die(char *msg) {
    printf("%s\n", msg);
    exit(1);
}

struct Stack *Stack_create() {
    struct Stack *s = malloc(sizeof(struct Stack));
    if (!s) {
        die("Malloc error");
    }
    s->head = NULL;

    return s;
}

void Stack_push(struct Stack *s, char *value) {
    struct Element *element = malloc(sizeof(struct Element));
    if (!element) {
        die("Failed to malloc an element");
    }

    char *copy = malloc(strlen(value) + 1);
    if (!copy) {
        die("Failed to malloc a string");
    }

    strcpy(copy, value);

    element->value = copy;
    element->next = s->head;
    s->head = element;
}

void Stack_pop(struct Stack *s) {
    if (s->head == NULL) {
        die("Can't pop an empty stack");
    }

    char *value = s->head->value;
    printf("Popped %s\n", value);

    struct Element *orig_head = s->head;
    s->head = s->head->next;

    // free stuff
    free(orig_head->value);
    free(orig_head);
}

int Stack_is_empty(struct Stack *s) {
    if (s->head == NULL) {
        printf("stack is empty\n");
        return 1;
    } else {
        printf("stack is not empty\n");
        return 0;
    }
}

char *Stack_peek(struct Stack *s) {
    if (Stack_is_empty(s)) {
        die("Can't peak an empty stack");
    }

    return s->head->value;
}

int main(int argc, char *argv[]) {
    struct Stack *s = Stack_create();
    Stack_is_empty(s);
    Stack_push(s, "hello");
    printf("Peaking stack: %s\n", Stack_peek(s));
    Stack_push(s, "world");
    printf("Peaking stack: %s\n", Stack_peek(s));
    Stack_is_empty(s);
    Stack_pop(s);
    Stack_pop(s);
    Stack_is_empty(s);

    return 0;
}

