#include <stdio.h>
#include <stdlib.h>

/** standard and file I/O functions in stdio.h
 *
 * 1. character based
 * int fgetc(FILE *f);
 * int fputc(int c, FILE *f);
 * int ungetc(int c, FILE *f);
 * int getchar(); // from stdin
 * int putchar(int c); // to stdout
 *
 * 2. string based
 * char *fgets(char *s, int n, FILE *f);
 * int fputs(char *s, FILE *f);
 *
 * 3. format based
 * int fprintf(FILE *f, char *format, ...);
 * int printf(char *format, ...);
 * int fscanf(FILE *f, char *format, ...);
 * int scanf(char *format, ...);
 *
 */

void print_example() {
    float x, y;
    char ch;

    x = 4.50001;
    y = 5.199999;
    ch = 'a';

    // single precision
    printf("%.1f %.1f\n", x, y);

    printf("%6.1f \t %6.1f \t %c\n", x, y, ch);

    printf("%6.1f \t %6.1f \t %c\n", x + 1, y + 1, ch + 1);

    printf("%6.1f \t %6.1f \t %c\n", x * 20, y * 20, ch + 2);

    int z = 26;
    ch = 'A';
    printf("z is %d in decimal, %x in hexadecimal, and %o in octal\n", z, z, z);
    printf("ch value is %d which is the ASCII value of %c\n", ch, ch);
}

void file_example() {
    FILE *infile;
    FILE *outfile;

    infile = fopen("/tmp/1", "r");
    if (infile == NULL) {
        printf("Error: unable to open infile %s\n", "/tmp/1");
        exit(1);
    }

    outfile = fopen("/tmp/1.bak", "w");
    if (outfile == NULL) {
        printf("Error: unable to open outfile %s\n", "/tmp/1.bak");
        exit(1);
    }

    int ch;
    ch = getc(infile);
    while (ch != EOF) {
        putc(ch, outfile);
        ch = getc(infile);
    }

    fclose(infile);
    fclose(outfile);
}

int main() {
    print_example();
    file_example();
}
