use crate::affix_combo_weights::AffixComboWeights;
use crate::arz::Record;
use crate::rollable::RollableItem;
use crate::util::ensure_len;

const PREFIX_TABLE_NAME: &str = "prefixTableName";
const PREFIX_TABLE_WEIGHT: &str = "prefixTableWeight";
const PREFIX_TABLE_MIN: &str = "prefixTableLevelMin";
const PREFIX_TABLE_MAX: &str = "prefixTableLevelMax";

#[derive(Debug, Clone)]
pub struct LootTable {
    pub id: String,
    pub loots: Vec<RollableItem>,
    pub prefix_tables: Vec<RollableItem>,
    pub suffix_tables: Vec<RollableItem>,
    pub rare_prefix_tables: Vec<RollableItem>,
    pub rare_suffix_tables: Vec<RollableItem>,
    pub combo_weights: AffixComboWeights,
}

impl From<&Record> for LootTable {
    fn from(record: &Record) -> Self {
        let mut loots = vec![];
        let mut loot_weights = vec![];
        let mut loot_ranges = vec![];
        let mut prefix_tables = vec![];
        let mut prefix_table_weights = vec![];
        let mut prefix_table_ranges = vec![];
        let mut suffix_tables = vec![];
        let mut suffix_table_weights = vec![];
        let mut suffix_table_ranges = vec![];
        let mut rare_prefix_tables = vec![];
        let mut rare_prefix_table_weights = vec![];
        let mut rare_prefix_table_ranges = vec![];
        let mut rare_suffix_tables = vec![];
        let mut rare_suffix_table_weights = vec![];
        let mut rare_suffix_table_ranges = vec![];
        let mut combo_weights = AffixComboWeights::new();
        for (key, value) in record.data.iter() {
            if let Some(weight) = value.as_float() {
                if combo_weights.set(key, weight) {
                    continue;
                }
            }
            if key.starts_with(PREFIX_TABLE_NAME) {
                let i = key[PREFIX_TABLE_NAME.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(prefix_tables, i, "".to_string());
                prefix_tables[i] = value.as_string().unwrap();
            } else if key.starts_with(PREFIX_TABLE_WEIGHT) {
                let i = key[PREFIX_TABLE_WEIGHT.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(prefix_table_weights, i, 0f32);
                prefix_table_weights[i] = value.as_float().unwrap();
            } else if key.starts_with(PREFIX_TABLE_MIN) {
                let i = key[PREFIX_TABLE_MIN.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(prefix_table_ranges, i, 0..1);
                let min = value.as_int().unwrap();
                prefix_table_ranges[i] = min..prefix_table_ranges[i].end.min(min + 1);
            } else if key.starts_with(PREFIX_TABLE_MAX) {
                let i = key[PREFIX_TABLE_MAX.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(prefix_table_ranges, i, 0..1);
                let max = value.as_int().unwrap();
                prefix_table_ranges[i] = prefix_table_ranges[i].start..max;
            }
        }

        let loots = loots.into_iter()
            .zip(loot_weights.into_iter())
            .zip(loot_ranges.into_iter())
            .map(|((id, weight), level_range)| RollableItem {
                id, weight, level_range,
            })
            .collect::<Vec<_>>();

        let prefix_tables = prefix_tables.into_iter()
            .zip(prefix_table_weights.into_iter())
            .zip(prefix_table_ranges.into_iter())
            .map(|((id, weight), level_range)| RollableItem {
                id, weight, level_range,
            })
            .collect::<Vec<_>>();

        let suffix_tables = suffix_tables.into_iter()
            .zip(suffix_table_weights.into_iter())
            .zip(suffix_table_ranges.into_iter())
            .map(|((id, weight), level_range)| RollableItem {
                id, weight, level_range,
            })
            .collect::<Vec<_>>();

        let rare_prefix_tables = rare_prefix_tables.into_iter()
            .zip(rare_prefix_table_weights.into_iter())
            .zip(rare_prefix_table_ranges.into_iter())
            .map(|((id, weight), level_range)| RollableItem {
                id, weight, level_range,
            })
            .collect::<Vec<_>>();

        let rare_suffix_tables = rare_suffix_tables.into_iter()
            .zip(rare_suffix_table_weights.into_iter())
            .zip(rare_suffix_table_ranges.into_iter())
            .map(|((id, weight), level_range)| RollableItem {
                id, weight, level_range,
            })
            .collect::<Vec<_>>();

        Self {
            id: record.id.clone(),
            loots,
            prefix_tables,
            suffix_tables,
            rare_prefix_tables,
            rare_suffix_tables,
            combo_weights,
        }
    }
}
