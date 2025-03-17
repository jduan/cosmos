package main

import (
	"fmt"
)

func main() {
	// numbers
	fmt.Println("1 + 1 =", 1+1)
	fmt.Println("1.0 + 1.0 =", 1.0+1.0)
	fmt.Println("32132 * 42452=", 32_132*42_452)

	// strings
	fmt.Println(len("Hello, world"))
	fmt.Println("Hello, world"[1])
	fmt.Println("Hello, " + "world")

	// booleans
	b := (true && false) || (false && true) || !(false && true)
	fmt.Println(b)
}
