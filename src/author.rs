use crate::blob_type::{BlobKind, BlobLike};

pub struct Author {
    name: String,
    email: String,
    time: String,
}

impl BlobLike for Author {
    fn kind(&self) -> BlobKind {
        BlobKind::Blob
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut serialized: Vec<u8> = Vec::new();

        serialized.extend_from_slice(self.name.as_bytes());
        serialized.extend_from_slice(b" <");
        serialized.extend_from_slice(self.email.as_bytes());
        serialized.extend_from_slice(b"> ");
        serialized.extend_from_slice(self.time.as_bytes());

        serialized
    }

    fn set_oid(&mut self, oid: &str) {
        todo!()
    }

    fn get_oid(&self) -> &str {
        todo!()
    }
}

impl Author {
    pub fn new(name: String, email: String, time: String) -> Self {
        Author { name, email, time }
    }
}
