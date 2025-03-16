package main

import (
	"fmt"
	"maps"
)

func main() {
	m := make(map[string]int)
	m["k1"] = 7
	m["k2"] = 13
	fmt.Println("map:", m)

	v1 := m["k1"]
	fmt.Println("v1:", v1)
	// if a key doesn't exist, return the "zero value" of the value type
	v3 := m["k3"]
	fmt.Println("v3:", v3)

	fmt.Println("len:", len(m))

	delete(m, "k2")
	fmt.Println("map:", m)

	// it's ok to delete a key that doesn't exist
	delete(m, "k5")
	fmt.Println("map:", m)

	// remove all key/value pairs
	clear(m)
	fmt.Println("map:", m)

	_, prs := m["k2"]
	fmt.Println("prs:", prs)

	// map literal
	n := map[string]int{
		"foo": 1,
		"bar": 2,
	}
	fmt.Println("map:", n)

	n2 := map[string]int{
		"foo": 1,
		"bar": 2,
	}
	if maps.Equal(n, n2) {
		fmt.Println("n == n2")
	}
}
