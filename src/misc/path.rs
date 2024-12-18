use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct PathMap {
    #[serde(flatten)]
    pub path_map: BTreeMap<String, Path>,
}

impl PathMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}paths.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.path_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Path> {
        self.path_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Path> {
        self.path_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Path {
    pub id: String,
    pub text: String,
    pub name: String,
    pub desc: String,
    pub icon: String,
}