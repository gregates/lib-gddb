use crate::arz::Record;

const ITEM_LEVEL: &str = "itemLevel";
const ITEM_TAG: &str = "itemNameTag";

#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub tag: String,
    pub level: u32,
}

impl From<&Record> for Item {
    fn from(record: &Record) -> Self {
        let tag = record.data.get(ITEM_TAG).map(|s| s.as_string().unwrap()).unwrap_or(record.id.clone());
        let level = record.data.get(ITEM_LEVEL).map(|i| i.as_int().unwrap()).expect(&format!("Item {} had no {}", record.id, ITEM_LEVEL));

        Self {
            id: record.id.clone(),
            tag,
            level,
        }
    }
}
