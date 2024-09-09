use super::{Debug, Display, Pad, Dir, Format};
use crate::format;

#[test]
fn format() {
    let x = 'x';
    let y = "foobar".to_string();
    let z = 123456_u32;

    let f = format!(
        x,

        " ",

        y as Debug,

        ' ',

        z as Pad::right(
            '0',
            9,
            Display,
        ),

        r##" "##,

        { 123 + 456 },
        { 0x20 as char },
        { &y.as_str()[3..] }
    );

    assert_eq!(f, "x \"foobar\" 000123456 579 bar");
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
