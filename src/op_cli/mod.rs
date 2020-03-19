use serde_json::Value;
use std::collections::HashMap;
use std::process::Command;

pub mod item;
pub mod metadata;
pub mod vault;
use item::*;
use metadata::*;
use vault::*;

pub struct OnePassword {
    pub vaults: Vec<Vault>,
    pub metadatas: Vec<ItemMetadata>,

    /// Map of items that have been retreived.
    pub items: HashMap<String, Item>,
}

impl OnePassword {
    pub fn new() -> OnePassword {
        OnePassword {
            vaults: vec![Vault {
                uuid: "".into(),
                name: "All Vaults".into(),
            }],
            metadatas: vec![],
            items: HashMap::new(),
        }
    }

    /// Filter metadata by vault uuid. Returns a list of their titles.
    pub fn metadata_in_vault(&self, index: usize) -> Vec<String> {
        let uuid = &self.vaults[index].uuid;
        self.metadatas
            .iter()
            .filter_map(|x| {
                if uuid == "" || uuid == &x.vault_uuid {
                    return Some(x.title.clone());
                }
                None
            })
            .collect()
    }

    /// Filter metadata by vault uuid. Returns a list of their titles.
    pub fn items_in_vault(&self, index: usize) -> Vec<&ItemMetadata> {
        let uuid = &self.vaults[index].uuid;
        self.metadatas
            .iter()
            .filter(|x| uuid == "" || uuid == &x.vault_uuid)
            .collect()
    }

    pub fn vault_index_by_uuid(&self, uuid: &str) -> Option<usize> {
        self.vaults.iter().position(|x| uuid == x.uuid)
    }

    /// Load all Vaults. Perform a case-insensitive sort on the titles.
    pub fn load_vaults(&mut self) {
        let vaults = Command::new("op")
            .args(&["list", "vaults"])
            .output()
            .expect("NOOOOOOOOOO");
        let vaults: Value = serde_json::from_slice(&vaults.stdout).unwrap_or(Value::Null);
        if let Some(vault) = vaults.as_array() {
            for v in vault {
                self.vaults.push(Vault {
                    uuid: v["uuid"].as_str().unwrap().into(),
                    name: v["name"].as_str().unwrap().to_string(),
                });
            }
        }
        self.vaults
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }

    /// Load all ItemMetadata. Perform a case-insensitive sort on the titles.
    pub fn load_item_metadata(&mut self) {
        let items = Command::new("op")
            .args(&["list", "items"])
            .output()
            .expect("NOOOOOOOOOO");
        let items: Value = serde_json::from_slice(&items.stdout).unwrap_or(Value::Null);
        if let Some(item) = items.as_array() {
            self.items.clear();
            for v in item {
                self.metadatas.push(ItemMetadata {
                    title: v["overview"]["title"].as_str().unwrap().to_string(),
                    metadata: v.to_string(),
                    uuid: v["uuid"].as_str().unwrap().to_string(),
                    vault_uuid: v["vaultUuid"].as_str().unwrap().to_string(),
                });
            }
        }
        self.metadatas
            .sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    }

    /// Load or retrieve an Item by its UUID.
    pub fn get_item(&mut self, uuid: &str) -> &Item {
        if !self.items.contains_key(uuid) {
            let item = Command::new("op")
                .args(&["get", "item", &uuid])
                .output()
                .expect("NOOOOOOOOOO");
            self.items.insert(
                uuid.to_string(),
                Item {
                    uuid: uuid.to_string(),
                    data: String::from_utf8(item.stdout)
                        .unwrap_or(format!("Couldn't load {uuid}", uuid = uuid)),
                },
            );
        }
        self.items.get(uuid).unwrap()
    }
}
