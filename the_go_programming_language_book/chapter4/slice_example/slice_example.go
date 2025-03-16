package main

import (
	"fmt"
	"slices"
)

func main() {
	// month_example()
	// reverse_example()
	// append_example()
	// non_empty_example()
	// remove_adjacent_dups_example()
	full_example()
}

func non_empty_example() {
	data := []string{"one", "", "three"}
	fmt.Printf("non_empty(data): %v\n", non_empty2(data))
	fmt.Printf("data: %v\n", data)
}

func non_empty2(strings []string) []string {
	var s2 []string
	for _, v := range strings {
		if v != "" {
			s2 = append(s2, v)
		}
	}
	return s2
}

// returns a slice holding only the non-empty strings
// the underlying array is modified during the call
func non_empty(strings []string) []string {
	i := 0
	for _, v := range strings {
		if v != "" {
			strings[i] = v
			i++
		}
	}
	return strings[:i]
}

func append_example() {
	var x, y []int
	for i := 0; i < 10; i++ {
		y = append(x, i)
		fmt.Printf("%d  cap=%d\t%v\n", i, cap(y), y)
		x = y
	}
}

func reverse_example() {
	a := [...]int{0, 1, 2, 3, 4, 5}
	fmt.Printf("array a: %v\n", a)
	reverse(a[:])
	fmt.Printf("array a reversed: %v\n", a)
	var b = [...]int{0, 1, 2, 3, 4, 5, 6}
	fmt.Printf("array b: %v\n", b)
	reverse_array_pointer(&b)
	fmt.Printf("array b reversed: %v\n", b)
}

func reverse(s []int) {
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
}

func reverse_array_pointer(s *[7]int) {
	for i, j := 0, 6; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
}

func remove_adjacent_dups_example() {
	s := []string{"hi", "hi", "world"}
	s = remove_adjacent_dups(s)
	fmt.Printf("after removing adjacent dups: %s\n", s)

	s = []string{"hi", "world", "again", "again"}
	s = remove_adjacent_dups(s)
	fmt.Printf("after removing adjacent dups: %s\n", s)
}

func remove_adjacent_dups(s []string) []string {
	if len(s) <= 1 {
		return s
	}
	var s2 []string
	for i, j := 0, 1; j < len(s); {
		for ; j < len(s) && s[i] == s[j]; j++ {
		}
		s2 = append(s2, s[i])
		i = j
	}
	return s2
}

func month_example() {
	months := [...]string{
		"January",
		"February",
		"March",
		"April",
		"May",
		"June",
		"July",
		"August",
		"September",
		"October",
		"November",
		"December",
	}

	q2 := months[4:7]
	fmt.Printf("q2 months: %v\n", q2)
	fmt.Printf("q2 len: %d, cap: %d\n", len(q2), cap(q2)) // len: 3, cap: 8

	summer := months[6:9]
	fmt.Printf("summer months: %v\n", summer)
	fmt.Printf("summer len: %d, cap: %d\n", len(summer), cap(summer)) // len: 3, cap: 6
}

// https://gobyexample.com/slices
func full_example() {
	var s []string
	fmt.Println("uninitialized:", s, s == nil, len(s) == 0)

	s = make([]string, 3, 5)
	fmt.Printf("empty slice: %s, len: %d, cap: %d\n", s, len(s), cap(s))

	s[0] = "a"
	s[1] = "b"
	s[2] = "c"
	fmt.Println("set:", s)
	fmt.Println("get:", s[2])
	s = append(s, "d", "e")
	fmt.Println("set:", s)
	c := make([]string, len(s))
	copy(c, s)
	fmt.Println("copy:", c)

	fmt.Println("s[2:5]", s[2:5])
	fmt.Println("s[:5]", s[:5])
	fmt.Println("s[2:]", s[2:])

	t := []string{"g", "h", "i"}
	fmt.Println("literal:", t)
	t2 := []string{"g", "h", "i"}
	fmt.Println("slice equals:", slices.Equal(t, t2))

	d2 := make([][]int, 3)
	for i := 0; i < len(d2); i++ {
		d2[i] = make([]int, i+1)
		for j := 0; j < len(d2[i]); j++ {
			d2[i][j] = i + j
		}
	}
	fmt.Println("2D array:", d2)
}
