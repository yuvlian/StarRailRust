use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct RelicSetMap {
    #[serde(flatten)]
    pub relic_set_map: BTreeMap<String, RelicSet>,
}

impl RelicSetMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}relic_sets.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.relic_set_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<RelicSet> {
        self.relic_set_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<RelicSet> {
        self.relic_set_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct RelicSet {
    pub id: String,
    pub name: String,
    pub desc: Vec<String>,
    pub properties: Vec<Vec<RelicSetProperty>>,
    pub icon: String,
    pub guide_overview: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RelicSetProperty {
    pub r#type: String,
    pub value: f64,
}
