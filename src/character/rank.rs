use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterRankMap {
    #[serde(flatten)]
    pub character_rank_map: BTreeMap<String, CharacterRank>,
}

impl CharacterRankMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}character_ranks.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.character_rank_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<CharacterRank> {
        self.character_rank_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<CharacterRank> {
        self.character_rank_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterRank {
    pub id: String,
    pub name: String,
    pub rank: u8,
    pub desc: String,
    pub materials: Vec<CharacterRankMaterial>,
    pub level_up_skills: Vec<CharacterRankLevelUpSkill>,
    pub icon: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterRankMaterial {
    pub id: String,
    pub num: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterRankLevelUpSkill {
    pub id: String,
    pub num: u8,
}
