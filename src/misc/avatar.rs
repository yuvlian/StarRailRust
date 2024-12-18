use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct AvatarMap {
    #[serde(flatten)]
    pub avatar_map: BTreeMap<String, Avatar>,
}

impl AvatarMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}avatars.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.avatar_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Avatar> {
        self.avatar_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Avatar> {
        self.avatar_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Avatar {
    pub id: String,
    pub name: String,
    pub icon: String,
}
