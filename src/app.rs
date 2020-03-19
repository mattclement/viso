use serde_json::Value;

pub enum Pane {
    Vault,
    Items,
}

pub struct Vault {
    pub uuid: String,
    pub name: String,
}

impl AsRef<str> for Vault {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

pub struct VaultItem {
    pub uuid: String,
    pub vault_uuid: String,
    pub title: String,
    pub content: String,
}

impl AsRef<str> for VaultItem {
    fn as_ref(&self) -> &str {
        &self.title
    }
}

pub struct App {
    pub selected_vault_index: usize,
    pub selected_item_index: usize,
    pub selected_item_value: Value,
    pub item_open: bool,
    pub vaults: Vec<Vault>,
    pub items: Vec<VaultItem>,
    pub active_pane: Pane,
}

impl App {
    pub fn new() -> App {
        App {
            selected_vault_index: 0,
            selected_item_index: 0,
            active_pane: Pane::Vault,
            selected_item_value: Value::Null,
            item_open: false,
            vaults: vec![Vault {
                uuid: "".into(),
                name: "All Vaults".into(),
            }],
            items: vec![],
        }
    }

    pub fn select_next_vault(&mut self) {
        // Reset the item index on vault change.
        self.selected_item_index = 0;
        self.selected_vault_index += 1;
        if self.selected_vault_index >= self.vaults.len() {
            self.selected_vault_index = 0;
        }
    }

    pub fn select_next_item(&mut self) {
        self.selected_item_index += 1;
        if self.selected_item_index >= self.get_items().len() {
            self.selected_item_index = 0;
        }
    }

    pub fn select_prev_vault(&mut self) {
        // Reset the item index on vault change.
        self.selected_item_index = 0;
        if self.selected_vault_index == 0 {
            self.selected_vault_index = self.vaults.len() - 1;
        } else {
            self.selected_vault_index -= 1;
        }
    }

    pub fn select_prev_item(&mut self) {
        if self.selected_item_index == 0 {
            self.selected_item_index = self.get_items().len() - 1;
        } else {
            self.selected_item_index -= 1;
        }
    }

    pub fn get_items(&self) -> Vec<String> {
        let vault_uuid = &self.vaults[self.selected_vault_index].uuid;
        self.items
            .iter()
            .filter_map(|x| {
                if vault_uuid == "" || &x.vault_uuid == vault_uuid {
                    return Some(x.title.clone());
                }
                None
            })
            .collect()
    }

    pub fn get_vaults(&mut self) {
        use std::process::Command;
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

    pub fn get_entries(&mut self) {
        use std::process::Command;
        let items = Command::new("op")
            .args(&["list", "items"])
            .output()
            .expect("NOOOOOOOOOO");
        let items: Value = serde_json::from_slice(&items.stdout).unwrap_or(Value::Null);
        if let Some(item) = items.as_array() {
            self.items.clear();
            for v in item {
                self.items.push(VaultItem {
                    title: v["overview"]["title"].as_str().unwrap().to_string(),
                    content: v.to_string(),
                    uuid: v["uuid"].as_str().unwrap().to_string(),
                    vault_uuid: v["vaultUuid"].as_str().unwrap().to_string(),
                });
            }
        }
        self.items
            .sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    }

    pub fn get_item(&mut self) {
        use std::process::Command;
        let uuid = &self.items[self.selected_item_index].uuid;
        let item = Command::new("op")
            .args(&["get", "item", &uuid])
            .output()
            .expect("NOOOOOOOOOO");
        let item: Value = serde_json::from_slice(&item.stdout).unwrap_or(Value::Null);
        self.selected_item_value = item;
    }
}
