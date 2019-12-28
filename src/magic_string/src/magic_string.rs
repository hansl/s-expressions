use crate::chunk::{Chunk, ChunkListIterator};
use crate::error::Error;
use std::convert::identity;
use std::string::FromUtf8Error;

pub struct MagicString<'a> {
    original_content: &'a str,
    head: Box<Chunk<'a>>,
}

impl<'a> MagicString<'a> {
    pub fn new(content: &str) -> MagicString {
        let original_content = content;
        MagicString {
            original_content,
            head: Box::new(Chunk::from_slice(original_content.as_bytes())),
        }
    }

    /// Returns the position in byte of the character i.
    fn _position_of(&self, i: usize) -> usize {
        let mut index = 0;
        for chunk in self.iter() {
            if i < chunk.end {
                return index
                    + self.original_content[chunk.start..chunk.end]
                        .chars()
                        .take(i - chunk.start)
                        .count();
            }
            index += self.original_content[chunk.start..chunk.end]
                .chars()
                .count();
        }

        // We're out of bound, just return more (we always assert after using this function)
        self.original_content.len() + 1
    }

    unsafe fn _slice(
        &'a mut self,
        i: usize,
    ) -> Result<(*mut Chunk<'a>, *mut Chunk<'a>, &'a mut Self), Error> {
        let index = self._position_of(i);

        let chunk = self.head.find(index);
        match chunk {
            None => Err(Error::IndexOutOfBoundError(i)),
            Some(ch) => {
                if index == (*ch).end {
                    if let Some(ref mut n) = (*ch).next {
                        return Ok((n.as_ptr(), n.as_ptr(), self));
                    }
                }

                let next = (*ch).slice(index)?.as_ptr();
                Ok((ch, next, self))
            }
        }
    }

    pub(crate) fn iter(&'a self) -> impl Iterator<Item = &'a Chunk<'a>> {
        ChunkListIterator::new(&self.head)
    }

    pub fn len(&self) -> usize {
        self.iter().fold(0, |a, c| a + c.len())
    }

    pub fn to_string(&self) -> Result<String, FromUtf8Error> {
        let mut s = String::new();
        for it in self.iter() {
            s.push_str(&String::from_utf8(it.to_bytes())?);
        }
        Ok(s)
    }

    pub fn insert_left(&'a mut self, index: usize, content: &str) -> Result<(), Error> {
        unsafe {
            (*self._slice(index)?.0).append(content.as_bytes(), false)?;
        }
        Ok(())
    }

    pub fn insert_essential_left(&'a mut self, index: usize, content: &str) -> Result<(), Error> {
        unsafe {
            (*self._slice(index)?.0).append(content.as_bytes(), true)?;
        }
        Ok(())
    }

    pub fn insert_right(&'a mut self, index: usize, content: &str) -> Result<(), Error> {
        unsafe {
            (*self._slice(index)?.1).prepend(content.as_bytes(), false)?;
        }
        Ok(())
    }

    pub fn insert_essential_right(&'a mut self, index: usize, content: &str) -> Result<(), Error> {
        unsafe {
            (*self._slice(index)?.1).prepend(content.as_bytes(), true)?;
        }
        Ok(())
    }

    pub fn remove(&'a mut self, start: usize, end: usize) -> Result<(), Error> {
        unsafe {
            let (_, first, s) = self._slice(start)?;
            let (_, last, s) = s._slice(end)?;

            //            let first = { self._slice(start)? }.1;
            //            let last = { self._slice(end)? }.1;

            // First, validate that we're doing something that will not assert.

            //            for it in ChunkListIterator::new(&mut self.head) {
            //                it.check(it != &*first, it != &*last, it == &*first)?;
            //            }

            //
            //            let mut curr = Some(first);
            //            while let Some(c) = curr {
            //                c.remove(c != first, c != last, c == first)?;
            //
            //                curr = c.next.map(|x| &mut **x.as_ptr());
            //                if curr.map_or_else(|| false, |x| x.eq(&last)) {
            //                    break;
            //                }
            //            }
            //
            //            if let Some(c) = curr {
            //                c.remove(true, false, false)?;
            //            }
            Ok(())
        }
    }
}
