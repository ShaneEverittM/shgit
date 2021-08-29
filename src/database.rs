use std::path::PathBuf;

use flate2::write::ZlibEncoder as Encoder;
use flate2::Compression;
use hex::ToHex;
use rand::{distributions::Alphanumeric, Rng};
use sha1::{Digest, Sha1};

use crate::Blob;
use std::fs::OpenOptions;
use std::io::{Result as IOResult, Write};

pub struct Database {
    pathname: PathBuf,
}

impl Database {
    pub fn in_dir<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            pathname: path.into(),
        }
    }

    /// Stores the given blob in the database.
    pub fn store(&self, blob: Blob) -> IOResult<()> {
        // Base vec that the blob will serialize to, start with kind
        let mut serialized = blob.kind().to_string().to_lowercase().as_bytes().to_vec();

        // Then a space
        serialized.push(b' ');

        // Then the length
        serialized.append(&mut blob.len().to_string().as_bytes().to_vec());

        // End the header with a null byte
        serialized.push(b'\0');

        // Now the actual data
        serialized.append(&mut blob.to_bytes());

        // Put the data in a SHA1 hasher
        let mut hasher = Sha1::new();
        hasher.update(&serialized);

        // Pull out the hash as a string, then encode it as hex
        let oid = hasher.finalize().to_vec().encode_hex();

        // Write to .git/objects/
        self.write_object(oid, serialized)
    }

    // Takes a hash and a serialized blob and writes it to a file.
    fn write_object(&self, sha1: String, bytes: Vec<u8>) -> IOResult<()> {
        // The object directory, from the first two bytes of the SHA1 hash
        let dirname = self.pathname.join(&sha1[0..=1]);

        // The full path of the final resulting object, using the rest of the hash
        let object_path = dirname.join(&sha1[2..]);

        // A temporary file for consistency
        let temp_path = dirname.join(Database::generate_temp_name());

        // Create
        if !dirname.exists() {
            std::fs::create_dir(dirname)?;
        }

        // In block so that file closes before rename
        {
            // Create new temp file
            let file = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .truncate(false)
                .open(&temp_path)?;

            // Encode
            let mut encoder = Encoder::new(&file, Compression::default());
            encoder.write_all(&bytes)?;
        }

        // Rename to final name now that writing is complete
        std::fs::rename(temp_path, object_path)
    }

    // Generates a temp file name for consistency with other processes
    fn generate_temp_name() -> String {
        String::from("tmp_obj_")
            + &rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect::<String>()
    }
}
