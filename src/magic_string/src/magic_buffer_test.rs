#![cfg(test)]

use crate::error::Error;
use crate::MagicBuffer;

#[test]
fn basic() -> Result<(), Error> {
    let content = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut buffer = MagicBuffer::new(&content);

    buffer.insert_left(3, &[10]);
    assert_eq!(buffer.to_bytes()?, vec![0, 1, 2, 10, 3, 4, 5, 6, 7, 8]);

    buffer.insert_right(3, &[11]);
    assert_eq!(buffer.to_bytes()?, vec![0, 1, 2, 10, 11, 3, 4, 5, 6, 7, 8]);

    //    buffer.insert_right(4, &[12]);
    //    assert_eq!(
    //        buffer.to_bytes()?,
    //        vec![0, 1, 2, 10, 11, 3, 4, 12, 5, 6, 7, 8]
    //    );

    Ok(())
}
