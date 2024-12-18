use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct RelicMap {
    #[serde(flatten)]
    pub relic_map: BTreeMap<String, Relic>,
}

impl RelicMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}relics.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.relic_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Relic> {
        self.relic_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Relic> {
        self.relic_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Relic {
    pub id: String,
    pub set_id: String,
    pub name: String,
    pub rarity: u8,
    pub r#type: String,
    pub max_level: u8,
    pub main_affix_id: String,
    pub sub_affix_id: String,
    pub icon: String,
}
