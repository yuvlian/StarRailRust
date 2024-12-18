use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct EventMap {
    #[serde(flatten)]
    pub event_map: BTreeMap<String, Event>,
}

impl EventMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}simulated_events.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.event_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Event> {
        self.event_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Event> {
        self.event_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub image: String,
}