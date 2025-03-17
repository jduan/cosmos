package main

import "fmt"

func main() {
	var runes []rune
	fmt.Printf("len: %d\n", len(runes))
	fmt.Printf("cap: %d\n", cap(runes))

	res := plus(1, 2)
	fmt.Println("1+2=", res)

	res = plusPlus(1, 2, 3)
	fmt.Println("1+2+3=", res)

	a, b := vals()
	fmt.Println("a=", a)
	fmt.Println("b=", b)

	sum(1, 2)
	sum(1, 2, 3)
	nums := []int{1, 2, 3, 4}
	sum(nums...) // unpack the slice

	nextInt := intSeq()
	fmt.Println("next int:", nextInt())
	fmt.Println("next int:", nextInt())
	fmt.Println("next int:", nextInt())

	newInts := intSeq()
	fmt.Println("new next int:", newInts())
}

func plus(a int, b int) int {
	return a + b
}

func plusPlus(a, b, c int) int {
	return a + b + c
}

func vals() (int, int) {
	return 3, 7
}

func sum(nums ...int) {
	fmt.Print(nums, " ")
	total := 0
	for _, num := range nums {
		total += num
	}
	fmt.Println(total)
}

// closures
func intSeq() func() int {
	i := 0
	return func() int {
		i++
		return i
	}
}
