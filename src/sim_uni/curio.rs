use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct CurioMap {
    #[serde(flatten)]
    pub curio_map: BTreeMap<String, Curio>,
}

impl CurioMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}simulated_curios.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.curio_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Curio> {
        self.curio_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Curio> {
        self.curio_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Curio {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub bg_desc: String,
    pub icon: String,
}
