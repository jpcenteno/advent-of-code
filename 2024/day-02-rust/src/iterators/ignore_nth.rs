use std::iter::Enumerate;

pub trait IgnoreNth: Iterator {
    fn ignore_nth(self, n: usize) -> IgnoreNthIterator<Self>
    where 
        Self: Sized,
    {
        let iter = self.enumerate();
        IgnoreNthIterator { iter, n }
    }
}

impl<I> IgnoreNth for I where I: Iterator {}

pub struct IgnoreNthIterator<I: Iterator> {
    iter: Enumerate<I>,
    n: usize
}

impl<I> Iterator for IgnoreNthIterator<I>
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
