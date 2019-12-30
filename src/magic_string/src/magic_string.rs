use crate::chunk::Chunk;
use crate::error::Error;
use crate::MagicBuffer;

pub struct MagicString<'a>(MagicBuffer<'a>);

impl<'a> MagicString<'a> {
    pub fn new(content: &'a str) -> MagicString<'a> {
        MagicString(MagicBuffer::new(content.as_bytes()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_string(&self) -> Result<String, Error> {
        return String::from_utf8(self.0.to_bytes()?).map_err(|_| Error::InvalidInternalState);
    }

    fn get_byte_index(&self, index: usize) -> Option<usize> {
        Some(index)
    }

    #[inline]
    fn do_insert(
        &mut self,
        index: usize,
        content: &str,
        left: bool,
        append: bool,
        essential: bool,
    ) -> Result<(), Error> {
        let index = self
            .get_byte_index(index)
            .ok_or(Error::IndexOutOfBoundError(index))?;

        let chunk = if left {
            self.0.chunks.slice(index)?.1
        } else {
            self.0.chunks.slice(index)?.0
        };
        if append {
            chunk.append(content.as_bytes(), essential)
        } else {
            chunk.prepend(content.as_bytes(), essential)
        }
    }

    pub fn append_left(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.do_insert(index, content, true, true, false)
    }
    pub fn prepend_left(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.do_insert(index, content, true, false, false)
    }
    pub fn append_right(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.do_insert(index, content, false, true, false)
    }
    pub fn prepend_right(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.do_insert(index, content, false, false, false)
    }
    pub fn prepend(&mut self, content: &str) -> Result<(), Error> {
        self.prepend_left(0, content)
    }
    pub fn append(&mut self, content: &str) -> Result<(), Error> {
        self.append_right(self.len(), content)
    }

    pub fn overwrite(&mut self, start: usize, end: usize, content: &str) -> Result<(), Error> {
        self.remove(start, end)?;
        self.append_right(start, content)
    }

    pub fn remove(&mut self, start: usize, end: usize) -> Result<(), Error> {
        unsafe {
            let first = self.0.chunks.slice(start)?.1 as *mut Chunk;
            let last = self.0.chunks.slice(end)?.1 as *mut Chunk;

            // First, validate that we're doing something that will not assert.
            for it in self.0.chunks.iter() {
                let it = it as *const Chunk;
                (*it).check(it != first, it != last, it == first)?;
            }

            let mut it = self.0.chunks.iter_mut();
            while let Some(c) = it.next() {
                let c = c as *mut Chunk;
                if c == last {
                    break;
                }
                (*c).remove(c != first, c != last, c == first)?;
            }

            if let Some(c) = it.next() {
                c.remove(true, false, false)?;
            }
        }

        Ok(())
    }
}
