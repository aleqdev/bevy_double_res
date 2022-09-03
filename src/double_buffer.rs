//! Straightforward double-buffering implementation
//!
//! Implementation uses two separate copies and current copy index

/// Enables double-buffering of your data by storing two separate copies and current copy index
///
/// # Example
///
/// ```
/// use bevy_double_res::{DoubleBuffer, IntoDoubleBuffer};
/// let mut tuple = (10, 20).into_double_buf();
///
/// assert_eq!(tuple.current(), &(10, 20));
/// assert_eq!(tuple.next(), &(10, 20));
///
/// tuple.apply(|current, next| {
///     next.0 = current.1;
///     next.1 = current.0;
/// });
/// tuple.swap();
///
/// assert_eq!(tuple.current(), &(20, 10));
/// assert_eq!(tuple.next(), &(10, 20));
///
/// tuple.apply(|current, next| {
///     next.0 = current.1;
///     next.1 = current.0;
/// });
/// tuple.swap();
///
/// assert_eq!(tuple.current(), &(10, 20));
/// assert_eq!(tuple.next(), &(20, 10));
/// ```
#[derive(Debug)]
pub struct DoubleBuffer<T>
where
    T: Sized,
{
    buffer: [T; 2],
    index: u8,
}

impl<T> DoubleBuffer<T> {
    /// Construct buffer manually using two copies and index
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// assert_eq!(tuple.current(), &(10, 20));
    /// ```
    pub fn from_buffer(buffer: [T; 2], index: u8) -> Self {
        Self { buffer, index }
    }

    /// Access underlying buffer for reading
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// assert_eq!(tuple.buffer(), &[(10, 20), (20, 10)]);
    /// ```
    pub fn buffer(&self) -> &[T; 2] {
        &self.buffer
    }

    /// Access underlying buffer for mutation
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// tuple.buffer_mut()[0] = (30, 40);
    ///
    /// assert_eq!(tuple.buffer(), &[(30, 40), (20, 10)]);
    /// ```
    pub fn buffer_mut(&mut self) -> &mut [T; 2] {
        &mut self.buffer
    }

    /// Access underlying current copy index
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::<()>::default();
    ///
    /// assert_eq!(tuple.index(), 0);
    ///
    /// tuple.swap();
    ///
    /// assert_eq!(tuple.index(), 1);
    ///
    /// tuple.swap();
    ///
    /// assert_eq!(tuple.index(), 0);
    /// ```
    ///
    /// Default value is always zero
    pub fn index(&self) -> u8 {
        self.index
    }

    /// Set underlying current copy index
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// tuple.set_index(1);
    ///
    /// assert_eq!(tuple.current(), &(20, 10));
    /// ```
    ///
    /// Setting index outside of range \[0, 1] and then getting the value is likely a panic
    pub fn set_index(&mut self, value: u8) {
        self.index = value;
    }

    /// Get readonly copy reference under current index
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// assert_eq!(tuple.current(), &(10, 20));
    ///
    /// tuple.swap();
    ///
    /// assert_eq!(tuple.current(), &(20, 10));
    /// ```
    pub fn current(&self) -> &T {
        &self.buffer[self.index as usize]
    }

    /// Get mutable copy reference under current index
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// tuple.current_mut().0 = 999;
    ///
    /// assert_eq!(tuple.current(), &(999, 20));
    ///
    /// tuple.swap();
    ///
    /// assert_eq!(tuple.current(), &(20, 10));
    /// ```
    pub fn current_mut(&mut self) -> &mut T {
        &mut self.buffer[self.index as usize]
    }

    /// Get readonly copy reference under opposite of current index
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// assert_eq!(tuple.next(), &(20, 10));
    ///
    /// tuple.swap();
    ///
    /// assert_eq!(tuple.current(), &(20, 10));
    /// assert_eq!(tuple.next(), &(10, 20));
    /// ```
    pub fn next(&self) -> &T {
        &self.buffer[1 - self.index as usize]
    }

    /// Get mutable copy reference under opposite of current index
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// tuple.next_mut().0 = 999;
    /// tuple.swap();
    ///
    /// assert_eq!(tuple.current(), &(999, 10));
    /// assert_eq!(tuple.next(), &(10, 20));
    /// ```
    pub fn next_mut(&mut self) -> &mut T {
        &mut self.buffer[1 - self.index as usize]
    }

    /// Toggles current index between **0** and **1**
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// assert_eq!(tuple.current(), &(10, 20));
    /// assert_eq!(tuple.index(), 0);
    ///
    /// tuple.swap();
    ///
    /// assert_eq!(tuple.current(), &(20, 10));
    /// assert_eq!(tuple.index(), 1);
    ///
    /// tuple.swap();
    ///
    /// assert_eq!(tuple.current(), &(10, 20));
    /// assert_eq!(tuple.index(), 0);
    /// ```
    pub fn swap(&mut self) {
        self.index = 1 - self.index;
    }

    /// Returns two readonly references to copies
    ///
    /// Order does **not** depend on current index!
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// assert_eq!(tuple.split(), (&(10, 20), &(20, 10)));
    ///
    /// tuple.swap();
    ///
    /// // Same as before!
    /// assert_eq!(tuple.split(), (&(10, 20), &(20, 10)));
    /// ```
    pub fn split(&self) -> (&T, &T) {
        let (first, second) = self.buffer.split_at(1);
        (&first[0], &second[0])
    }

    /// Returns two mutable references to copies
    ///
    /// Order does **not** depend on current index!
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (20, 10)], 0);
    ///
    /// assert_eq!(tuple.split_mut(), (&mut (10, 20), &mut (20, 10)));
    ///
    /// tuple.swap();
    ///
    /// // Same as before!
    /// assert_eq!(tuple.split_mut(), (&mut (10, 20), &mut (20, 10)));
    /// ```
    pub fn split_mut(&mut self) -> (&mut T, &mut T) {
        let (first, second) = self.buffer.split_at_mut(1);
        (&mut first[0], &mut second[0])
    }

    /// Returns two references to copies
    ///
    /// Order **does** depend on current index and is:
    /// 1. **current** - immutable reference
    /// 2. **next** - mutable reference
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (0, 0)], 0);
    ///
    /// let (current, next) = tuple.split_ordered();
    ///
    /// assert_eq!(current, &(10, 20));
    /// assert_eq!(next, &mut (0, 0));
    ///
    /// next.0 = current.1;
    /// next.1 = current.0;
    ///
    /// tuple.swap();
    ///
    /// let (current, next) = tuple.split_ordered();
    ///
    /// assert_eq!(current, &(20, 10));
    /// assert_eq!(next, &mut (10, 20));
    /// ```
    pub fn split_ordered(&mut self) -> (&T, &mut T) {
        if self.index == 0 {
            let (first, second) = self.split_mut();
            (&*first, second)
        } else {
            let (first, second) = self.split_mut();
            (&*second, first)
        }
    }

    /// Applies function to operate on current and next copies
    ///
    /// More idiomatic version of [`DoubleBuffer::split_ordered`]
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let mut tuple = DoubleBuffer::from_buffer([(10, 20), (0, 0)], 0);
    ///
    /// tuple.apply(|current, next| {
    ///     assert_eq!(current, &(10, 20));
    ///     assert_eq!(next, &mut (0, 0));
    ///
    ///     next.0 = current.1;
    ///     next.1 = current.0;
    /// });
    ///
    /// tuple.swap();
    ///
    /// let result = tuple.apply(|current, next| {
    ///     assert_eq!(current, &(20, 10));
    ///     assert_eq!(next, &mut (10, 20));
    ///
    ///     return "You can return values from here too";
    /// });
    ///
    /// assert_eq!(result, "You can return values from here too");
    /// ```
    pub fn apply<Res>(&mut self, f: impl FnOnce(&T, &mut T) -> Res) -> Res {
        let (prev, next) = self.split_ordered();
        f(prev, next)
    }
}

impl<T> DoubleBuffer<T>
where
    T: Clone,
{
    /// Create double buffer of **T** from one copy of **T**
    ///
    /// Second copy will be cloned and index will be **0**
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let tuple = DoubleBuffer::new((10, 20));
    ///
    /// assert_eq!(tuple.current(), &(10, 20));
    /// assert_eq!(tuple.next(), &(10, 20));
    /// assert_eq!(tuple.index(), 0);
    /// ```
    pub fn new(value: T) -> Self {
        Self::from_buffer([value.clone(), value], 0)
    }
}

impl<T> From<T> for DoubleBuffer<T>
where
    T: Clone,
{
    /// Create double buffer of **T** from one copy of **T** by conversion
    ///
    /// Second copy will be cloned and index will be **0**
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let tuple: DoubleBuffer<(u8, u8)> = (10, 20).into();
    ///
    /// assert_eq!(tuple.current(), &(10, 20));
    /// assert_eq!(tuple.next(), &(10, 20));
    /// assert_eq!(tuple.index(), 0);
    /// ```
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Default for DoubleBuffer<T>
where
    T: Default + Clone,
{
    /// Create double buffer of **T** with default values of **T**
    ///
    /// Index will be **0**
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_double_res::DoubleBuffer;
    /// let tuple = DoubleBuffer::<(u8, u8)>::default();
    ///
    /// assert_eq!(tuple.current(), &(0, 0));
    /// assert_eq!(tuple.next(), &(0, 0));
    /// assert_eq!(tuple.index(), 0);
    /// ```
    fn default() -> Self {
        Self::new(T::default())
    }
}
