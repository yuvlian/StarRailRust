use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct NicknameData {
    pub characters: NicknameMap,
    pub light_cones: NicknameMap,
    pub relic_sets: NicknameMap,
}

impl NicknameData {
    pub async fn fetch_data(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}nickname.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct NicknameMap {
    #[serde(flatten)]
    pub id: BTreeMap<String, Vec<String>>,
}

impl NicknameMap {
    pub fn get_keys(&self) -> Vec<String> {
        self.id.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Vec<String>> {
        self.id.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Vec<String>> {
        self.id.get(key).cloned()
    }
}
