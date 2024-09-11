use super::Prefix;

#[derive(Default, Clone, Copy)]
pub struct Hex(pub bool);
impl super::Style for Hex {}

impl Hex {
    pub fn prefix(uppercase: bool) -> Prefix<&'static str, Self> {
        Prefix("0x", Self(uppercase))
    }
}
