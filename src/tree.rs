use crate::blob_type::{BlobKind, BlobLike};
pub use crate::entry::Entry;

const MODE: &str = "100644";

pub struct Tree {
    entries: Vec<Entry>,
    oid: String,
}

impl BlobLike for Tree {
    fn kind(&self) -> BlobKind {
        BlobKind::Tree
    }

    fn to_bytes(&self) -> Vec<u8> {
        // Create a clone to sort in place
        let mut sorted_entries = self.entries.clone();

        // Sort by filename
        sorted_entries.sort_by(|e1, e2| e1.name.cmp(&e2.name));

        // Serialize
        let mut serialized_entries = Vec::new();
        for entry in sorted_entries {
            // First the special MODE string
            serialized_entries.extend_from_slice(MODE.as_bytes());

            // Then a space
            serialized_entries.push(b' ');

            // Then the name of the file
            serialized_entries.extend_from_slice(entry.name.as_bytes());

            // Then a null byte
            serialized_entries.push(b'\0');

            // Last the oid of the object
            serialized_entries.append(&mut hex::decode(entry.oid).unwrap());
        }
        serialized_entries
    }

    fn set_oid(&mut self, oid: &str) {
        self.oid = oid.to_string()
    }

    fn get_oid(&self) -> &str {
        &self.oid
    }
}

impl Tree {
    pub fn from(entries: Vec<Entry>) -> Self {
        Self {
            entries,
            oid: String::new(),
        }
    }
}
