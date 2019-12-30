#![cfg(test)]
use crate::chunk_list::ChunkList;

#[test]
fn basic() -> Result<(), crate::error::Error> {
    let content: Vec<u8> = vec![1, 2, 3, 4];
    let mut cl = ChunkList::new(&content);

    if let Some(c) = cl.peek() {
        assert_eq!(c.start, 0);
        assert_eq!(c.end, 4);
        assert_eq!(c.to_bytes(), content);
    } else {
        assert!(false);
    }

    let (c1, c2) = cl.slice(2)?;
    assert_eq!(c1.to_bytes().as_slice(), &content[0..2]);
    assert_eq!(c2.to_bytes().as_slice(), &content[2..]);

    Ok(())
}

#[test]
fn append() -> Result<(), crate::error::Error> {
    let content: Vec<u8> = vec![1, 2, 3, 4];
    let mut cl = ChunkList::new(&content);

    if let Some(ref mut c) = cl.peek_mut().take() {
        c.append(&[5, 6, 7, 8], false);
        assert_eq!(c.to_bytes().as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8]);
    }

    let _ = cl.slice(2)?;
    let mut it = cl.iter();
    assert_eq!(it.next().map(|x| x.to_bytes()), Some(vec![1, 2]));
    assert_eq!(
        it.next().map(|x| x.to_bytes()),
        Some(vec![3, 4, 5, 6, 7, 8,])
    );
    assert_eq!(it.next().map(|x| x.to_bytes()), None);

    Ok(())
}

#[test]
fn prepend() -> Result<(), crate::error::Error> {
    let content: Vec<u8> = vec![1, 2, 3, 4];
    let mut cl = ChunkList::new(&content);

    if let Some(ref mut c) = cl.peek_mut().take() {
        c.prepend(&[5, 6, 7, 8], false);
        assert_eq!(c.to_bytes().as_slice(), &[5, 6, 7, 8, 1, 2, 3, 4]);
    }

    let _ = cl.slice(2)?;
    let mut it = cl.iter();
    assert_eq!(
        it.next().map(|x| x.to_bytes()),
        Some(vec![5, 6, 7, 8, 1, 2])
    );
    assert_eq!(it.next().map(|x| x.to_bytes()), Some(vec![3, 4]));
    assert_eq!(it.next().map(|x| x.to_bytes()), None);

    Ok(())
}

#[test]
fn get_chunk_at() -> Result<(), crate::error::Error> {
    let content: Vec<u8> = vec![1, 2, 3, 4];
    let mut cl = ChunkList::new(&content);

    let _ = cl.slice(2);
    assert_eq!(cl.get_chunk_at(0).map(|x| x.to_bytes()), Some(vec![1, 2]));
    assert_eq!(cl.get_chunk_at(1).map(|x| x.to_bytes()), Some(vec![1, 2]));
    assert_eq!(cl.get_chunk_at(2).map(|x| x.to_bytes()), Some(vec![3, 4]));
    assert_eq!(cl.get_chunk_at(3).map(|x| x.to_bytes()), Some(vec![3, 4]));
    assert_eq!(cl.get_chunk_at(4).map(|x| x.to_bytes()), None);

    Ok(())
}
