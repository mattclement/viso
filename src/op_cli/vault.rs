pub struct Vault {
    pub uuid: String,
    pub name: String,
}

impl AsRef<str> for Vault {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

