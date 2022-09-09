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
}