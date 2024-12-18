use std::collections::HashMap;
use std::fmt;

use crate::arz::Record;

pub const PREFIX_PATH: &str = "records/items/lootaffixes/prefix/";
pub const SUFFIX_PATH: &str = "records/items/lootaffixes/suffix/";

pub struct Affix {
    pub id: String,
    pub tag: String,
    pub description: Option<String>,
    pub record: Record,
}

impl Affix {
    pub fn localize(&self, tags: &HashMap<String, String>) -> String {
        tags.get(&self.tag)
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.tag.clone())
    }
}

impl From<Record> for Affix {
    fn from(record: Record) -> Self {
        Self {
            id: record.id.clone(),
            tag: record.data.get("lootRandomizerName").unwrap().to_string(),
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
