use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterSkillMap {
    #[serde(flatten)]
    pub character_skill_map: BTreeMap<String, CharacterSkill>,
}

impl CharacterSkillMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}character_skills.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.character_skill_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<CharacterSkill> {
        self.character_skill_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<CharacterSkill> {
        self.character_skill_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterSkill {
    pub id: String,
    pub name: String,
    pub max_level: u8,
    pub element: String,
    pub r#type: String,
    pub type_text: String,
    pub effect: String,
    pub effect_text: String,
    pub simple_desc: String,
    pub desc: String,
    pub params: Vec<Vec<f64>>,
    pub icon: String,
}
