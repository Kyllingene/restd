use super::{Debug, Display};
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
fn manual_args_with_args() {
    use crate::fmt::args::*;

    todo!()
}
