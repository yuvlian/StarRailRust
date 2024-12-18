use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct ItemMap {
    #[serde(flatten)]
    pub item_map: BTreeMap<String, Item>,
}

impl ItemMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}items.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.item_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Item> {
        self.item_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Item> {
        self.item_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub sub_type: String,
    pub rarity: u8,
    pub icon: String,
    pub come_from: Vec<String>,
}
