use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct LightconeRankMap {
    #[serde(flatten)]
    pub lightcone_rank_map: BTreeMap<String, LightconeRank>,
}

impl LightconeRankMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}lightcone_ranks.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.lightcone_rank_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<LightconeRank> {
        self.lightcone_rank_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<LightconeRank> {
        self.lightcone_rank_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct LightconeRank {
    pub id: String,
    pub skill: String,
    pub desc: String,
    pub params: Vec<Vec<f64>>,
    pub properties: Vec<Vec<LightconeRankProperty>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LightconeRankProperty {
    pub r#type: String,
    pub value: f64,
}
