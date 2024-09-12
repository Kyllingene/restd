use super::{Debug, Dir, Display, Format, Hex, Pad, Prefix, Pretty, Result, Write};
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
        { &y.as_str()[3..] },
    );

    assert_eq!(f, "x \"foobar\" 000123456 579 bar");
}

#[test]
fn integer() {
    let x = 123_u8;
    let y = -456_isize;

    let mut f = String::new();
    x.fmt(&mut f, &Display).unwrap();
    f.push('\n');
    y.fmt(&mut f, &Debug).unwrap();

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

#[test]
fn debug_helpers() {
    struct Foo(u32, &'static str);
    struct Bar {
        foo: Foo,
        bar: [char; 3],
    }

    impl Format<Debug> for Foo {
        fn fmt(&self, f: &mut dyn Write, s: &Debug) -> Result {
            s.dbg_tuple(f, "Foo").field(&self.0).field(&self.1).finish()
        }
    }

    impl Format<Debug> for Bar {
        fn fmt(&self, f: &mut dyn Write, s: &Debug) -> Result {
            s.dbg_struct(f, "Bar")
                .field("foo", &self.foo)
                .field("bar", &self.bar)
                .finish()
        }
    }

    let foo = Foo(123, "foo");
    let bar = Bar {
        foo,
        bar: ['b', 'a', 'r'],
    };

    let f = bar.stringify(&Debug);
    assert_eq!(f, r#"Bar { foo: Foo(123, "foo"), bar: ['b', 'a', 'r'] }"#);
}

#[test]
fn pretty_helpers() {
    struct Foo(u32, &'static str);
    struct Bar {
        foo: Foo,
        bar: [char; 3],
    }

    impl Format<Pretty> for Foo {
        fn fmt(&self, f: &mut dyn Write, s: &Pretty) -> Result {
            s.dbg_tuple(f, "Foo").field(&self.0).field(&self.1).finish()
        }
    }

    impl Format<Pretty> for Bar {
        fn fmt(&self, f: &mut dyn Write, s: &Pretty) -> Result {
            s.dbg_struct(f, "Bar")
                .field("foo", &self.foo)
                .field("bar", &self.bar)
                .finish()
        }
    }

    let foo = Foo(123, "foo");
    let bar = Bar {
        foo,
        bar: ['b', 'a', 'r'],
    };

    let f = bar.stringify(&Pretty(0));
    let ex = r#"Bar {
    foo: Foo(
        123,
        "foo",
    ),
    bar: [
        'b',
        'a',
        'r',
    ],
}"#;
    assert_eq!(f, ex);
}

#[test]
fn hex() {
    let x = 0x1a23_u32;
    let y = 0x0123456789abcdef_u64;

    let f = format!(x as Hex(false), ' ', y as Hex(true));
    assert_eq!(f, "1a23 123456789ABCDEF");
}

#[test]
fn floats() {
    let x = 123.456_f32;
    let y = 12345678.87654321_f64;

    assert_eq!(format!(x, ' ', y), std::format!("{x} {y}"),);
}

#[test]
fn derive() {
    use crate::derive;

    struct Foo(u32, &'static str);
    derive!(struct Foo(x, y as Debug,));

    let f = Foo(123, "foo").stringify(&Debug);
    assert_eq!(f, r#"Foo(123, "foo")"#);

    #[allow(dead_code)]
    struct Bar {
        x: u32,
        y: u32,
        z: bool,
    }
    derive!(struct Bar {
        x as Prefix("0x", Hex(false)),
        y as Hex::prefix(true),
        ...
    });

    let f = Bar {
        x: 0xabc,
        y: 0xDEF,
        z: false,
    }
    .stringify(&Debug);
    assert_eq!(f, "Bar { x: 0xabc, y: 0xDEF, ... }");

    #[allow(dead_code)]
    enum Baz {
        A(u32),
        B,
        C { x: f32, y: () },
    }
    derive!(enum Baz {
        A(x as Hex(false),),
        B,
        C { x, ... },
    });

    let f = Baz::A(0x1a2b).stringify(&Debug);
    assert_eq!(f, "A(1a2b)");

    let f = Baz::B.stringify(&Debug);
    assert_eq!(f, "B");

    let f = Baz::C { x: 12.34, y: () }.stringify(&Debug);
    assert_eq!(f, "C { x: 12.34, ... }");

    struct Qux<T, Q> {
        x: T,
        y: Q,
    }
    derive!(struct Qux<T!, Q> where [Q: Format<Display>] {
        x,
        y as Display,
    });

    let f = Qux {
        x: [1.23, 4.56, 7.89],
        y: 'y',
    }
    .stringify(&Pretty(0));
    let ex = r#"Qux {
    x: [
        1.23,
        4.56,
        7.89,
    ],
    y: y,
}"#;
    assert_eq!(f, ex);
}
