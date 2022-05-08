#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void crash() {
    // Attempt to write a 12-byte string into a 5-character array.
    char mystr[5];
    strcpy(mystr, "hello world");
    printf("copied to a char array");

    // Attempt to write to a string with a NULL destination.
    char *mystr2 = NULL;
    strcpy(mystr2, "try again");
    printf("copied to a char pointer");

    // Attempt to modify a read-only string literal.
    char *mystr3 = "string literal value";
    strcat(mystr3, "string literals aren't writable");
    printf("copied to a string literal");
}

void dym_alloc() {
    int size;
    char str[64]; // statically allocated
    char *new_str = NULL; // dynamically allocated

    strcpy(str, "hello");
    printf("str: %s\n", str);
    size = strlen(str); // 5

    // need extra space for '\0'
    new_str = malloc(sizeof(char) * (size + 1));
    if (new_str == NULL) {
        printf("ERROR: malloc failed");
        exit(1);
    }

    strcpy(new_str, str);
    printf("%s %s\n", str, new_str);

    strcat(str, " world");
    printf("%s\n", str);

    free(new_str);
    new_str = NULL;
}

void basic() {
    char str1[10];
    char str2[10];
    int len;

    str1[0] = 'h';
    str1[1] = 'i';
    str1[2] = '\0';

    len = strlen(str1);
    printf("%s %d\n", str1, len);

    strcpy(str2, str1);
    printf("%s\n", str2);

    strcpy(str2, "hello");
    len = strlen(str2);
    printf("%s has %d chars\n", str2, len);
}

void lib_example() {
    char str[32];
    char *d_str, *ptr;

    // copy strings
    strcpy(str, "Hello There");
    printf("len: %lu\n", strlen(str));

    d_str = malloc(sizeof(char) * (strlen(str) + 1));
    if (d_str == NULL) {
        printf("ERROR: malloc failed");
        exit(1);
    }

    strncpy(d_str, str, 5);
    d_str[5] = '\0'; // explicitly add null to the end

    printf("%lu:%s\n", strlen(str), str);
    printf("%lu:%s\n", strlen(d_str), d_str);

    // compare strings
    strcpy(str, "alligator");
    strcpy(d_str, "Zebra");
    // int ret = strcmp(str, d_str);
    int ret = strncmp(str, d_str, 3);
    if (ret == 0) {
        printf("%s is equal to %s\n", str, d_str);
    } else if (ret < 0) {
        printf("%s is less than %s\n", str, d_str);
    } else {
        printf("%s is greater than %s\n", str, d_str);
    }

    // concat, find substring
    strcpy(str, "Zebra fish");
    strcat(str, " stripes");
    printf("%s\n", str);
    strncat(str, " are black.", 8);
    printf("%s\n", str);
    printf("str len: %lu\n", strlen(str));

    ptr = strstr(str, "trip");
    if (ptr != NULL) {
        printf("%s\n", ptr);
    }
    ptr = strchr(str, 'e');
    if (ptr != NULL) {
        printf("%s\n", ptr);
    }

    free(d_str);
    d_str = NULL;
}

// tokenize a string
int tokens() {
    char *whitespace = " \t\f\r\v\n";
    char *token;
    char *line;

    line = malloc(200 * sizeof(char));
    if (line == NULL) {
        printf("Error: malloc failed");
        exit(1);
    }

    printf("Enter a line of text:\n");
    line = fgets(line, 200 * sizeof(char), stdin);
    if (line == NULL) {
        printf("ERROR: reading input failed, exiting");
        exit(1);
    }
    printf("The input line is:\n%s\n", line);

    token = strtok(line, whitespace);
    while (token != NULL) {
        printf("Next token is %s\n", token);
        token = strtok(NULL, whitespace);
    }

    free(line);
    return 0;
}

void sprintf_example() {
    char str[64];
    float ave = 76.8;
    int num = 2;

    sprintf(str, "%s is %d years old and in grade %d", "Henry", 12, 7);
    printf("%s\n", str);

    sprintf(str, "The average grade on exam %d is %g", num, ave);
    printf("%s\n", str);
}

/*
 * int islower(ch);
 * int isupper(ch);       // these functions return a non-zero value if the
 * int isalpha(ch);       // test is TRUE, otherwise they return 0 (FALSE)
 * int isdigit(ch);
 * int isalnum(ch);
 * int ispunct(ch);
 * int isspace(ch);
 * char tolower(ch);     // returns ASCII value of lower-case of argument
 * char toupper(ch);
 */
void char_example() {
    char str[64];
    int len, i;

    strcpy(str, "I see 20 ZEBRAS, GOATS, and COWS");

    if (islower(str[2])) {
        printf("%c is lower case\n", str[2]);
    }

    len = strlen(str);
    for (i = 0; i < len; i++) {
        if (isupper(str[i])) {
            str[i] = tolower(str[i]);
        } else if (isdigit(str[i])) {
            str[i] = 'X';
        }
    }
    printf("%s\n", str);
}

int main() {
    // dym_alloc();
    // crash();
    // lib_example();
    // tokens();
    // sprintf_example();
    char_example();

    return 0;
}
