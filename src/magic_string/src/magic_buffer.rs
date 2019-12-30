use crate::chunk_list::ChunkList;
use crate::error::Error;

pub struct MagicBuffer<'a> {
    pub(crate) chunks: ChunkList<'a>,
}

impl<'a> MagicBuffer<'a> {
    pub fn new(content: &'a [u8]) -> MagicBuffer<'a> {
        MagicBuffer {
            chunks: ChunkList::new(content),
        }
    }

    pub fn len(&self) -> usize {
        self.chunks.iter().fold(0, |a, c| a + c.len())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut v = Vec::new();
        for it in self.chunks.iter() {
            v.append(&mut it.to_bytes());
        }
        Ok(v)
    }

    #[inline]
    fn do_insert(
        &mut self,
        index: usize,
        content: &[u8],
        left: bool,
        essential: bool,
    ) -> Result<(), Error> {
        if left {
            self.chunks.slice(index)?.0.append(content, essential)
        } else {
            self.chunks.slice(index)?.1.prepend(content, essential)
        }
    }

    pub fn insert_left(&mut self, index: usize, content: &[u8]) -> Result<(), Error> {
        self.do_insert(index, content, true, false)
    }

    pub fn insert_essential_left(&mut self, index: usize, content: &[u8]) -> Result<(), Error> {
        self.do_insert(index, content, true, true)
    }

    pub fn insert_right(&mut self, index: usize, content: &[u8]) -> Result<(), Error> {
        self.do_insert(index, content, false, false)
    }

    pub fn insert_essential_right(&mut self, index: usize, content: &[u8]) -> Result<(), Error> {
        self.do_insert(index, content, false, true)
    }
}
