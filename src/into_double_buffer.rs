use crate::DoubleBuffer;

pub trait IntoDoubleBuffer: Clone
where
    Self: Sized,
{
    type Item;
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
