/// There are three types of structures ("structs") that can be created using the struct keyword:
//
//    Tuple structs, which are, basically, named tuples.
//    The classic C structs
//    Unit structs, which are field-less, are useful for generics.

/// A tuple struct
pub struct Pair(i32, f32);

/// A unit struct
pub struct Nil;

/// A classic C struct
pub struct Point {
    x: f32,
    y: f32,
}

/// A C struct with pointers
#[derive(Debug)]
pub struct Person<'a> {
    name: &'a str,
    age: u8,
}

/// A struct can have another struct
pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        ((self.p1.x - self.p2.x) * (self.p1.y - self.p2.y)).abs()
    }
}

pub fn square(p: Point, width: f32) -> Rectangle {
    let p2 = Point {
        x: p.x + width,
        y: p.y + width,
    };
    Rectangle { p1: p, p2 }
}

#[cfg(test)]
mod tests {
    use crate::structs::{square, Pair, Person, Point, Rectangle};

    #[test]
    fn test_person() {
        let person = Person {
            name: "Peter",
            age: 27,
        };
        assert_eq!(27, person.age);
    }

    #[test]
    fn test_point() {
        let point = Point { x: 0.3, y: 0.4 };
        assert!(approx_eq!(f32, 0.7, point.x + point.y));

        // Create another point from an existing point
        let point2 = Point { x: 0.1, ..point };
        assert!(approx_eq!(f32, 0.5, point2.x + point2.y));

        // destruct
        let Point { x: my_x, y: my_y } = point2;
        assert!(approx_eq!(f32, 0.1, my_x));
        assert!(approx_eq!(f32, 0.4, my_y));

        let rect = Rectangle {
            p1: point,
            p2: Point { x: 0.9, y: 0.8 },
        };
        assert_eq!(0.24, rect.area());
    }

    #[test]
    fn test_tuple_struct() {
        let pair = Pair(1, 0.1);
        assert_eq!(1, pair.0);

        let Pair(integer, decimal) = pair;
        assert_eq!(1, integer);
    }

    #[test]
    fn test_square() {
        let point = Point { x: 1.0, y: 1.0 };
        let rect = square(point, 2.0);
        assert!(approx_eq!(f32, 4.0, rect.area()));
    }
}
