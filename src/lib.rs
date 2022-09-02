pub mod double_buffer;
pub mod into_double_buffer;

pub use double_buffer::*;
pub use into_double_buffer::*;

use bevy_ecs::prelude::*;

pub type DoubleRes<'w, T> = Res<'w, DoubleBuffer<T>>;
pub type DoubleResMut<'w, T> = ResMut<'w, DoubleBuffer<T>>;

pub mod prelude {
    pub use super::{DoubleBuffer, IntoDoubleBuffer, DoubleRes, DoubleResMut};
}
