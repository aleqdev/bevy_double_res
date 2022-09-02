#[derive(Debug)]
pub struct DoubleBuffer<T>
where
    T: Sized,
{
    buffer: [T; 2],
    index: usize,
}

impl<T> DoubleBuffer<T> {
    pub fn from_buffer(buffer: [T; 2], index: usize) -> Self {
        Self { buffer, index }
    }

    pub fn buffer(&self) -> &[T; 2] {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut [T; 2] {
        &mut self.buffer
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, value: usize) {
        self.index = value;
    }

    pub fn current(&self) -> &T {
        &self.buffer[self.index]
    }

    pub fn current_mut(&mut self) -> &mut T {
        &mut self.buffer[self.index]
    }

    pub fn next(&self) -> &T {
        &self.buffer[1 - self.index]
    }

    pub fn next_mut(&mut self) -> &mut T {
        &mut self.buffer[1 - self.index]
    }

    pub fn swap(&mut self) {
        self.index = 1 - self.index;
    }

    pub fn split(&self) -> (&T, &T) {
        let (first, second) = self.buffer.split_at(1);
        (&first[0], &second[0])
    }

    pub fn split_mut(&mut self) -> (&mut T, &mut T) {
        let (first, second) = self.buffer.split_at_mut(1);
        (&mut first[0], &mut second[0])
    }

    pub fn split_ordered(&mut self) -> (&T, &mut T) {
        if self.index == 0 {
            let (first, second) = self.split_mut();
            (&*first, second)
        } else {
            let (first, second) = self.split_mut();
            (&*second, first)
        }
    }

    pub fn apply<Res>(&mut self, f: impl FnOnce(&T, &mut T) -> Res) -> Res {
        let (prev, next) = self.split_ordered();
        f(prev, next)
    }
}

impl<T> DoubleBuffer<T>
where
    T: Clone,
{
    pub fn new(value: T) -> Self {
        Self::from_buffer([value.clone(), value], 0)
    }
}

impl<T> From<T> for DoubleBuffer<T>
where
    T: Clone,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Default for DoubleBuffer<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}
