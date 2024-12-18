pub use reqwest::Client;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct FetchError {
    url: String,
    status: reqwest::StatusCode,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to fetch URL: {}. Status: {}",
            self.url, self.status
        )
    }
}

impl Error for FetchError {}

async fn fetch_json(url: &str, client: &Client) -> Result<String, Box<dyn Error>> {
    let response = client.get(url).send().await?;

    if response.status().is_success() {
        response.text().await.map_err(Into::into)
    } else {
        Err(Box::new(FetchError {
            url: url.to_string(),
            status: response.status(),
        }))
    }
}

pub enum BaseUrl {
    Cht,
    Cn,
    De,
    En,
    Es,
    Fr,
    Id,
    Jp,
    Kr,
    Pt,
    Ru,
    Th,
    Vi,
}

impl BaseUrl {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cht => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/cht/"
            }
            Self::Cn => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/cn/"
            }
            Self::De => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/de/"
            }
            Self::En => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/en/"
            }
            Self::Es => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/es/"
            }
            Self::Fr => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/fr/"
            }
            Self::Id => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/id/"
            }
            Self::Jp => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/jp/"
            }
            Self::Kr => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/kr/"
            }
            Self::Pt => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/pt/"
            }
            Self::Ru => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/ru/"
            }
            Self::Th => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/th/"
            }
            Self::Vi => {
                "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/vi/"
            }
        }
    }
}

pub mod character;
pub mod lightcone;
pub mod misc;
pub mod relic;
pub mod sim_uni;
