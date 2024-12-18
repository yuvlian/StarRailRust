use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct BlockMap {
    #[serde(flatten)]
    pub block_map: BTreeMap<String, Block>,
}

impl BlockMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}simulated_blocks.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.block_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Block> {
        self.block_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Block> {
        self.block_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Block {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub icon: String,
    pub color: String,
}
