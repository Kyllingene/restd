use super::{Format, Display, Style, Modifier, Write, Result};

pub struct Prefix<P, S>(pub P, pub S);

// TODO: perhaps make this more generic
impl<P: Format<Display>, S: Style> Style for Prefix<P, S> {}
impl<P: Format<Display>, S: Style> Modifier for Prefix<P, S> {
    type Inner = S;

    fn apply<T>(&self, f: &mut dyn Write, data: &T) -> Result
    where
        T: Format<Self::Inner> + ?Sized
    {
        self.0.fmt(f, &Display)?;
        data.fmt(f, &self.1)?;
        Ok(())
    }
}
