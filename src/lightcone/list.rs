use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct LightconeMap {
    #[serde(flatten)]
    pub lightcone_map: BTreeMap<String, Lightcone>,
}

impl LightconeMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}light_cones.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.lightcone_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Lightcone> {
        self.lightcone_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Lightcone> {
        self.lightcone_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Lightcone {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    pub path: String,
    pub desc: String,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
}
