//! # Straightforward double-buffering implementation for [bevy engine](https://bevyengine.org/)
//!
//! Consists of main struct [DoubleBuffer], helper auto trait [IntoDoubleBuffer] and two aliases
//! [DoubleRes] and [DoubleResMut]
//!
//! *Crate was developed by solving author's personal problems so you can expect some bugs*


#![warn(missing_docs)]

pub mod double_buffer;
pub mod into_double_buffer;

pub use double_buffer::*;
pub use into_double_buffer::*;

use bevy_ecs::prelude::*;

/// Alias for [Res] of [DoubleBuffer<T>]
///
/// # Example
///
/// ```
/// use bevy_double_res::DoubleRes;
/// fn circular_dependent_system(double_buffer: DoubleRes<(u8, u8, u8)>) {
///     // ...
/// }
/// ```
///
/// # Same example without sugar
///
/// ```
/// use bevy_ecs::prelude::Res;
/// use bevy_double_res::DoubleBuffer;
/// fn circular_dependent_system(double_buffer: Res<DoubleBuffer<(u8, u8, u8)>>) {
///     // ...
/// }
/// ```
pub type DoubleRes<'w, T> = Res<'w, DoubleBuffer<T>>;

/// Alias for [ResMut] of [DoubleBuffer<T>]
///
/// # Example
///
/// ```
/// use bevy_double_res::DoubleResMut;
/// fn circular_dependent_system(mut double_buffer: DoubleResMut<(u8, u8, u8)>) {
///     // ...
/// }
/// ```
///
/// # Same example without sugar
///
/// ```
/// use bevy_ecs::prelude::ResMut;
/// use bevy_double_res::DoubleBuffer;
/// fn circular_dependent_system(mut double_buffer: ResMut<DoubleBuffer<(u8, u8, u8)>>) {
///     // ...
/// }
/// ```
pub type DoubleResMut<'w, T> = ResMut<'w, DoubleBuffer<T>>;

pub mod prelude {
    //! Provides all crate items

    pub use super::{DoubleBuffer, DoubleRes, DoubleResMut, IntoDoubleBuffer};
}
