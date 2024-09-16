use super::Prefix;

/// A repr for formatting data in hexadecimal. Implemented for all integers.
///
/// The inner `bool` specifies whether or not output should be uppercase.
///
/// Unlike [`Debug`](super::Debug) and [`Pretty`](super::Pretty), cannot be
/// derived, since most types cannot be intuitively formatted in hex.
///
/// Equivalent of [`core::fmt::Hex`].
#[derive(Default, Clone, Copy)]
pub struct Hex(pub bool);
impl super::Style for Hex {}
super::derive!(struct Hex(uppercase));

impl Hex {
    /// Create a new `Hex`, with the prefix `0x` automatically applied.
    pub fn prefix(uppercase: bool) -> Prefix<&'static str, Self> {
        Prefix("0x", Self(uppercase))
    }
}
