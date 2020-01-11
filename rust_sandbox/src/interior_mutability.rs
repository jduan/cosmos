// Immutability is great. But sometimes you want a little bit of mutable data inside an otherwise
// Immutabile value. This is called "interior mutability". Rust offers a few flavors of it.
//
// A Cell<T> is a struct that contains a single private value of type T. The only special thing
// about a Cell is that you can get and set the field even if you don't have mut access to the Cell
// itself:
//
// Cell::new(value) creates a new Cell object
// cell.get() returns a copy of the value in the cell
// cell.set(value) stores the given value in the cell, dropping the previously stored value
//      Note that this "set" method takes self as a non-mut reference. This is why Cell is special.
//
//
// Cell doesn't let you call "mut" methods on a shared value. Also, the .get() method returns a
// copy of the value in the cell, so it only works if T implements the Copy trait.
//
// RefCell<T> is similar to Cell<T> but it supports borrowing references to its T value:
//
// RefCell::new(value) creates a new RefCell object
// ref_cell.borrow() returns a Ref<T>, which is a shared ref to the value
// ref_cell.borrow_mut() returns a RefMut<T>, a mutable ref to the value
//      This method panics if the value is already borrowed.
//
//
// Normally, when you borrow a reference to a variable, Rust checks at compile time to ensure that
// you're using the reference safely. If the check fails, you get a compiler error. RefCell
// enforces the same rule using "runtime checks". So you're breaking the rules, you get a panic.
//
// Having to call .get() and .set() or .borrow() and .borrow_mut() is slightly awkward, but that's
// just the price we pay for bending the rules. The other drawback is less obvious and more
// serious: cells (and any types that contain them) are not thread-safe. Rust therefore will not
// allow multiple threads to access them at once. For that, you will need Mutex, Atomics, etc.

use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::Write;

pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    hardware_error_count: Cell<u32>,
    log_file: RefCell<File>,
}

impl SpiderRobot {
    pub fn new(species: String) -> SpiderRobot {
        let log_file = File::create("/tmp/spider_robot.log").unwrap();

        SpiderRobot {
            species: species,
            web_enabled: false,
            hardware_error_count: Cell::new(0),
            log_file: RefCell::new(log_file),
        }
    }

    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }

    pub fn log(&self, message: &str) {
        let mut file = self.log_file.borrow_mut();
        writeln!(file, "{}", message).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell() {
        let robot = SpiderRobot::new(String::from("automatic"));
        robot.add_hardware_error();
        robot.add_hardware_error();
        assert!(robot.has_hardware_errors());
    }

    #[test]
    #[should_panic]
    fn test_ref_cell() {
        let ref_cell: RefCell<String> = RefCell::new("hello".to_string());

        let r = ref_cell.borrow();
        let count = r.len();
        assert_eq!(5, count);

        // this will panic because the ref_cell is also borrowed above by r
        let mut w = ref_cell.borrow_mut();
        w.push_str(" world");
    }
}
