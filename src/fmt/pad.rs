use super::{Format, Modifier, Result, Style, Write};
use crate::io::Counter;

/// The direction to place the text in when padding.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Left,
    Center,
    Right,
}
super::derive!(enum Dir {
    Left,
    Center,
    Right,
});

// TODO: should this support arbitrary str padding?
/// Pad any data with a given character until it reaches a certain width.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pad<S> {
    /// The direction to place the text in when padding.
    pub align: Dir,
    /// The character to pad with.
    pub with: char,
    /// The width to target when padding.
    ///
    /// If the data matches or exceeds this in length already, padding does
    /// nothing.
    pub count: usize,
    /// The style being wrapped by this modifier.
    pub style: S,
}
super::derive!(struct Pad<S!> { align, with, count, style });

impl<S: Style> Pad<S> {
    /// Pad left.
    ///
    /// Shorthand for `Pad { dir: Dir::Left, with, count, style }`
    pub const fn left(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Left,
            with,
            count,
            style,
        }
    }

    /// Pad to the center.
    ///
    /// Shorthand for `Pad { dir: Dir::Center, with, count, style }`
    pub const fn center(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Center,
            with,
            count,
            style,
        }
    }

    /// Pad right.
    ///
    /// Shorthand for `Pad { dir: Dir::Right, with, count, style }`
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
        let chs = self.count.saturating_sub(counter.0);

        match self.align {
            Dir::Left => {
                data.fmt(f, &self.style)?;
                for _ in 0..chs {
                    f.write_char(self.with)?;
                }
            }
            Dir::Center => {
                let before = chs / 2;
                let after = chs / 2 + (chs % 2);

                for _ in 0..before {
                    f.write_char(self.with)?;
                }
                data.fmt(f, &self.style)?;
                for _ in 0..after {
                    f.write_char(self.with)?;
                }
            }
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
