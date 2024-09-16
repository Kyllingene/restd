use super::Prefix;

/// A repr for formatting data in binary. Implemented for all integers.
///
/// Unlike [`Debug`](super::Debug) and [`Pretty`](super::Pretty), cannot be
/// derived, since most types cannot be intuitively formatted in binary.
///
/// Equivalent of [`core::fmt::Binary`].
#[derive(Default, Clone, Copy)]
pub struct Binary;
impl super::Style for Binary {}
super::derive!(struct Binary);

impl Binary {
    /// Create a new `Binary`, with the prefix `0b` automatically applied.
    pub fn prefix() -> Prefix<&'static str, Self> {
        Prefix("0b", Self)
    }
}
