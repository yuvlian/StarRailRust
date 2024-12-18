use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct RelicSubAffixList {
    #[serde(flatten)]
    pub relic_sub_affix: BTreeMap<String, RelicSubAffix>,
}

impl RelicSubAffixList {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}relic_sub_affixes.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.relic_sub_affix.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<RelicSubAffix> {
        self.relic_sub_affix.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<RelicSubAffix> {
        self.relic_sub_affix.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct RelicSubAffix {
    pub id: String,
    pub affixes: BTreeMap<String, RelicSubAffixProperty>,
}

impl RelicSubAffix {
    pub fn get_keys(&self) -> Vec<String> {
        self.affixes.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<RelicSubAffixProperty> {
        self.affixes.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<RelicSubAffixProperty> {
        self.affixes.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct RelicSubAffixProperty {
    pub affix_id: String,
    pub property: String,
    pub base: f64,
    pub step: f64,
    pub step_num: u8,
}
