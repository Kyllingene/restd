use super::Write;

pub struct Arguments {
    segments: &'static [Segment],
}

pub enum Segment {
    Str(&'static str),
    Var(fn(&mut dyn Write)),
}
