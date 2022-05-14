#include <stdio.h>

int main(int argc, char *argv[]) {
    int i = 0;
    int num_args = argc - 1;

    if (num_args == 0) {
        printf("You have no arguments. You suck.\n");
    } else if (num_args == 1) {
        printf("You only have one argument. You suck.\n");
    } else if (num_args > 1 && num_args < 4) {
        printf("Here's your arguments:\n");
        for (i = 1; i < argc; i++) {
            printf("%s ", argv[i]);
        }
        printf("\n");
    } else {
        printf("You have too many arguments. You suck.\n");
    }

    return 0;
}
