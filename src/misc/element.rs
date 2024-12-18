use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct ElementMap {
    #[serde(flatten)]
    pub element_map: BTreeMap<String, Element>,
}

impl ElementMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}elements.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.element_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Element> {
        self.element_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Element> {
        self.element_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub color: String,
    pub icon: String,
}
