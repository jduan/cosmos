package main

import (
	"fmt"
	"math"
)

type Circle struct {
	x float64
	y float64
	r float64
}

type Rectangle struct {
	x1, y1, x2, y2 float64
}

type Person struct {
	Name string
}

type Android struct {
	// embedded
	Person
	Model string
}

// interface
type Shape interface {
	area() float64
	perimeter() float64
}

func totalArea(shapes ...Shape) float64 {
	var area float64
	for _, s := range shapes {
		area += s.area()
	}
	return area
}

type MultiShape struct {
	shapes []Shape
}

func (m *MultiShape) area() float64 {
	var area float64
	for _, s := range m.shapes {
		area += s.area()
	}
	return area
}

func main() {
	var c Circle
	c = Circle{x: 0, y: 0, r: 5}
	fmt.Println(c)
	// cp := new(Circle)

	cp := &Circle{x: 0, y: 0, r: 5}
	fmt.Println("x:", cp.x)
	fmt.Println("y:", cp.y)
	fmt.Println("r:", cp.r)

	cp.r = 10
	fmt.Println("area of circle:", circleArea(cp))
	fmt.Println("area of circle:", cp.area())
	// you can call area on a non-pointer var as well
	fmt.Println("area of circle:", c.area())

	r := Rectangle{0, 0, 10, 10}
	fmt.Println("area of rectangle:", r.area())

	a := &Android{Person: Person{Name: "John"}, Model: "TN100"}
	a.Person.Talk()
	// this works too because of embedding
	a.Talk()

	// interface
	fmt.Printf("Total area of c and r is: %v\n", totalArea(&c, &r))

	multiShape := MultiShape{
		shapes: []Shape{
			&Circle{0, 0, 5},
			&Rectangle{0, 0, 10, 10},
		},
	}
	fmt.Println("multishape:", multiShape)
	fmt.Println("area of multishape:", multiShape.area())

	fmt.Println("perimeter of circle:", c.perimeter())
	fmt.Println("perimeter of rectangle:", r.perimeter())
}

func circleArea(c *Circle) float64 {
	return math.Pi * c.r * c.r
}

// method
// "(c *Circle)" is the "receiver"
func (c *Circle) area() float64 {
	return math.Pi * c.r * c.r
}

func distance(x1, y1, x2, y2 float64) float64 {
	a := x2 - x1
	b := y2 - y1
	return math.Sqrt(a*a + b*b)
}

func (r *Rectangle) area() float64 {
	l := distance(r.x1, r.y1, r.x1, r.y2)
	w := distance(r.x1, r.y1, r.x2, r.y1)
	return l * w
}

func (p *Person) Talk() {
	fmt.Println("Hi, my name is", p.Name)
}

func (c *Circle) perimeter() float64 {
	return 2 * math.Pi * c.r
}

func (r *Rectangle) perimeter() float64 {
	a := math.Abs(r.x2 - r.x1)
	b := math.Abs(r.y2 - r.y1)
	return 2 * (a + b)
}
