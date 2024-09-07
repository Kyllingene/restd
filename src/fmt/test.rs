#![allow(warnings)]
use super::{Debug, Display, Pad, Dir};
use crate::format;

// #[test]
// fn normal_str() {
//     let display = format!("Hello, World!");
//     assert_eq!(display, "Hello, World!");

//     let debug = format!("Hello, World!" => Debug);
//     assert_eq!(debug, "\"Hello, World!\"");
// }

// #[test]
// fn format_multiple_args() {
//     let multiple = format!(() => Debug, " foobar ", 'T');
//     assert_eq!(multiple, "() foobar T");
// }

#[test]
fn manual_args() {
    use crate::fmt::args::*;

    let x = 'x';
    let y = "foobar".to_string();

    let slice = [
        Var::new(&"hello, world!\n", &Display),
        Var::new(&x, &Display),
        Var::new(&" ", &Display),
        Var::new(&y, &Debug),
    ];
    let args = Arguments(&slice[..]);

    let mut f = String::new();
    args.write(&mut f).unwrap();

    assert_eq!(f, "hello, world!\nx \"foobar\"");
}

#[test]
fn manual_pad() {
    use crate::fmt::args::*;

    let rpad = Pad {
        align: Dir::Right,
        ch: ' ',
        count: 12,
        style: Display,
    };
    let lpad = Pad {
        align: Dir::Left,
        ch: ' ',
        count: 12,
        style: Display,
    };
    let slice = [
        Var::new(&"foobar", &rpad),
        Var::new(&'\n', &Display),
        Var::new(&"longer-string", &lpad),
    ];
    let args = Arguments(&slice[..]);

    let mut f = String::new();
    args.write(&mut f).unwrap();

    assert_eq!(f, "      foobar\nlonger-string");
}
