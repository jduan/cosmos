package main

import (
	"fmt"
)

// y is a package-level variable
// Go is lexically scoped using blocks
var y = "Hello, 世界"

func main() {
	var x string = "Hello, world"
	fmt.Println(x)

	x = "something differnet"
	fmt.Println(x)

	f()

	// define multiple vars
	var (
		a = 5
		b = 10
		c = 15
	)
	var d, e, f int = 1, 2, 3

	fmt.Printf("a = %d, b = %d, c = %d\n", a, b, c)
	fmt.Printf("d = %d, e = %d, f = %d\n", d, e, f)

	fmt.Print("Enter a number: ")
	var input float64
	fmt.Scanf("%f", &input)
	output := input * 2
	fmt.Println("Double of that number is", output)

	temperature := 72.0
	fmt.Printf("72 Fahrenheit is the same as %f Celsium\n", fToC(temperature))

	height := 6.2
	fmt.Printf("6.2 feet is the same as %f meters\n", feetToMeter(height))
}

func f() {
	fmt.Println(y)
}

// convert from Fahrenheit to Celsium
func fToC(f float64) float64 {
	return (f - 32) * 5 / 9
}

func feetToMeter(f float64) float64 {
	return f * 0.3048
}
