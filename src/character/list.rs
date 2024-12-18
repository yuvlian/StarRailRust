use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterMap {
    #[serde(flatten)]
    pub character_map: BTreeMap<String, Character>,
}

impl CharacterMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}characters.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.character_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<Character> {
        self.character_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<Character> {
        self.character_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub rarity: u8,
    pub path: String,
    pub element: String,
    pub max_sp: u16,
    pub ranks: Vec<String>,
    pub skills: Vec<String>,
    pub skill_trees: Vec<String>,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
}
