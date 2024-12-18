use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct DescriptionMap {
    #[serde(flatten)]
    pub description_map: BTreeMap<String, Description>,
}

impl DescriptionMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}descriptions.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.description_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Description> {
        self.description_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Description> {
        self.description_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Description {
    pub id: String,
    pub title: String,
    pub desc: String,
}
