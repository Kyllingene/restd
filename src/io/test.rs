use super::{Buffer, Cursor, Read, Seek, Write};

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

#[test]
fn cursor() {
    let mut cur = Cursor::new([0; 8]);

    cur.write_all(&[1, 2, 3]).unwrap();
    cur.write_all(&[4, 5, 6]).unwrap();

    cur.rewind().unwrap();
    assert_eq!(cur.remaining(), [1, 2, 3, 4, 5, 6, 0, 0]);

    let mut sub = [0; 3];
    cur.read_exact(&mut sub).unwrap();
    assert_eq!(sub, [1, 2, 3]);
    assert_eq!(cur.remaining(), [4, 5, 6, 0, 0]);
}
