package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Printf("basename is %s\n", basename1("a/b/c/hello.go"))
	fmt.Printf("basename is %s\n", basename1("c.d.go"))
	fmt.Printf("basename is %s\n", basename1("abc"))
	fmt.Printf("basename is %s\n", basename2("a/b/c/hello.go"))
	fmt.Printf("basename is %s\n", basename2("c.d.go"))
	fmt.Printf("basename is %s\n", basename2("abc"))
}

// Given a path like string, return the base name.
// For example, return "hello" for "a/b/c/hello.go"
func basename1(s string) string {
	for i := len(s) - 1; i >= 0; {
		if s[i] == '/' {
			s = s[i+1:]
			break
		}
		i--
	}

	for i := len(s) - 1; i >= 0; {
		if s[i] == '.' {
			s = s[:i]
			break
		}
		i--
	}

	return s
}

func basename2(s string) string {
	slash := strings.LastIndex(s, "/")
	if dot := strings.LastIndex(s, "."); dot == -1 {
		dot = len(s)
		return s[slash+1 : dot]
	} else {
		return s[slash+1 : dot]
	}
}
