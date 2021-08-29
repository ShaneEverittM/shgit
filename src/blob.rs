use derive_more::Display;

pub struct Blob {
    data: Vec<u8>,
}

#[derive(Display)]
pub enum BlobType {
    Blob,
}

impl Blob {
    pub fn from(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn kind(&self) -> BlobType {
        BlobType::Blob
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
