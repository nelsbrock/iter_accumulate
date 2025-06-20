#![no_std]
#![warn(missing_docs)]

//! This crate provides [`accumulate()`], an iterator adaptor that accumulates the elements from the
//! base iterator using the provided closure.
//!
//! [`accumulate()`] takes two arguments: an initial value, and a closure with two arguments:
//! an 'accumulator', and an element.
//!
//! The initial value is the value the accumulator will have when the closure is first called.
//! On each call to [`next()`], the closure is executed with the current accumulator and the element
//! yielded by the upstream iterator. The return value of the closure is then set as the new value
//! of the accumulator and returned to the caller.
//!
//! Since the accumulated value needs to be both stored as the accumulator *and* returned to the
//! caller, the accumulator type must implement [`Copy`]. If you want to operate on non-copyable
//! types, you should use [`Iterator::scan`] instead.
//!
//! The returned iterator is **not** fused and it is not specified what happens when the base
//! iterator returns [`None`].
//! If you want a fused iterator, use [`fuse()`].
//!
//! # Differences to [`fold()`]
//!
//! In principle, [`accumulate()`] is similar to [`fold()`]. However, instead of returning the final
//! accumulated result, it returns an iterator that yields the current value of the accumulator for
//! each iteration. In other words, the last element yielded by [`accumulate()`] is what would have
//! been returned by [`fold()`] if it had been used instead.
//!
//! # Examples
//!
//! ```
//! use iter_accumulate::IterAccumulate;
//!
//! let input = [1, 2, 3, 4, 5];
//! let mut iter = input.iter().accumulate(1, |acc, i| acc * i);
//!
//! assert_eq!(iter.next(), Some(1));
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.next(), Some(6));
//! assert_eq!(iter.next(), Some(24));
//! assert_eq!(iter.next(), Some(120));
//! assert_eq!(iter.next(), None);
//! ```
//!
//! [`accumulate()`]: IterAccumulate::accumulate
//! [`fold()`]: Iterator::fold
//! [`next()`]: Iterator::next
//! [`fuse()`]: Iterator::fuse

use core::fmt;

/// An iterator adaptor that accumulates the elements from the base iterator using the provided
/// closure.
///
/// See the [crate-level documentation](crate) for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Accumulate<I, B, F> {
    iter: I,
    acc: B,
    f: F,
}

impl<I, B, F> Accumulate<I, B, F> {
    fn new(iter: I, acc: B, f: F) -> Self {
        Self { iter, acc, f }
    }
}

impl<I, B, F> fmt::Debug for Accumulate<I, B, F>
where
    I: fmt::Debug,
    B: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Accumulate")
            .field("iter", &self.iter)
            .field("acc", &self.acc)
            .finish_non_exhaustive()
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
    /// See the [crate-level documentation](crate) for more information.
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
