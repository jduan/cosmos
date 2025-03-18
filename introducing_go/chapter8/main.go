package main

import (
	"container/list"
	"fmt"
	"os"
	"sort"
	"strings"
	"time"
)

func main() {
	// string_example()
	// file_example()
	// file_example2()
	// linked_list_example()
	// sort_example()
	server_example()
}

func string_example() {
	fmt.Println(strings.Contains("test", "es"))
	fmt.Println(strings.Count("test", "t"))
	fmt.Println(strings.HasPrefix("test", "te"))
	fmt.Println(strings.HasSuffix("test", "st"))
	fmt.Println(strings.Index("test", "e"))
	fmt.Println(strings.Join([]string{"a", "b"}, "-"))
	fmt.Println(strings.Repeat("test", 5))
	fmt.Println(strings.Replace("aaaa", "a", "b", 2))
	fmt.Println(strings.Split("a-b-c-d-e", "-"))
	fmt.Println(strings.ToLower("TESt"))
}

func file_example() {
	file, err := os.Open("test.txt")
	if err != nil {
		// handle the error here
		fmt.Println("Failed to open file", err)
		return
	}
	defer file.Close()

	stat, err := file.Stat()
	if err != nil {
		fmt.Println("Failed to get file stat", err)
	}

	bs := make([]byte, stat.Size())
	_, err = file.Read(bs)
	if err != nil {
		fmt.Println("Failed to read file", err)
	}

	str := string(bs)
	fmt.Println(str)
}

// a shorter way to read a file
func file_example2() {
	bs, err := os.ReadFile("test.txt")
	if err != nil {
		fmt.Println("Failed to read test.txt", err)
	}
	str := string(bs)
	fmt.Println(str)
}

func linked_list_example() {
	var x list.List
	x.PushBack(1)
	x.PushBack(2)
	x.PushBack(3)

	for e := x.Front(); e != nil; e = e.Next() {
		// e.Value Stores the value as interface{} (since list supports any type).
		// Type assertion: e.Value.(int) Converts interface{} to int.
		fmt.Println(e.Value.(int))
	}
}

type Person struct {
	Name string
	Age  int
}

type ByName []Person

func (p ByName) Len() int {
	return len(p)
}

func (p ByName) Less(i, j int) bool {
	return p[i].Name < p[j].Name
}

func (p ByName) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func sort_example() {
	kids := ByName{
		Person{"Lucy", 10},
		Person{"Monica", 6},
		Person{"John", 9},
	}
	fmt.Println("kids before sort", kids)
	sort.Sort(ByName(kids))
	fmt.Println("kids after sort", kids)
}

func server_example() {
	go server()
	time.Sleep(2 * time.Second)
	go client()

	fmt.Println("type anything to exit")
	var input string
	fmt.Scanln(&input)
}
