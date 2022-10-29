use super::BLOCK_SIZE;

#[derive(Debug)]
#[repr(C)]
pub struct Bitmap([u8; BLOCK_SIZE]);

impl Default for Bitmap {
    fn default() -> Self {
        Self([0; BLOCK_SIZE])
    }
}

impl Bitmap {
    pub fn new() -> Self {
        Self([0; BLOCK_SIZE])
    }

    pub fn from(data: [u8; BLOCK_SIZE]) -> Self {
        Self(data)
    }

    fn search_empty(&self) -> Result<usize, BitmapError> {
        let mut index = 0;
        let mut map_iter = self.0.iter();
        while let Some(byte) = map_iter.next() {
            for i in 0..8 {
                if (byte & 1<<i)==0 {
                    return Ok(index + i)
                } else {
                    index += 1;
                }
            }
        }
        Err(BitmapError::NoEmpty)
    }

    pub fn alloc(&mut self) -> Result<usize, BitmapError> {
        let index = self.search_empty()?;
        let nbyte = index/8;
        self.0[nbyte] |= 1<<index%8;
        Ok(index)
    }

    pub fn dealloc(&mut self, index: usize) -> Result<(), BitmapError> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BitmapError {
    NoEmpty,
    Invalid
}