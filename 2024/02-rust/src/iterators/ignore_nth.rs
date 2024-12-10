use std::iter::Enumerate;

/// This trait extends the `Iterator` trait with the method `ignore_nth` that returns a wrapped
/// `Iterator` with type `IgnoreNth` which will skip the `n`-th element of it's input. 
pub trait IgnoreNthTrait: Iterator {
    // Creates an iterator that skips the `n`-th element.
    fn ignore_nth(self, n: usize) -> IgnoreNth<Self>
    where
        Self: Sized,
    {
        let iter = self.enumerate();
        IgnoreNth { iter, n }
    }
}

impl<I> IgnoreNthTrait for I where I: Iterator {}

/// An iterator that skips the `n`-th element of `iter`.
///
/// This struct is created by the `ignore_nth` method of `IgnoreNthTrait`.
pub struct IgnoreNth<I: Iterator> {
    iter: Enumerate<I>,
    n: usize
}

impl<I> Iterator for IgnoreNth<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((i, item)) = self.iter.next() {
            if i == self.n {
                self.next()
            } else {
                Some(item)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: [char; 3] = ['a', 'b', 'c'];

    #[test]
    fn skip_test_0() {
        let mut iter = INPUT.iter().ignore_nth(0);
        assert_eq!(iter.next(), Some(&'b'));
        assert_eq!(iter.next(), Some(&'c'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn skip_test_1() {
        let mut iter = INPUT.iter().ignore_nth(1);
        assert_eq!(iter.next(), Some(&'a'));
        assert_eq!(iter.next(), Some(&'c'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn skip_test_2() {
        let mut iter = INPUT.iter().ignore_nth(2);
        assert_eq!(iter.next(), Some(&'a'));
        assert_eq!(iter.next(), Some(&'b'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn skip_test_4_out_of_bounds() {
        let mut iter = INPUT.iter().ignore_nth(4);
        assert_eq!(iter.next(), Some(&'a'));
        assert_eq!(iter.next(), Some(&'b'));
        assert_eq!(iter.next(), Some(&'c'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn skip_test_5_past_out_of_bounds() {
        let mut iter = INPUT.iter().ignore_nth(5);
        assert_eq!(iter.next(), Some(&'a'));
        assert_eq!(iter.next(), Some(&'b'));
        assert_eq!(iter.next(), Some(&'c'));
        assert_eq!(iter.next(), None);
    }
}
