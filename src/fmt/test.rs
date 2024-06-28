use super::Debug;
use crate::format;

#[test]
fn normal_str() {
    let display = format!("Hello, World!");
    assert_eq!(display, "Hello, World!");

    let debug = format!("Hello, World!" => Debug);
    assert_eq!(debug, "\"Hello, World!\"");
}

#[test]
fn format_multiple_args() {
    let multiple = format!(() => Debug, " foobar ", 'T');
    assert_eq!(multiple, "() foobar T");
}
