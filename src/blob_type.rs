use derive_more::Display;

#[derive(Display)]
pub enum BlobKind {
    Blob,
    Tree,
    Commit,
}

/// Used by Database for serialization
pub trait BlobLike {
    /// What kind of blob are you?
    fn kind(&self) -> BlobKind;

    /// Serialize yourself to bytes
    fn to_bytes(&self) -> Vec<u8>;

    /// After the Database does some work, it will reach into the object given to it to store
    /// and set its oid with to match the newly created file in the DB
    fn set_oid(&mut self, oid: &str);

    fn get_oid(&self) -> &str;
}
