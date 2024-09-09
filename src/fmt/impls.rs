use super::{Debug, Display, Format, Result, Style, Write};

crate::stylable![
    (),
    str,
    char,
];
crate::stylable!(for(T: Format<Debug>) [T]);

fn dbg_char(ch: char, f: &mut dyn Write) -> Result {
    if ch == '\'' {
        f.write_str("\\'")?;
    } else if ch.is_control() {
        todo!()
    } else {
        f.write_char(ch)?;
    }

    Ok(())
}

impl<T, S> Format<S> for &'_ T
where
    T: Format<S> + ?Sized,
    S: Style,
{
    fn fmt(&self, f: &mut dyn Write, s: &S) -> Result {
        (*self).fmt(f, s)
    }
}

impl Format<Debug> for () {
    fn fmt(&self, f: &mut dyn Write, _: &Debug) -> Result {
        f.write_str("()")
    }
}

impl Format<Debug> for str {
    fn fmt(&self, f: &mut dyn Write, _: &Debug) -> Result {
        f.write_char('"')?;
        for ch in self.chars() {
            if ch == '"' {
                f.write_str(r#"\"#)?;
            } else {
                dbg_char(ch, f)?;
            }
        }
        f.write_char('"')
    }
}

impl Format<Display> for str {
    fn fmt(&self, f: &mut dyn Write, _: &Display) -> Result {
        f.write_str(self)
    }
}

impl Format<Debug> for char {
    fn fmt(&self, f: &mut dyn Write, _: &Debug) -> Result {
        f.write_char('\'')?;
        dbg_char(*self, f)?;
        f.write_char('\'')?;

        Ok(())
    }
}

impl Format<Display> for char {
    fn fmt(&self, f: &mut dyn Write, _: &Display) -> Result {
        f.write_char(*self)
    }
}

macro_rules! impl_int {
    ($( $t:ident ),*) => {$(
        $crate::stylable!($t);

        impl Format<Debug> for $t {
            fn fmt(&self, f: &mut dyn Write, _: &Debug) -> Result {
                // this will get optimized out for unsigned anyways
                #[allow(unused_comparisons)]
                let mut x = if *self < 0 {
                    f.write_char('-')?;

                    // to get around unsigned integers having no Neg
                    (!*self) + 1
                } else {
                    *self
                };

                let mut digits = [0_u8; 40];
                let mut i = 0;

                digits[i] = (x % 10) as u8;
                x /= 10;
                i += 1;

                while x > 0 {
                    digits[i] = (x % 10) as u8;
                    x /= 10;
                    i += 1;
                }

                while i > 0 {
                    i -= 1;
                    let ch = (b'0' + digits[i]) as char;
                    f.write_char(ch)?;
                }

                Ok(())
            }
        }

        impl Format<Display> for $t {
            fn fmt(&self, f: &mut dyn Write, _: &Display) -> Result {
                <$t as Format<Debug>>::fmt(self, f, &Debug)
            }
        }
    )*};
}
impl_int![
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize
];

impl<T> Format<Debug> for [T]
where
    T: Format<Debug>,
{
    fn fmt(&self, f: &mut dyn Write, s: &Debug) -> Result {
        f.write_char('[')?;

        if !self.is_empty() {
            let (lead, last) = self.split_at(self.len() - 1);

            for x in lead {
                x.fmt(f, s)?;
                f.write_str(", ")?;
            }

            last[0].fmt(f, s)?;
        }

        f.write_char(']')
    }
}

#[cfg(any(test, feature = "std"))]
mod with_std {
    use crate::fmt::{Debug, Display, Format, Write, Result};

    crate::stylable![String];
    crate::stylable!(for(T: Format<Debug>) Vec<T>);

    impl Format<Debug> for String {
        fn fmt(&self, f: &mut dyn Write, s: &Debug) -> Result {
            self.as_str().fmt(f, s)
        }
    }

    impl Format<Display> for String {
        fn fmt(&self, f: &mut dyn Write, s: &Display) -> Result {
            self.as_str().fmt(f, s)
        }
    }

    impl<T> Format<Debug> for Vec<T>
    where
        T: Format<Debug>,
    {
        fn fmt(&self, f: &mut dyn Write, s: &Debug) -> Result {
            self.as_slice().fmt(f, s)
        }
    }
}
