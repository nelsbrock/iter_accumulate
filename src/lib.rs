#![no_std]
#![warn(missing_docs)]

//! This crate provides the [`accumulate()`](IterAccumulate::accumulate) iterator adaptor.

/// An iterator adaptor that accumulates the elements from the base iterator using the provided
/// closure.
///
/// See [`IterAccumulate::accumulate()`] for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Accumulate<I, B, F> {
    iter: I,
    acc: B,
    f: F,
}

impl<I, B, F> Accumulate<I, B, F> {
    fn new(iter: I, acc: B, f: F) -> Accumulate<I, B, F> {
        Accumulate { iter, acc, f }
    }
}

impl<I, B, F> Iterator for Accumulate<I, B, F>
where
    I: Iterator,
    B: Copy,
    F: FnMut(B, I::Item) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => {
                self.acc = (self.f)(self.acc, item);
                Some(self.acc)
            }
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }
}

/// An [`Iterator`] blanket implementation that provides the [`accumulate()`](Self::accumulate)
/// function.
pub trait IterAccumulate: Iterator {
    /// Creates an iterator adaptor that accumulates the elements from the base iterator using the
    /// provided closure.
    ///
    /// `accumulate()` is similar to [`fold()`], but instead of returning the final accumulated
    /// result, it returns an iterator that yields the current accumulated value for each iteration.
    /// In other words, the last element yielded by `accumulate()` is what would have been returned
    /// by [`fold()`] if it was used instead.
    ///
    /// The returned iterator is **not** fused and it is not specified what happens when the base
    /// iterator returns [`None`].
    /// If you want a fused iterator, use [`fuse()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_accumulate::IterAccumulate;
    ///
    /// let input = [1, 2, 3, 4, 5];
    /// let result = input
    ///     .into_iter()
    ///     .accumulate(1, |acc, i| acc * i)
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(result, vec![1, 2, 6, 24, 120]);
    /// ```
    ///
    /// [`fold()`]: Iterator::fold
    /// [`fuse()`]: Iterator::fuse
    #[inline]
    fn accumulate<B, F>(self, init: B, f: F) -> Accumulate<Self, B, F>
    where
        Self: Sized,
        B: Copy,
        F: FnMut(B, Self::Item) -> B,
    {
        Accumulate::new(self, init, f)
    }
}

impl<I: Iterator> IterAccumulate for I {}
