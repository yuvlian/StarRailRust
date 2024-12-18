use crate::{BTreeMap, BaseUrl, Client, Deserialize, Error, fetch_json};

#[derive(Deserialize, Clone, Debug)]
pub struct LightconePromotionMap {
    #[serde(flatten)]
    pub lightcone_promotion_map: BTreeMap<String, LightconePromotion>,
}

impl LightconePromotionMap {
    pub async fn fetch_map(base_url: &BaseUrl, client: &Client) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}lightcone_promotions.json", base_url.as_str());
        let json_text = fetch_json(&url, client).await?;
        let v: Self = serde_json::from_str(&json_text)?;
        Ok(v)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.lightcone_promotion_map.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<LightconePromotion> {
        self.lightcone_promotion_map.values().cloned().collect()
    }

    pub fn get_value_by_key(&self, key: &str) -> Option<LightconePromotion> {
        self.lightcone_promotion_map.get(key).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct LightconePromotion {
    pub id: String,
    pub values: Vec<LightconePromotionValue>,
    pub materials: Vec<Vec<LightconePromotionMaterial>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LightconePromotionValue {
    pub hp: LightconePromotionValueType,
    pub atk: LightconePromotionValueType,
    pub def: LightconePromotionValueType,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LightconePromotionValueType {
    pub base: f64,
    pub step: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LightconePromotionMaterial {
    pub id: String,
    pub num: u32,
}
