use serde_json::Value;

pub struct Item {
    pub uuid: String,
    pub data: String,
}

impl Item {
    pub fn get_password(&self) -> String {
        let fields: Value = serde_json::from_str(&self.data).unwrap_or(Value::Null);
        if let Some(fields) = fields["details"]["fields"].as_array() {
            for field in fields {
                if field["designation"].as_str() == Some("password") {
                    return field["value"].as_str().unwrap().to_string();
                }
            }
        }

        "".into()
    }
}
