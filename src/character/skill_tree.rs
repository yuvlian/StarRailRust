use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterSkillTreeMap {
    #[serde(flatten)]
    pub character_skill_tree_map: BTreeMap<String, CharacterSkillTree>,
}

impl CharacterSkillTreeMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}character_skill_trees.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.character_skill_tree_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<CharacterSkillTree> {
        self.character_skill_tree_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<CharacterSkillTree> {
        self.character_skill_tree_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterSkillTree {
    pub id: String,
    pub name: String,
    pub max_level: u8,
    pub desc: String,
    pub params: Vec<Vec<f64>>,
    pub anchor: String,
    pub pre_points: Vec<String>,
    pub level_up_skills: Vec<CharacterSkillTreeLevelUpSkill>,
    pub levels: Vec<CharacterSkillTreeLevel>,
    pub icon: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterSkillTreeLevelUpSkill {
    pub id: String,
    pub num: u8,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterSkillTreeLevel {
    pub promotion: u8,
    pub level: u8,
    pub properties: Vec<CharacterSkillTreeProperty>,
    pub materials: Vec<CharacterSkillTreeMaterial>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterSkillTreeProperty {
    pub r#type: String,
    pub value: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterSkillTreeMaterial {
    pub id: String,
    pub num: u32,
}
