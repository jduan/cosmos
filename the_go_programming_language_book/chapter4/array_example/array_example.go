package main

import (
	"fmt"
)

// Arrays are inherently inflexible because of their fixed size.
func main() {
	var a [3]int // array of 3 integers (all initialized to 0s)
	fmt.Println(a)

	a[0] = 1
	fmt.Println(a)

	for i, v := range a {
		fmt.Printf("array a[%d] = %d\n", i, v)
	}

	b := [...]int{1, 2, 3}
	fmt.Printf("array b: %v\n", b)
	fmt.Printf("array b length: %d\n", len(b))

	r := [...]int{9: 9} // create an array of 10 integers, the 10th element is 9
	fmt.Printf("array r: %v\n", r)
}
