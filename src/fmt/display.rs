/// A repr for formatting data in a user-friendly style.
///
/// Unlike [`Debug`](super::Debug) and [`Pretty`](super::Pretty), cannot be
/// derived, since what is "user-friendly" output varies from type to type.
///
/// Equivalent of [`core::fmt::Display`].
#[derive(Default, Clone, Copy)]
pub struct Display;
impl super::Style for Display {}
super::derive!(struct Display);
