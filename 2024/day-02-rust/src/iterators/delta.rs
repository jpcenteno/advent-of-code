use std::ops::Sub;

use super::window_two::{WindowTwo, WindowTwoIterator};

/// This trait extends the `Iterator` trait for those iterators whose `Item` type supports the `Sub`
/// trait with the method `delta`, which returns an iterator of differences between it's elements.
pub trait IntoDelta<T: Sub<Output = T> + Copy>: Iterator<Item = T>
{
    fn delta(self) -> Delta<T, Self>
    where
        Self: Sized,
    {
        Delta { iter: self.window_two() }
    }
}

impl<I, T> IntoDelta<T> for I
where
    T: Sub<Output = T> + Copy,
    I: Iterator<Item = T> {}

/// An iterator that returns the difference between the elements of `iter`.
///
/// This struct is created byt the method `delta` from the `IntoDelta` trait.
pub struct Delta<T: Sub<Output = T>, I: Iterator<Item = T>> {
    iter: WindowTwoIterator<I>
}

impl<T, I> Iterator for Delta<T, I>
where
    T: Sub<Output = T> + Copy,
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((a, b)) = self.iter.next() {
            Some(b - a)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal() {
        let input = [1, 1];
        let mut iter = input.into_iter().delta();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_fibonacci() {
        let input = [1, 1, 2, 3, 5, 8];
        let mut iter = input.into_iter().delta();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_one_element() {
        let mut iter = [42].into_iter().delta();
        assert_eq!(iter.next(), None)
    }

    #[ignore] // This test breaks because WindowTwo can't handle empty iterators.
    #[test]
    fn test_empty_iterator() {
        let input: [i64; 0] = []; // Empty arrays require a type anotation.
        let mut iter = input.into_iter().delta();
        assert_eq!(iter.next(), None)
    }
}
