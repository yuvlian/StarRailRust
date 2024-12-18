use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct PropertyMap {
    #[serde(flatten)]
    pub property_map: BTreeMap<String, Property>,
}

impl PropertyMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}properties.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.property_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Property> {
        self.property_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Property> {
        self.property_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Property {
    pub r#type: String,
    pub name: String,
    pub field: String,
    pub affix: bool,
    pub ratio: bool,
    pub percent: bool,
    pub order: u8,
    pub icon: String,
}
