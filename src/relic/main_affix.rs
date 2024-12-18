use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct RelicMainAffixMap {
    #[serde(flatten)]
    pub relic_main_affix_map: BTreeMap<String, RelicMainAffix>,
}

impl RelicMainAffixMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}relic_main_affixes.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.relic_main_affix_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<RelicMainAffix> {
        self.relic_main_affix_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<RelicMainAffix> {
        self.relic_main_affix_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct RelicMainAffix {
    pub id: String,
    pub affixes: BTreeMap<String, RelicMainAffixProperty>,
}

impl RelicMainAffix {
    pub fn get_keys(&self) -> Vec<String> {
        self.affixes.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<RelicMainAffixProperty> {
        self.affixes.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<RelicMainAffixProperty> {
        self.affixes.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct RelicMainAffixProperty {
    pub affix_id: String,
    pub property: String,
    pub base: f64,
    pub step: f64,
}
