use super::{Display, Format, Modifier, Result, Style, Write};

/// Prints `P` before the data.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prefix<P, S>(pub P, pub S);
super::derive!(struct Prefix<P!, S!>(p, s));

// TODO: perhaps make this more generic
impl<P: Format<Display>, S: Style> Style for Prefix<P, S> {}
impl<P: Format<Display>, S: Style> Modifier for Prefix<P, S> {
    type Inner = S;

    fn apply<T>(&self, f: &mut dyn Write, data: &T) -> Result
    where
        T: Format<Self::Inner> + ?Sized,
    {
        self.0.fmt(f, &Display)?;
        data.fmt(f, &self.1)?;
        Ok(())
    }
}
