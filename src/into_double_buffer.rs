//! Provides helper trait for more convenient buffer creation

use crate::DoubleBuffer;

/// Helper trait for more convenient buffer creation
///
/// # Example
///
/// ```
/// use bevy_double_res::{DoubleBuffer, IntoDoubleBuffer};
/// let tuple = (10, 20).into_double_buf();
///
/// assert_eq!(tuple.current(), &(10, 20));
/// assert_eq!(tuple.next(), &(10, 20));
/// assert_eq!(tuple.index(), 0);
/// ```
///
/// # Warning
///
/// This trait is implemented for every cloneable type, but this means references also!
///
/// When inserting resource in the bevy world, is should have type of [DoubleBuffer<T>],
/// not [DoubleBuffer<&T>]!
///
/// Implementation of trait tries to prevent such accidental conversions, but keep in mind
/// this fact when your system panics about missing of **T**, because it might be **&T** existing in
/// the world.
pub trait IntoDoubleBuffer: Clone
where
    Self: Sized,
{
    /// Output of [IntoDoubleBuffer::into_double_buf]
    ///
    /// This exists to prevent references from being accidentally cloned
    type Item;

    /// Method for more convenient buffer creation
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::{DoubleBuffer, IntoDoubleBuffer};
    /// let tuple = (10, 20).into_double_buf();
    ///
    /// assert_eq!(tuple.current(), &(10, 20));
    /// assert_eq!(tuple.next(), &(10, 20));
    /// assert_eq!(tuple.index(), 0);
    /// ```
    ///
    /// # Warning
    ///
    /// [IntoDoubleBuffer] trait is implemented for every cloneable type, but this means references
    /// also!
    ///
    /// When inserting resource in the bevy world, is should have type of [DoubleBuffer<T>],
    /// not [DoubleBuffer<&T>]!
    ///
    /// Implementation of [IntoDoubleBuffer] tries to prevent such accidental conversions, but keep
    /// in mind this fact when your system panics about missing of **T**, because it might be **&T**
    /// existing in the world.
    fn into_double_buf(self) -> DoubleBuffer<Self::Item>;
}

impl<T> IntoDoubleBuffer for &T
where
    T: Clone,
{
    type Item = T;
    fn into_double_buf(self) -> DoubleBuffer<Self::Item> {
        DoubleBuffer::new(self.to_owned())
    }
}
