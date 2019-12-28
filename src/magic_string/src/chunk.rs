use crate::error::Error;
use std::ptr::NonNull;

pub(crate) struct ChunkListIterator<'a> {
    current: Option<&'a Chunk<'a>>,
}

impl<'a> ChunkListIterator<'a> {
    pub fn new(b: &'a Box<Chunk<'a>>) -> ChunkListIterator<'a> {
        Self {
            current: Some(b.as_ref()),
        }
    }
}

impl<'a> Iterator for ChunkListIterator<'a> {
    type Item = &'a Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.and_then(|chunk| unsafe {
            let next = (*chunk).next.map(|x| x.as_ptr());
            self.current = next.map(|x| &*x);
            self.current
        })
    }
}

struct ChunkIterator<'a> {
    chunk: &'a Chunk<'a>,
    which: u8,
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.which == 0 {
            self.which += 1;
            if let Some(ref v) = self.chunk.left {
                return Some(v.as_slice());
            }
        }
        if self.which == 1 {
            self.which += 1;
            if let Some(ref v) = self.chunk.content {
                return Some(v);
            }
        }
        if self.which == 2 {
            self.which += 1;
            if let Some(ref v) = self.chunk.right {
                return Some(v.as_slice());
            }
        }

        None
    }
}

/// Chunks are parts of a memory that have an intro and an outro.
/// They are chunks of bytes, and not strings, as we export two types; a magic
/// string that deals with String content, and a magic buffer that deals with
/// binary data. Because both reuse the same chunk type (this one), this type
/// is storage agnostic.
pub(crate) struct Chunk<'a> {
    left: Option<Vec<u8>>,
    right: Option<Vec<u8>>,
    content: Option<&'a [u8]>,
    original_content: &'a [u8],
    assert_left: bool,
    assert_right: bool,
    pub start: usize,
    pub end: usize,

    pub next: Option<NonNull<Chunk<'a>>>,
}

impl<'a> Chunk<'a> {
    pub fn from_slice(original_content: &'a [u8]) -> Chunk<'a> {
        Chunk {
            left: Some(Vec::new()),
            content: Some(&original_content[..]),
            right: Some(Vec::new()),
            original_content: &original_content[..],
            assert_left: false,
            assert_right: false,
            start: 0,
            end: original_content.len(),
            next: None,
        }
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a [u8]> {
        ChunkIterator {
            chunk: self,
            which: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.iter().fold(0, |acc, i| acc + i.len())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        if let Some(ref l) = self.left {
            v.extend_from_slice(l);
        }
        if let Some(ref c) = self.content {
            v.extend_from_slice(c);
        }
        if let Some(ref r) = self.right {
            v.extend_from_slice(r);
        }
        v
    }

    pub unsafe fn slice(&'a mut self, start: usize) -> Result<&NonNull<Chunk<'a>>, Error> {
        if start > self.end || start < self.start {
            return Err(Error::IndexOutOfBoundError(start));
        }

        let new_chunk = Box::new(Chunk {
            left: Some(Vec::new()),
            right: self.right.take(),
            content: self.content.map(|c| &c[start..self.end]),
            original_content: self.original_content,
            assert_left: false,
            assert_right: self.assert_right,
            start,
            end: self.end,
            next: self.next.take(),
        });

        self.end = start;
        self.assert_right = false;
        let ptr = Box::into_raw(new_chunk);
        self.next = Some(NonNull::new_unchecked(ptr));

        Ok(self.next.as_ref().unwrap())
    }

    pub fn append(&mut self, content: &[u8], essential: bool) -> Result<(), Error> {
        match self.right {
            None => {
                if essential {
                    Err(Error::EssentialContentCannotBeAppended)
                } else {
                    Ok(())
                }
            }
            Some(ref mut r) => {
                r.extend_from_slice(content);
                self.assert_right = self.assert_right && essential;
                Ok(())
            }
        }
    }

    pub fn prepend(&mut self, content: &[u8], essential: bool) -> Result<(), Error> {
        match self.left {
            None => {
                if essential {
                    Err(Error::EssentialContentCannotBePrepended)
                } else {
                    Ok(())
                }
            }
            Some(ref l) => {
                let mut tmp = content.to_owned();
                tmp.extend_from_slice(l);
                self.left = Some(tmp);
                self.assert_left = self.assert_left && essential;
                Ok(())
            }
        }
    }

    pub fn check(&self, l: bool, _c: bool, r: bool) -> Result<(), Error> {
        if l && self.left == None && self.assert_left {
            Err(Error::ContentShouldNotBeRemoved)
        } else if r && self.right == None && self.assert_right {
            Err(Error::ContentShouldNotBeRemoved)
        } else {
            Ok(())
        }
    }

    pub fn remove(&mut self, left: bool, content: bool, right: bool) -> Result<(), Error> {
        if left {
            if self.assert_left {
                return Err(Error::ContentShouldNotBeRemoved);
            }
            self.left = None;
        }
        if content {
            self.content = None;
        }
        if right {
            if self.assert_right {
                return Err(Error::ContentShouldNotBeRemoved);
            }
            self.right = None;
        }
        Ok(())
    }

    pub unsafe fn find(&'a mut self, index: usize) -> Option<*mut Chunk<'a>> {
        if index < self.end {
            Some(self)
        } else {
            match self.next {
                None => None,
                Some(ref mut c) => c.as_mut().find(index),
            }
        }
    }
}

impl<'a> PartialEq for Chunk<'a> {
    fn eq(&self, other: &Chunk<'a>) -> bool {
        self.original_content == other.original_content
            && self.start == other.start
            && self.end == other.end
    }
}
