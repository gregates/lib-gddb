use crate::arz::Record;
use crate::rarity::Rarity;

const ITEM_LEVEL: &str = "itemLevel";
const ITEM_TAG: &str = "itemNameTag";
const ITEM_RARITY: &str = "itemClassification";

#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub tag: String,
    pub level: u32,
    pub rarity: Rarity,
}

impl From<&Record> for Item {
    fn from(record: &Record) -> Self {
        let tag = record.data.get(ITEM_TAG).map(|s| s.as_string().unwrap()).unwrap_or(record.id.clone());
        let level = record.data.get(ITEM_LEVEL).map(|i| i.as_int().unwrap()).expect(&format!("Item {} had no {}", record.id, ITEM_LEVEL));
        let rarity = record.data.get(ITEM_RARITY).map(|s| s.as_string().unwrap().parse::<Rarity>().unwrap()).unwrap_or(Rarity::Unknown);

        Self {
            id: record.id.clone(),
            tag,
            level,
            rarity,
        }
    }
}
