use std::iter::Iterator;


#[derive(Clone)]
pub struct TransposeChunks<'a, T> where T: 'a {
    inner:      &'a [T], 
    chunk_size: usize,
    offset:     usize, 
}

impl<'a, T> TransposeChunks<'a, T> where T: 'a + Clone {
    pub fn transpose(slice: &'a [T], chunk_size: usize) -> TransposeChunks<'a, T> {
        assert!(chunk_size != 0);

        TransposeChunks {
            inner:      slice,
            chunk_size: chunk_size,
            offset:     0,
        }
    }
}

impl<'a, T> Iterator for TransposeChunks<'a, T> where T: 'a + Clone {
    type Item = Vec<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.chunk_size {
            return None;
        }

        let mut chunk = Vec::new();
        for i in 0..self.chunk_size {
            chunk.push(self.inner[(i * self.chunk_size + self.offset) % self.inner.len()].clone());
        }

        self.offset += 1;

        Some(chunk)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.offset, Some(self.chunk_size))
    }
}

impl<'a, T> DoubleEndedIterator for TransposeChunks<'a, T> where T: 'a + Clone {  
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.offset == 0 {
            return None;
        }

        let mut chunk = Vec::new();
        for i in 0..self.chunk_size {
            chunk.push(self.inner[(i * self.chunk_size + self.offset) % self.inner.len()].clone());
        }

        self.offset -= 1;

        Some(chunk)
    }
}

impl<'a, T> ExactSizeIterator for TransposeChunks<'a, T> where T: 'a + Clone {
    #[inline]
    fn len(&self) -> usize {
        let chunk_count =  self.inner.len() / self.chunk_size;

        if self.inner.len() % self.chunk_size == 0 {
            return chunk_count;
        }
            
        chunk_count + 1
    }
}