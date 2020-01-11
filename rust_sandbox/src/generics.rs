pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// we can also implement methods on concrete Point types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// You can use as many generic type parameters in a definition as you want, but using more than a
// few makes your code hard to read. When you need lots of generic types in your code, it could
// indicate that your code needs restructuring into smaller pieces.
#[derive(Debug)]
pub struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    // interacts with another struct of different types
    // here the generic types V, W only apply to this method, not the struct itself
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// redefinition of std Option
pub enum MyOption<T> {
    Some(T),
    None,
}

// redefinition of std Result
pub enum MyResult<T, U> {
    Ok(T),
    Err(U),
}

pub fn find_largest_number(numbers: &[i32]) -> i32 {
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num
        }
    }

    largest
}

pub fn find_largest_number_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        let number_list = vec![34, 50, 25, 100, 65];
        assert_eq!(100, find_largest_number(&number_list));

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        assert_eq!(6000, find_largest_number(&number_list));

        let char_list = vec!['y', 'm', 'a', 'q'];
        assert_eq!('y', find_largest_number_generic(&char_list));

        let integer = Point { x: 5, y: 10 };
        assert_eq!(5, integer.x);
        let float = Point { x: 1.0, y: 4.0 };

        let integer_and_float = Point2 { x: 5, y: 4.0 };
        let p2 = Point2 { x: "Hello", y: 'c' };
        let p3 = integer_and_float.mixup(p2);
        assert_eq!(5, p3.x);
        assert_eq!('c', p3.y);
    }
}
