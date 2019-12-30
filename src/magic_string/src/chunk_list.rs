use crate::chunk::Chunk;
use crate::error::Error;
use std::mem;

pub(crate) struct ChunkList<'a> {
    head: Link<'a>,
}

type Link<'a> = Option<Box<Node<'a>>>;
struct Node<'a> {
    pub elem: Chunk<'a>,
    pub next: Link<'a>,
}

impl<'a> ChunkList<'a> {
    pub fn new(original_content: &'a [u8]) -> Self {
        ChunkList {
            head: Some(Box::new(Node {
                elem: Chunk::new(original_content),
                next: None,
            })),
        }
    }

    pub fn peek(&self) -> Option<&Chunk> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut Chunk<'a>> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn slice(&mut self, start: usize) -> Result<(&mut Chunk<'a>, &mut Chunk<'a>), Error> {
        if let Some(node) = self.get_node_at(start) {
            let chunk = &mut node.elem;
            let inner_start = start - chunk.start;
            let new_right = chunk.right.as_ref().map(|_| Vec::new());
            let new_chunk = Chunk {
                left: Some(Vec::new()),
                right: chunk.right.take(),
                content: chunk.content.as_ref().map(|c| &c[inner_start..]),
                original_content: chunk.original_content,
                assert_left: false,
                assert_right: chunk.assert_right,
                start,
                end: chunk.end,
            };
            let box_node = Box::new(Node {
                elem: new_chunk,
                next: mem::replace(&mut node.next, None),
            });

            chunk.content = chunk.content.as_ref().map(|x| &x[..inner_start]);
            chunk.end = start;
            chunk.right = new_right;
            chunk.assert_right = false;
            node.next = Some(box_node);

            Ok((chunk, &mut node.next.as_mut().unwrap().as_mut().elem))
        } else {
            Err(Error::IndexOutOfBoundError(start))
        }
    }

    fn get_node_at(&mut self, index: usize) -> Option<&mut Node<'a>> {
        let mut current = &mut self.head;
        while let Some(ref mut box_node) = current {
            let node = box_node.as_mut();
            if index >= node.elem.start && index <= node.elem.end {
                return Some(node);
            }
            current = &mut node.next;
        }
        None
    }

    pub fn get_chunk_at(&self, index: usize) -> Option<&Chunk<'a>> {
        let mut current = &self.head;
        while let Some(ref box_node) = current {
            let node = box_node.as_ref();
            if index >= node.elem.start && index < node.elem.end {
                return Some(&node.elem);
            }
            current = &node.next;
        }
        None
    }

    pub fn get_mut_chunk_at(&mut self, index: usize) -> Option<&mut Chunk<'a>> {
        self.get_node_at(index).map(|node| &mut node.elem)
    }

    pub fn iter<'b: 'a>(&'b self) -> Iter<'_, 'a> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, 'a> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a> Drop for ChunkList<'a> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub(crate) struct Iter<'b, 'a: 'b> {
    next: Option<&'a Node<'b>>,
}

impl<'b, 'a: 'b> Iterator for Iter<'b, 'a> {
    type Item = &'b Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub(crate) struct IterMut<'b, 'a: 'b> {
    next: Option<&'b mut Node<'a>>,
}

impl<'b, 'a: 'b> Iterator for IterMut<'b, 'a> {
    type Item = &'b mut Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}
