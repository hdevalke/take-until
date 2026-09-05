use core::fmt;
use core::iter::FusedIterator;

/// TakeUntilExt is an extension trait for iterators.
/// It adds the [`Self::take_until`] method.
pub trait TakeUntilExt<P>
where
    Self: Sized,
{
    /// Accepts elements until a predicate is true. Using this iterator is
    /// equivalent to an **inclusive** [`Iterator::take_while`] with a negated
    /// condition.
    ///
    /// # Example
    ///
    /// ## Parsing the next base 128 varint from a byte slice.
    ///
    /// ```rust
    /// use take_until::TakeUntilExt;
    ///
    /// let varint = &[0b1010_1100u8, 0b0000_0010, 0b1000_0001];
    /// let int: u32 = varint
    ///     .iter()
    ///     .take_until(|b| (**b & 0b1000_0000) == 0)
    ///     .enumerate()
    ///     .fold(0, |acc, (i, b)| {
    ///         acc | ((*b & 0b0111_1111) as u32) << (i * 7)
    ///      });
    /// assert_eq!(300, int);
    /// ```
    ///
    /// ## Take Until vs Take While (from Standard Library)
    /// ```rust
    /// use take_until::TakeUntilExt;
    ///
    /// let items = [1, 2, 3, 4, -5, -6, -7, -8];
    /// let filtered_take_while = items
    ///     .into_iter()
    ///     .take_while(|x| *x > 0)
    ///     .collect::<Vec<i32>>();
    /// let filtered_take_until = items
    ///     .into_iter()
    ///     .take_until(|x| *x <= 0)
    ///     .collect::<Vec<i32>>();
    /// assert_eq!([1, 2, 3, 4], filtered_take_while.as_slice());
    /// assert_eq!([1, 2, 3, 4, -5], filtered_take_until.as_slice());
    /// ```
    fn take_until(self, predicate: P) -> TakeUntil<Self, P>;
}

impl<I, P> TakeUntilExt<P> for I
where
    I: Sized + Iterator,
    P: FnMut(&I::Item) -> bool,
{
    fn take_until(self, predicate: P) -> TakeUntil<Self, P> {
        TakeUntil {
            iter: self,
            flag: false,
            predicate,
        }
    }
}
/// TakeUntil is similar to the TakeWhile iterator,
/// but takes items until the predicate is true,
/// including the item that made the predicate true.
pub struct TakeUntil<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I: fmt::Debug, P> fmt::Debug for TakeUntil<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TakeUntil")
            .field("iter", &self.iter)
            .field("flag", &self.flag)
            .finish()
    }
}

impl<I, P> Iterator for TakeUntil<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            return None;
        }
        let item = self.iter.next()?;
        self.flag = (self.predicate)(&item);
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.flag {
            (0, Some(0))
        } else {
            let (lower, upper) = self.iter.size_hint();
            (lower.min(1), upper) // The stopping item is included.
        }
    }
}

impl<I, P> FusedIterator for TakeUntil<I, P>
where
    I: FusedIterator,
    P: FnMut(&I::Item) -> bool,
{
}
