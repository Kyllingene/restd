use super::{Format, Modifier, Result, Style, Write};
use crate::io::Counter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Left,
    Center,
    Right,
}

// TODO: should this support arbitrary str padding?
pub struct Pad<S> {
    pub align: Dir,
    pub with: char,
    pub count: usize,
    pub style: S,
}

impl<S: Style> Pad<S> {
    pub const fn left(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Left,
            with,
            count,
            style,
        }
    }

    pub const fn center(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Center,
            with,
            count,
            style,
        }
    }

    pub const fn right(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Right,
            with,
            count,
            style,
        }
    }
}

impl<S: Style> Style for Pad<S> {}

impl<S: Style> Modifier for Pad<S> {
    type Inner = S;

    fn apply<T>(&self, f: &mut dyn Write, data: &T) -> Result
    where
        T: Format<S> + ?Sized,
    {
        let mut counter = Counter::new();
        data.fmt(&mut counter, &self.style)?;
        let chs = self.count.saturating_sub(counter.count());

        match self.align {
            Dir::Left => {
                data.fmt(f, &self.style)?;
                for _ in 0..chs {
                    f.write_char(self.with)?;
                }
            }
            Dir::Center => todo!(),
            Dir::Right => {
                for _ in 0..chs {
                    f.write_char(self.with)?;
                }
                data.fmt(f, &self.style)?;
            }
        }

        Ok(())
    }
}
