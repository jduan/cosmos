pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    println!("Area of square is {}", square.area());
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // we use & here becaue we don't want to take ownership and we just want to read the data in
    // the struct, not write to it.
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn enlarge_by(&mut self, by: u32) {
        self.width *= by;
        self.height *= by;
    }

    // return if it can hold another rectangle completely
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Pair owns resources: two heap allocated integers.
#[derive(Debug)]
pub struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) -> i32 {
        let Pair(first, second) = self;
        first.as_ref() + second.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
        assert_eq!(2700, rect3.area());
    }

    #[test]
    fn test_enlarge() {
        let mut rect3 = Rectangle {
            width: 60,
            height: 45,
        };
        assert_eq!(2700, rect3.area());
        rect3.enlarge_by(10);
        assert_eq!(270000, rect3.area());
    }

    #[test]
    fn test_pair() {
        let pair = Pair(Box::new(3), Box::new(5));
        assert_eq!(8, pair.destroy());
        // This line doesn't compile because "destroy" consumes the pair object.
        // println!("pair is {:?}", pair);
    }
}
