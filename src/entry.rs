#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub oid: String,
}

impl Entry {
    pub fn from(name: String, oid: String) -> Self {
        Self { name, oid }
    }
}
