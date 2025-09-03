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
        let tag = record.data.get(ITEM_TAG).expect("Item {id} had no {ITEM_NAME_TAG");
        let level = record.data.get(ITEM_LEVEL).expect("Item {id} had no {ITEM_NAME_TAG");

        Self {
            id: record.id.clone(),
            tag: tag.as_string().unwrap(),
            level: level.as_int().unwrap(),
        }
    }
}
