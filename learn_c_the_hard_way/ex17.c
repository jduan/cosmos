#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_DATA 512
#define MAX_ROWS 100

// This struct is meant to show "struct packing"
struct Foo {
    char *p; // pointers are 8 bytes
    char c;  // will be padded to 8 bytes
    long x;  // longs are 8 bytes
};

// name and email don't need to be pointers because the whole struct can be
// malloc'd on the heap.
struct Address {
    int id;
    int set;
    char name[MAX_DATA];
    char email[MAX_DATA];
};

// just a wrapper of an array of addresses
struct Database {
    struct Address rows[MAX_ROWS];
};

struct Connection {
    FILE *file;
    struct Database *db;
};

// This is just to demonstrate global variables. You should almost never use
// global variables.
struct Connection *conn;

void die(const char *message) {
    if (errno) {
        perror(message);
    } else {
        printf("ERROR: %s\n", message);
    }

    exit(1);
}

void Address_print(struct Address *addr) {
    printf("Address: %d %s %s\n", addr->id, addr->name, addr->email);
}

// Read the database, all the rows
void Database_load(struct Connection *conn) {
    // You can read the whole Database struct from the file
    int rc = fread(conn->db, sizeof(struct Database), 1, conn->file);
    if (rc != 1) {
        die("Failed to load database.");
    }
}

struct Connection *Database_open(const char *filename, char mode) {
    struct Connection *conn = malloc(sizeof(struct Connection));
    if (!conn) {
        die("Memory error");
    }
    conn->db = malloc(sizeof(struct Database));
    if (!conn->db) {
        die("Memory error");
    }

    if (mode == 'c') {
        // create for the first time
        conn->file = fopen(filename, "w");
    } else {
        // load the database from disk
        conn->file = fopen(filename, "r+");
        if (conn->file) {
            Database_load(conn);
        }
    }
    if (!conn->file) {
        die("Failed to open the file");
    }

    return conn;
}

void Database_close() {
    if (conn) {
        if (conn->file) {
            fclose(conn->file);
        }
        if (conn->db) {
            free(conn->db);
        }
        free(conn);
    }
}

void Database_write() {
    rewind(conn->file);
    // you can write the entire struct to disk
    int rc = fwrite(conn->db, sizeof(struct Database), 1, conn->file);
    if (rc != 1) {
        die("Failed to save the database to file");
    }
    rc = fflush(conn->file);
    if (rc != 0) {
        die("Failed to flush the database file to disk");
    }
}

// this only initializes the database struct
void Database_create() {
    for (int i = 0; i < MAX_ROWS; i++) {
        // if you don't initialize some fields, they default to being empty
        struct Address addr = {.id = i, .set = 0};
        conn->db->rows[i] = addr;
    }
}

void Database_set(int id, const char *name,
        const char *email) {
    struct Address *addr = &conn->db->rows[id];
    if (addr->set) {
        die("Already set, delete it first");
    }

    addr->set = 1;

    // Always set the last character to '\0' because strncpy may not do it
    char *res = strncpy(addr->name, name, MAX_DATA - 1);
    addr->name[MAX_DATA - 1] = '\0';
    if (!res) {
        die("Name copy failed");
    }

    res = strncpy(addr->email, email, MAX_DATA - 1);
    addr->name[MAX_DATA - 1] = '\0';
    if (!res) {
        die("Email copy failed");
    }
}

void Database_get(int id) {
    struct Address *addr = &conn->db->rows[id];
    if (addr->set) {
        Address_print(addr);
    } else {
        die("ID is not set");
    }
}

void Database_find(char *name) {
    for (int i = 0; i < MAX_ROWS; i++) {
        struct Address addr = conn->db->rows[i];
        if (strcmp(name, addr.name) == 0) {
            printf("Found %s, id is %d\n", name, addr.id);
            return;
        }
    }
    printf("Failed to find an address with name %s\n", name);
}

void Database_delete(int id) {
    struct Address *addr = &conn->db->rows[id];
    if (addr->set) {
        addr->set = 0;
        memset(addr->name, 0, MAX_DATA);
        memset(addr->email, 0, MAX_DATA);
    }
}

void Database_list() {
    for (int i = 0; i < MAX_ROWS; i++) {
        struct Address *addr = &conn->db->rows[i];
        if (addr->set) {
            Address_print(addr);
        }
    }
}

int main(int argc, char *argv[]) {
    printf("The size of struct Foo is %lu\n", sizeof(struct Foo));
    printf("The size of struct Address is %lu\n", sizeof(struct Address));
    printf("The size of pointers is %lu\n", sizeof(char *));
    if (argc < 3) {
        die("Usage: ex17 dbfile action [action params]");
    }

    char *filename = argv[1];
    char action = argv[2][0];
    conn = Database_open(filename, action);
    int id;
    if (argc > 3) {
        id = atoi(argv[3]);
    }

    switch (action) {
        case 'c': // create
            Database_create();
            Database_write();
            break;
        case 'g': // get
            if (argc != 4) {
                die("Need an id to get");
            }
            Database_get(id);
            break;
        case 'f': // find by name
            if (argc != 4) {
                die("Need an name to find");
            }
            Database_find(argv[3]);
            break;
        case 's': // set
            if (argc != 6) {
                die("Need id, name, email to set");
            }
            Database_set(id, argv[4], argv[5]);
            Database_write();
            break;
        case 'd': // delete
            if (argc != 4) {
                die("Need id to delete");
            }
            Database_delete(id);
            Database_write();
            break;
        case 'l': // list
            Database_list();
            break;
        default:
            die("Invalid action: c = create, g = get, s = set, d = delete, l = list");

    }

    Database_close();

    return 0;
}

