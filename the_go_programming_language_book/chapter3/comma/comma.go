package main

import (
	"fmt"
)

func main() {
	fmt.Println(comma("123"))
	fmt.Println(comma("1234"))
	fmt.Println(comma("12345"))
	fmt.Println(comma("123456"))
	fmt.Println(comma("1234567"))
}

// inserts commas in a non-negative decimal integer string
// eg: "12345" -> "12,345"
func comma(s string) string {
	var s2 string
	runes := []rune(s)
	count := 0
	for i := len(runes) - 1; i >= 0; i-- {
		if count%3 == 0 && count != 0 {
			s2 = string(runes[i]) + "," + s2
		} else {
			s2 = string(runes[i]) + s2
		}
		count++
	}

	return s2
}
