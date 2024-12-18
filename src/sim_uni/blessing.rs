use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct BlessingMap {
    #[serde(flatten)]
    pub blessing_map: BTreeMap<String, Blessing>,
}

impl BlessingMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}simulated_blessings.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.blessing_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Blessing> {
        self.blessing_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Blessing> {
        self.blessing_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Blessing {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub enhanced_desc: String,
}
