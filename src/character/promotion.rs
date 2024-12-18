use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterPromotionMap {
    #[serde(flatten)]
    pub character_promotion_map: BTreeMap<String, CharacterPromotion>,
}

impl CharacterPromotionMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}character_promotions.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.character_promotion_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<CharacterPromotion> {
        self.character_promotion_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<CharacterPromotion> {
        self.character_promotion_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterPromotion {
    pub id: String,
    pub values: Vec<CharacterPromotionValue>,
    pub materials: Vec<Vec<CharacterPromotionMaterial>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterPromotionValue {
    pub hp: CharacterPromotionValueType,
    pub atk: CharacterPromotionValueType,
    pub def: CharacterPromotionValueType,
    pub spd: CharacterPromotionValueType,
    pub taunt: CharacterPromotionValueType,
    pub crit_rate: CharacterPromotionValueType,
    pub crit_dmg: CharacterPromotionValueType,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterPromotionValueType {
    pub base: f64,
    pub step: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CharacterPromotionMaterial {
    pub id: String,
    pub num: u32,
}
