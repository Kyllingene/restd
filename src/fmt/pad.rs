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
super::derive!(enum Dir { Left, Center, Right });

/// The style of padding.
#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    /// Pad until the data is at least `count` characters wide.
    #[default]
    Full,

    /// Pad until the data is a multiple of `count` characters wide.
    Mod,
}
super::derive!(enum Kind { Full, Mod });

// TODO: should this support arbitrary str padding?
/// Pad any data with a given character until it reaches a certain width.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pad<S> {
    /// The direction to place the text in when padding.
    pub align: Dir,

    /// The character to pad with.
    pub with: char,

    /// The width to target when padding.
    pub count: usize,

    /// The kind of padding.
    pub kind: Kind,

    /// The style being wrapped by this modifier.
    pub style: S,
}
super::derive!(struct Pad<S!> { align, with, count, kind, style });

impl<S: Style> Pad<S> {
    /// Pad left.
    ///
    /// Shorthand for `Pad { dir: Dir::Left, with, count, kind: Full, style }`
    pub const fn left(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Left,
            with,
            count,
            kind: Kind::Full,
            style,
        }
    }

    /// Pad to the center.
    ///
    /// Shorthand for `Pad { dir: Dir::Center, with, count, kind: Full, style }`
    pub const fn center(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Center,
            with,
            count,
            kind: Kind::Full,
            style,
        }
    }

    /// Pad right.
    ///
    /// Shorthand for `Pad { dir: Dir::Right, with, count, kind: Full, style }`
    pub const fn right(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Right,
            with,
            count,
            kind: Kind::Full,
            style,
        }
    }

    /// Pad left using [modulo padding](Kind::Mod).
    ///
    /// Shorthand for `Pad { dir: Dir::Left, with, count, kind: Mod, style }`
    pub const fn left_mod(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Left,
            with,
            count,
            kind: Kind::Mod,
            style,
        }
    }

    /// Pad to the center using [modulo padding](Kind::Mod).
    ///
    /// Shorthand for `Pad { dir: Dir::Center, with, count, kind: Mod, style }`
    pub const fn center_mod(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Center,
            with,
            count,
            kind: Kind::Mod,
            style,
        }
    }

    /// Pad right using [modulo padding](Kind::Mod).
    ///
    /// Shorthand for `Pad { dir: Dir::Right, with, count, kind: Mod, style }`
    pub const fn right_mod(with: char, count: usize, style: S) -> Self {
        Self {
            align: Dir::Right,
            with,
            count,
            kind: Kind::Mod,
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

        let chs = match self.kind {
            Kind::Full => self.count.saturating_sub(counter.0),
            Kind::Mod => self.count - counter.0.checked_rem(self.count).unwrap_or(0),
        };

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
