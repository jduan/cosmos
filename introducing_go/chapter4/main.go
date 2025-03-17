package main

import "fmt"

func main() {
	for_and_if()
	switch_example(3)
	switch_example(8)
	divisible_by_3(1, 100)
	fizzbuzz()
}

func divisible_by_3(start int, end int) {
	fmt.Printf("Numbers between %d and %d that are divisible by 3:\n", start, end)
	for i := start; i <= end; i++ {
		if i%3 == 0 {
			fmt.Print(i, " ")
		}
	}
	fmt.Println()
}

func fizzbuzz() {
	for i := 1; i <= 100; i++ {
		if i%3 == 0 && i%5 == 0 {
			fmt.Println(i, "FizzBuzz")
		} else if i%3 == 0 {
			fmt.Println(i, "Fizz")
		} else if i%5 == 0 {
			fmt.Println(i, "Buzz")
		}
	}
}

func for_and_if() {
	for i := 1; i <= 10; i++ {
		if i%2 == 0 {
			fmt.Println(i, "even")
		} else {
			fmt.Println(i, "odd")
		}
	}
}

func switch_example(i int) {
	switch i {
	case 0:
		fmt.Println("zero")
	case 1:
		fmt.Println("one")
	case 2:
		fmt.Println("two")
	case 3:
		fmt.Println("three")
	case 4:
		fmt.Println("four")
	default:
		fmt.Println("Unknown number")
	}
}
