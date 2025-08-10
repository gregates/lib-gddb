use std::collections::HashMap;

use crate::affix::Affix;
use crate::arz::Record;
use crate::rollable::RollableItem;
use crate::util::ensure_len;

const NAME: &str = "randomizerName";
const WEIGHT: &str = "randomizerWeight";
const MIN: &str = "randomizerLevelMin";
const MAX: &str = "randomizerLevelMax";

#[derive(Debug, Clone)]
pub struct AffixTable {
    pub id: String,
    pub loot_randomizers: Vec<RollableItem>,
}

impl AffixTable {
    pub fn resolve<'a>(&self, level: u32, affixes: &HashMap<String, &'a Affix>) -> Vec<(&'a Affix, f32)> {
        let total = self.loot_randomizers
            .iter()
            .filter(|rollable| rollable.level_range.contains(&level))
            .map(|rollable| rollable.weight)
            .sum::<f32>();
        self.loot_randomizers
            .iter()
            .filter(|rollable| rollable.level_range.contains(&level))
            .map(|rollable| {
                let affix = affixes.get(&rollable.id).expect(&format!("Missing affix: {}", rollable.id));
                (*affix, rollable.weight / total)
            })
            .collect()
    }
}

impl From<&Record> for AffixTable {
    fn from(record: &Record) -> Self {
        let mut affixes = vec![];
        let mut weights = vec![];
        let mut ranges = vec![];
        for (key, value) in record.data.iter() {
            if key.starts_with(NAME) {
                let i = key[NAME.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(affixes, i, "".to_string());
                affixes[i] = value.as_string().unwrap();
            } else if key.starts_with(WEIGHT) {
                let i = key[WEIGHT.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(weights, i, 0f32);
                weights[i] = value.as_float().unwrap();
            } else if key.starts_with(MIN) {
                let i = key[MIN.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(ranges, i, 0..1);
                let min = value.as_int().unwrap();
                ranges[i] = min..ranges[i].end.min(min + 1);
            } else if key.starts_with(MAX) {
                let i = key[MAX.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(ranges, i, 0..1);
                let max = value.as_int().unwrap();
                ranges[i] = ranges[i].start..max;
            }
        }

        let loot_randomizers = affixes.into_iter()
            .zip(weights.into_iter())
            .zip(ranges.into_iter())
            .map(|((id, weight), level_range)| RollableItem {
                id, weight, level_range,
            })
            .collect::<Vec<_>>();

        Self {
            id: record.id.clone(),
            loot_randomizers,
        }
    }
}
