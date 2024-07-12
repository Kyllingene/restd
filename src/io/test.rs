use super::{Buffer, Read, Write};

#[test]
fn buffer() {
    let mut buf = Buffer::new([0; 8]);

    buf.write_all(&[1, 2, 3]).unwrap();
    assert_eq!(buf.data(), [1, 2, 3]);

    buf.write_all(&[4, 5, 6]).unwrap();
    assert_eq!(buf.data(), [1, 2, 3, 4, 5, 6]);

    let mut sub = [0; 3];
    buf.read_exact(&mut sub).unwrap();
    assert_eq!(sub, [1, 2, 3]);
    assert_eq!(buf.data(), [4, 5, 6]);
}
