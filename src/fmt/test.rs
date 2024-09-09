#![allow(warnings)]
use super::{Debug, Display, Pad, Dir, Format};
use crate::format_args;

#[test]
fn format_args() {
    let x = 'x';
    let y = "foobar".to_string();
    let z = 123456_u32;

    let args = format_args!(
        x,

        " ",

        y as Debug,

        ' ',

        z as Pad {
            align: Dir::Right,
            with: '0',
            count: 9,
            style: Display,
        },

        r##" "##,

        { 123 }
    );

    let mut f = String::new();
    args.write(&mut f).unwrap();

    assert_eq!(f, "x \"foobar\" 000123456 123");
}

#[test]
fn integer() {
    let x = 123_u8;
    let y = -456_isize;

    let mut f = String::new();
    <u8 as Format<Display>>::fmt(&x, &mut f, &Display).unwrap();
    f.push('\n');
    <isize as Format<Debug>>::fmt(&y, &mut f, &Debug).unwrap();

    assert_eq!(f, "123\n-456");
}

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

    let args = Arguments([
        Var::new(&"hello, world!\n", &Display),
        Var::new(&x, &Display),
        Var::new(&" ", &Display),
        Var::new(&y, &Debug),
    ]);

    let mut f = String::new();
    args.write(&mut f).unwrap();

    assert_eq!(f, "hello, world!\nx \"foobar\"");
}

#[test]
fn manual_pad() {
    use crate::fmt::args::*;

    let rpad = Pad {
        align: Dir::Right,
        with: ' ',
        count: 12,
        style: Display,
    };
    let lpad = Pad {
        align: Dir::Left,
        with: ' ',
        count: 12,
        style: Display,
    };

    let args = Arguments([
        Var::new(&"foobar", &rpad),
        Var::new(&'\n', &Display),
        Var::new(&"longer-string", &lpad),
    ]);

    let mut f = String::new();
    args.write(&mut f).unwrap();

    assert_eq!(f, "      foobar\nlonger-string");
}
