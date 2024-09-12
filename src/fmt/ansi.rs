use super::{Display, Format, Modifier, Result, Style, Write};
use crate::write;

pub const RESET: &str = "\x1b[0m";

basic! {
    Bold: 1,
    Faint: 2,
    Italic: 3,
    Underline: 4,
    Strike: 9,
}

pub struct Fg<S>(pub u8, pub S);

impl<S: Style> Style for Fg<S> {}
impl<S: Style> Modifier for Fg<S> {
    type Inner = S;

    fn apply<T>(&self, f: &mut dyn Write, data: &T) -> Result
    where
        T: Format<S> + ?Sized,
    {
        f.write_str("\x1b[38;5;")?;
        self.0.fmt(f, &Display)?;
        f.write_char('m')?;

        data.fmt(f, &self.1)?;

        f.write_str(RESET)?;
        Ok(())
    }
}

pub struct Bg<S>(pub u8, pub S);

impl<S: Style> Style for Bg<S> {}
impl<S: Style> Modifier for Bg<S> {
    type Inner = S;

    fn apply<T>(&self, f: &mut dyn Write, data: &T) -> Result
    where
        T: Format<S> + ?Sized,
    {
        f.write_str("\x1b[48;5;")?;
        self.0.fmt(f, &Display)?;
        f.write_char('m')?;

        data.fmt(f, &self.1)?;

        f.write_str(RESET)?;
        Ok(())
    }
}

pub struct FgRgb<S>(pub u8, pub u8, pub u8, pub S);

impl<S: Style> Style for FgRgb<S> {}
impl<S: Style> Modifier for FgRgb<S> {
    type Inner = S;

    fn apply<T>(&self, mut f: &mut dyn Write, data: &T) -> Result
    where
        T: Format<S> + ?Sized,
    {
        write!(&mut f, "\x1b[38;2;", self.0, ';', self.1, ';', self.2, 'm' )?;
        data.fmt(f, &self.3)?;

        f.write_str(RESET)?;
        Ok(())
    }
}

pub struct BgRgb<S>(pub u8, pub u8, pub u8, pub S);

impl<S: Style> Style for BgRgb<S> {}
impl<S: Style> Modifier for BgRgb<S> {
    type Inner = S;

    fn apply<T>(&self, mut f: &mut dyn Write, data: &T) -> Result
    where
        T: Format<S> + ?Sized,
    {
        write!(&mut f, "\x1b[48;2;", self.0, ';', self.1, ';', self.2, 'm' )?;
        data.fmt(f, &self.3)?;

        f.write_str(RESET)?;
        Ok(())
    }
}

macro_rules! basic {
    ($($name:ident: $escape:literal),+ $(,)?) => {$(
        pub struct $name<S>(pub S);

        impl<S: Style> Style for $name<S> {}
        impl<S: Style> Modifier for $name<S> {
            type Inner = S;

            fn apply<T>(&self, f: &mut dyn Write, data: &T) -> Result
            where
                T: Format<S> + ?Sized,
            {
                f.write_str(concat!(
                    "\x1b[",
                    $escape,
                    "m",
                ))?;
                data.fmt(f, &self.0)?;
                f.write_str(RESET)?;
                Ok(())
            }
        }
    )+};
}
use basic;
