use super::{Result, Style, Format, Write, Modifier};
use crate::io::Counter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Left,
    Center,
    Right,
}

pub struct Pad<S> {
    pub align: Dir,
    pub ch: char,
    pub count: usize,
    pub style: S,
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
                    f.write_char(self.ch)?;
                }
            }
            Dir::Center => todo!(),
            Dir::Right => {
                for _ in 0..chs {
                    f.write_char(self.ch)?;
                }
                data.fmt(f, &self.style)?;
            }
        }

        Ok(())
    }
}
