use crate::author::Author;
use crate::blob_type::{BlobKind, BlobLike};

pub struct Commit {
    tree_oid: String,
    author: Author,
    message: String,
    oid: String,
}

impl BlobLike for Commit {
    fn kind(&self) -> BlobKind {
        BlobKind::Commit
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut serialized: Vec<u8> = Vec::new();

        serialized.extend_from_slice(b"tree ");
        serialized.extend_from_slice(self.tree_oid.as_bytes());
        serialized.push(b'\n');

        serialized.extend_from_slice(b"author ");
        serialized.append(&mut self.author.to_bytes());
        serialized.push(b'\n');

        serialized.extend_from_slice(b"committer ");
        serialized.append(&mut self.author.to_bytes());
        serialized.push(b'\n');
        serialized.push(b'\n');

        serialized.extend_from_slice(self.message.as_bytes());

        serialized
    }

    fn set_oid(&mut self, oid: &str) {
        self.oid = oid.into()
    }

    fn get_oid(&self) -> &str {
        &self.oid
    }
}

impl Commit {
    pub fn new(tree_oid: String, author: Author, message: String) -> Self {
        Commit {
            tree_oid,
            author,
            message,
            oid: String::new(),
        }
    }
}
