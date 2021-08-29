use crate::blob_type::{BlobKind, BlobLike};

pub struct Blob {
    data: Vec<u8>,
    oid: String,
}

impl BlobLike for Blob {
    fn kind(&self) -> BlobKind {
        BlobKind::Blob
    }
    fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn set_oid(&mut self, oid: &str) {
        self.oid = oid.to_string();
    }

    fn get_oid(&self) -> &str {
        &self.oid
    }
}

impl Blob {
    pub fn from(data: Vec<u8>) -> Self {
        Self {
            data,
            oid: String::new(),
        }
    }
}
