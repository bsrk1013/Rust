#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.length
    }
}

pub fn add_two(data: i32) -> i32 {
    data + 2
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, value:{}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn add_two_test() {
        let data = 5;
        assert_eq!(add_two(data), data + 2);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn generator_guess() {
        let guess = Guess::new(200);
    }
}
