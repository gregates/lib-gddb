use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::arz::Record;

pub const PREFIX_PATH: &str = "records/items/lootaffixes/prefix/";
pub const SUFFIX_PATH: &str = "records/items/lootaffixes/suffix/";

const AFFIX_NAME: &str = "lootRandomizerName";
const AFFIX_RARITY: &str = "itemClassification";

#[derive(Debug, Clone)]
pub struct Affix {
    pub id: String,
    pub tag: String,
    pub rarity: AffixRarity,
    pub description: Option<String>,
    pub record: Record,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AffixRarity {
    Unknown,
    Magic,
    Rare,
}

impl fmt::Display for AffixRarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "err: unknown value for affix rarity"),
            Self::Magic => write!(f, "magic"),
            Self::Rare => write!(f, "rare"),
        }
    }
}

impl FromStr for AffixRarity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Magical" | "magical" => Ok(Self::Magic),
            "Rare" | "rare" => Ok(Self::Rare),
            _ => Ok(Self::Unknown),
        }
    }
}

impl Affix {
    pub fn localize<'a>(&'a self, tags: &'a HashMap<String, String>) -> &'a str {
        tags.get(&self.tag)
            .map(|s| s.as_str())
            .unwrap_or_else(|| self.tag.as_str())
    }
}

impl From<Record> for Affix {
    fn from(record: Record) -> Self {
        Self {
            id: record.id.clone(),
            tag: record
                .data
                .get(AFFIX_NAME)
                .map(|val| val.as_string().unwrap())
                .unwrap_or_else(|| record.id.clone()),
            rarity: record
                .data
                .get(AFFIX_RARITY)
                .map(|rarity| rarity
                    .as_string()
                    .unwrap()
                    .parse::<AffixRarity>()
                    .unwrap()
                )
                .unwrap_or(AffixRarity::Unknown),
            description: record.data.get("FileDescription").map(|desc| desc.to_string()),
            record,
        }
    }
}

impl fmt::Display for Affix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Affix {{ id: {}, desc: {}, tag: {} }}",
            self.id,
            self.description.as_ref().map(|d| d.as_str()).unwrap_or("undefined"),
            self.tag
        )
    }
}
