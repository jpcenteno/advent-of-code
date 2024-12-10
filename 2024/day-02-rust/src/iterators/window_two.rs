use std::mem;

pub trait WindowTwo: Iterator {
    fn window_two(mut self) -> WindowTwoIterator<Self>
    where
        Self: Sized,
        Self::Item: Copy,
    {
        // We don't care about panics given the assumption that the iterator is not empty.
        let prev = self.next().unwrap();

        // WindowTwoIterator takes ownership of the wrapped iterator.
        WindowTwoIterator {
            iter: self,
            prev
        }
    }
}

impl<T> WindowTwo for T where T: Iterator {}

pub struct WindowTwoIterator<I: Iterator> {
    iter: I,
    prev: I::Item
}

impl<I> Iterator for WindowTwoIterator<I>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.iter.next() {
            let prev = mem::replace(&mut self.prev, next);
            Some((prev, next))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_window_two_iterator() {
        let input = vec![1, 2, 3, 4];
        let expected = vec![(1, 2), (2, 3), (3, 4)];
        let actual: Vec<(u64, u64)> = input.iter().window_two().map(|(a, b)| (a.to_owned(), b.to_owned())).collect();
        assert_eq!(expected, actual);
    }
}
