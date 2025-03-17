package main

import "fmt"

func main() {
	xs := []float64{98, 93, 77, 82, 83}
	fmt.Println("avergage is", average(xs))
	fmt.Println(add(1, 2, 3))

	xs2 := []int{4, 5, 6}
	fmt.Println(add(xs2...))

	closure_example()
	nextEven := makeEventGenerator()
	fmt.Println(nextEven())
	fmt.Println(nextEven())
	fmt.Println(nextEven())

	fmt.Printf("factorial(10)=%d\n", factorial(10))

	panic_and_defer()
	pointer_example()

	fmt.Println(half_number(1))
	fmt.Println(half_number(2))

	fmt.Printf("Largest number in [1, 2, 3] is %d\n", find_largest(1, 2, 3))

	x := 1
	y := 2
	fmt.Printf("Before swapping, x=%d, y=%d\n", x, y)
	swap(&x, &y)
	fmt.Printf("After swapping, x=%d, y=%d\n", x, y)
}

func average(xs []float64) float64 {
	total := 0.0
	for _, v := range xs {
		total += v
	}
	return total / float64(len(xs))
}

// variadic args
func add(args ...int) int {
	total := 0
	for _, v := range args {
		total += v
	}
	return total
}

func closure_example() {
	fmt.Println("============= closure_example ==============")
	x := 0
	increment := func() int {
		x++
		return x
	}
	fmt.Println(increment())
	fmt.Println(increment())
}

func makeEventGenerator() func() uint {
	fmt.Println("============= makeEventGenerator ==============")
	i := uint(0)
	return func() uint {
		ret := i
		i += 2
		return ret
	}
}

func factorial(x uint) uint {
	if x == 0 {
		return 1
	} else {
		return x * factorial(x-1)
	}
}

func panic_and_defer() {
	fmt.Println("============= panic_and_defer ==============")
	defer func() {
		str := recover()
		fmt.Println(str)
	}()
	panic("PANIC!")
}

func pointer_example() {
	fmt.Println("============= pointer_example ==============")
	// Go doesn't allow nested functions but anonymous functions are ok
	zero := func(xp *int) {
		*xp = 0
	}

	x := 5
	fmt.Println("x=", x)
	zero(&x)
	fmt.Println("x=", x)
}

func half_number(num int) (int, bool) {
	half := int(num / 2)
	if num%2 == 0 {
		return half, true
	} else {
		return half, false
	}
}

func find_largest(nums ...int) int {
	largest := nums[0]
	for _, v := range nums {
		if v > largest {
			largest = v
		}
	}
	return largest
}

func swap(x *int, y *int) {
	*x, *y = *y, *x
}
