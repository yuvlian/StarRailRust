use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct AchievementMap {
    #[serde(flatten)]
    pub achievement_map: BTreeMap<String, Achievement>,
}

impl AchievementMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}achievements.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.achievement_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Achievement> {
        self.achievement_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Achievement> {
        self.achievement_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Achievement {
    pub id: String,
    pub series_id: String,
    pub title: String,
    pub desc: String,
    pub hide_desc: String,
    pub hide: bool,
}
