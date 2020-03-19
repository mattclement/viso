pub struct ItemMetadata {
    pub uuid: String,
    pub vault_uuid: String,
    pub title: String,
    pub metadata: String,
}

impl AsRef<str> for ItemMetadata {
    fn as_ref(&self) -> &str {
        &self.title
    }
}
