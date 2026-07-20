use crate::arz::Record;

const CHECK_LIST: &str = "affixCheckList";
const OVERRIDE: &str = "affixOverride";

#[derive(Debug, Clone)]
pub struct AscensionAffixSwap {
    pub check_list: Vec<String>,
    pub override_table: String,
}

impl From<&Record> for AscensionAffixSwap {
    fn from(record: &Record) -> Self {
        let check_list = record
            .data
            .get(CHECK_LIST)
            .and_then(|value| value.as_strings())
            .unwrap_or_default();
        let override_table = record
            .data
            .get(OVERRIDE)
            .and_then(|value| value.as_string())
            .unwrap_or_default();
        Self {
            check_list,
            override_table,
        }
    }
}
