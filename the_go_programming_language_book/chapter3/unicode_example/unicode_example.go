package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	s := "Hello, 世界"
	fmt.Println(len(s))                    // output: 13
	fmt.Println(utf8.RuneCountInString(s)) // output: 9
	print_unicodes(s)
	print_unicodes2(s)
}

func print_unicodes(s string) {
	fmt.Println("print_unicodes")
	for i := 0; i < len(s); {
		r, size := utf8.DecodeRuneInString(s[i:])
		fmt.Printf("%d\t%c\n", i, r)
		i += size
	}
}

func print_unicodes2(s string) {
	fmt.Println("print_unicode2")
	for i, r := range s {
		fmt.Printf("%d\t%c\n", i, r)
	}
}
