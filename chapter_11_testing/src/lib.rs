use std::ops::Add;

/// Add some `values` together.
///
/// # Panics
///
/// Panics if `values` is empty.
///
/// # Examples
///
/// ```
/// let values = [1, 2, 3];
/// let result = chapter_11_testing::add(&values);
/// assert_eq!(result, 6);
/// ```
pub fn add<T: Add<T, Output = T> + Clone>(values: &[T]) -> T {
    if values.is_empty() {
        panic!("`values` should not be empty.");
    }
    let mut result = values[0].clone();
    for item in values.iter().skip(1) {
        result = result + item.clone();
    }
    result
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Rectangle) -> bool {
        self.width == other.width && self.height == other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(3, add(&[1, 2]), "1 + 2 should be 3");
    }

    #[test]
    #[should_panic(expected = "`values` should not be empty.")]
    pub fn test_add_panic() {
        let empty: Vec<i32> = Vec::new();
        add(&empty);
    }

    #[test]
    fn test_rectangle_can_hold() {
        let rectangle1 = Rectangle {
            width: 10,
            height: 10,
        };
        let rectangle2 = Rectangle {
            width: 5,
            height: 5,
        };
        let rectangle3 = Rectangle {
            width: 15,
            height: 15,
        };
        assert!(rectangle1.can_hold(&rectangle2));
        assert!(!rectangle1.can_hold(&rectangle3));
    }

    #[test]
    pub fn test_rectangle_equalities() {
        let rectangle1 = Rectangle {
            width: 10,
            height: 10,
        };
        let rectangle2 = Rectangle {
            width: 10,
            height: 10,
        };
        let rectangle3 = Rectangle {
            width: 15,
            height: 15,
        };
        assert_eq!(rectangle1, rectangle2);
        assert_ne!(rectangle1, rectangle3);
    }
}
