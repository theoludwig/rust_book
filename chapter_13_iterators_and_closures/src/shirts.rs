use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, EnumIter, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub trait TotalOccurrences<T: PartialEq> {
    fn total_occurrences(&self, value: T) -> usize;
}

impl<T: PartialEq> TotalOccurrences<T> for Vec<T> {
    fn total_occurrences(&self, value: T) -> usize {
        self.iter().filter(|&item| *item == value).count()
    }
}

#[derive(Debug)]
pub struct ShirtsInventory {
    pub shirts: Vec<ShirtColor>,
}

impl ShirtsInventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    /// Returns the [`ShirtColor`] variant that occurs the most in this [`ShirtsInventory`].
    ///
    /// # Panics
    ///
    /// Panics if the [`ShirtColor`] enum has no variants.
    ///
    /// # Examples
    ///
    /// ```
    /// let store = ShirtsInventory {
    ///     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    /// };
    /// assert_eq!(ShirtColor::Blue, store.most_stocked());
    /// ```
    pub fn most_stocked(&self) -> ShirtColor {
        let mut iter = ShirtColor::iter();
        let mut result = iter
            .next()
            .expect("`ShirtColor` enum should have at least one enum variant.");
        let mut result_total = self.shirts.total_occurrences(result);
        for shirt_color in iter {
            let shirt_color_total = self.shirts.total_occurrences(shirt_color);
            if shirt_color_total > result_total {
                result = shirt_color;
                result_total = shirt_color_total;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_occurrences() {
        let numbers: Vec<i32> = vec![1, 2, 3, 3, 3];
        assert_eq!(numbers.total_occurrences(1), 1);
        assert_eq!(numbers.total_occurrences(2), 1);
        assert_eq!(numbers.total_occurrences(3), 3);
        assert_eq!(numbers.total_occurrences(4), 0);
    }
}
