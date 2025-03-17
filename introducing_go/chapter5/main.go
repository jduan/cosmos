package main

import "fmt"

func main() {
	array_example()
	array_example2()
	slice_example()
	map_example()
	map_example2()
	nums := []int{
		48, 96, 86, 68,
		57, 82, 63, 70,
		37, 34, 83, 27,
		19, 97, 9, 17,
	}
	find_smallest(nums)
}

func array_example() {
	fmt.Println("============= array_example ==============")
	var x [5]int
	x[4] = 100
	fmt.Println(x)
}

func array_example2() {
	fmt.Println("============= array_example2 ==============")
	x := [5]float64{98, 93, 77, 82, 83}
	fmt.Println(x)

	total := 0.0
	for i := 0; i < len(x); i++ {
		total += x[i]
	}
	fmt.Println(total / float64(len(x)))

	total = 0
	for _, value := range x {
		total += value
	}
	fmt.Println(total / float64(len(x)))
}

func slice_example() {
	fmt.Println("============= slice_example ==============")
	// make a slice of 5 floats with capacity 10
	x := make([]float64, 5, 10)
	fmt.Printf("x=%v\n", x)

	x = append(x, 1, 2)
	x = append(x, 3, 4, 5)
	fmt.Printf("x=%v\n", x)

	fmt.Println("copy a slice")
	slice1 := []int{1, 2, 3}
	slice2 := make([]int, 2)
	copy(slice2, slice1)
	fmt.Printf("slice1: %v\n", slice1)
	fmt.Printf("slice2: %v\n", slice2)
}

func map_example() {
	fmt.Println("============= map_example ==============")
	x := make(map[string]int)
	x["key"] = 10
	fmt.Println(x)
}

func map_example2() {
	fmt.Println("============= map_example2 ==============")
	elements := map[string]map[string]string{
		"H": map[string]string{
			"name":  "Hydrogen",
			"state": "gas",
		},
		"He": map[string]string{
			"name":  "Helium",
			"state": "gas",
		},
		"Li": map[string]string{
			"name":  "Lithium",
			"state": "solid",
		},
	}

	fmt.Printf("map size: %d\n", len(elements))
	if el, ok := elements["Li"]; ok {
		fmt.Println(el["name"], el["state"])
	}
	if _, ok := elements["G"]; !ok {
		fmt.Println("key G doesn't exist in map")
	}
}

func find_smallest(nums []int) int {
	fmt.Println("============= find_smallest ==============")
	smallest := nums[0]
	for _, num := range nums {
		if num < smallest {
			smallest = num
		}
	}
	fmt.Println("smallest number is", smallest)
	return smallest
}
