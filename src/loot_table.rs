use std::collections::HashMap;

use crate::affix::Affix;
use crate::affix_combo_weights::{AffixCombo, AffixComboWeights};
use crate::affix_table::AffixTable;
use crate::arz::Record;
use crate::rollable::RollableItem;
use crate::util::ensure_len;

const PREFIX_TABLE_NAME: &str = "prefixTableName";
const PREFIX_TABLE_WEIGHT: &str = "prefixTableWeight";
const PREFIX_TABLE_MIN: &str = "prefixTableLevelMin";
const PREFIX_TABLE_MAX: &str = "prefixTableLevelMax";
const SUFFIX_TABLE_NAME: &str = "suffixTableName";
const SUFFIX_TABLE_WEIGHT: &str = "suffixTableWeight";
const SUFFIX_TABLE_MIN: &str = "suffixTableLevelMin";
const SUFFIX_TABLE_MAX: &str = "suffixTableLevelMax";
const RARE_PREFIX_TABLE_NAME: &str = "rarePrefixTableName";
const RARE_PREFIX_TABLE_WEIGHT: &str = "rarePrefixTableWeight";
const RARE_PREFIX_TABLE_MIN: &str = "rarePrefixTableLevelMin";
const RARE_PREFIX_TABLE_MAX: &str = "rarePrefixTableLevelMax";
const RARE_SUFFIX_TABLE_NAME: &str = "rareSuffixTableName";
const RARE_SUFFIX_TABLE_WEIGHT: &str = "rareSuffixTableWeight";
const RARE_SUFFIX_TABLE_MIN: &str = "rareSuffixTableLevelMin";
const RARE_SUFFIX_TABLE_MAX: &str = "rareSuffixTableLevelMax";


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

impl LootTable {
    pub fn resolve<'a>(
        &self,
        level: u32,
        modifiers: &AffixComboWeights,
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(Option<&'a Affix>, Option<&'a Affix>, f64)> {
        let modified_combo_chances = (&self.combo_weights * modifiers).normalize();
        AffixCombo::iter()
            .flat_map(|combo| self.resolve_combo(level, combo, modified_combo_chances.get(combo), affix_table_lookup, affix_lookup))
            .collect::<Vec<_>>()
    }

    pub fn resolve_prefix<'a>(
        &self,
        level: u32,
        modifiers: &AffixComboWeights,
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(Option<&'a Affix>, f64)> {
        let modified_combo_chances = (&self.combo_weights * modifiers).normalize();
        let combos = AffixCombo::iter()
            .flat_map(|combo| self.resolve_combo_prefix(level, combo, modified_combo_chances.get(combo), affix_table_lookup, affix_lookup).into_iter())
            .collect::<Vec<_>>();
        Self::dedup_opt(combos)
    }

    pub fn resolve_suffix<'a>(
        &self,
        level: u32,
        modifiers: &AffixComboWeights,
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(Option<&'a Affix>, f64)> {
        let modified_combo_chances = (&self.combo_weights * modifiers).normalize();
        let combos = AffixCombo::iter()
            .flat_map(|combo| self.resolve_combo_suffix(level, combo, modified_combo_chances.get(combo), affix_table_lookup, affix_lookup))
            .collect::<Vec<_>>();
        Self::dedup_opt(combos)
    }

    fn resolve_combo_prefix<'a>(
        &self,
        level: u32,
        combo: AffixCombo,
        combo_chance: f64,
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(Option<&'a Affix>, f64)> {
        if combo_chance == 0.0 {
            return vec![];
        }
        match combo {
            AffixCombo::BrokenOnly | AffixCombo::NoPrefixNoSuffix | AffixCombo::SuffixOnly | AffixCombo::RareSuffixOnly => vec![(None, 1.0f64)],
            AffixCombo::PrefixOnly | AffixCombo::NormalPrefixRareSuffix | AffixCombo::BothPrefixSuffix => self.resolve_magic_prefix(level, affix_table_lookup, affix_lookup)
                .into_iter()
                .map(|(prefix, chance)| (Some(prefix), chance * combo_chance))
                .collect(),
            AffixCombo::RarePrefixOnly | AffixCombo::RarePrefixNormalSuffix | AffixCombo::RareBothPrefixSuffix => self.resolve_rare_prefix(level, affix_table_lookup, affix_lookup)
                .into_iter()
                .map(|(prefix, chance)| (Some(prefix), chance * combo_chance))
                .collect(),
        }
    }

    fn resolve_combo_suffix<'a>(
        &self,
        level: u32,
        combo: AffixCombo,
        combo_chance: f64,
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(Option<&'a Affix>, f64)> {
        if combo_chance == 0.0 {
            return vec![];
        }
        match combo {
            AffixCombo::BrokenOnly | AffixCombo::NoPrefixNoSuffix | AffixCombo::PrefixOnly | AffixCombo::RarePrefixOnly => vec![(None, 1.0f64)],
            AffixCombo::SuffixOnly | AffixCombo::RarePrefixNormalSuffix | AffixCombo::BothPrefixSuffix => self.resolve_magic_suffix(level, affix_table_lookup, affix_lookup)
                .into_iter()
                .map(|(suffix, chance)| (Some(suffix), chance * combo_chance))
                .collect(),
            AffixCombo::RareSuffixOnly | AffixCombo::NormalPrefixRareSuffix | AffixCombo::RareBothPrefixSuffix => self.resolve_rare_suffix(level, affix_table_lookup, affix_lookup)
                .into_iter()
                .map(|(suffix, chance)| (Some(suffix), chance * combo_chance))
                .collect(),
        }
    }

    fn resolve_combo<'a>(
        &self,
        level: u32,
        combo: AffixCombo,
        combo_chance: f64,
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(Option<&'a Affix>, Option<&'a Affix>, f64)> {
        if combo_chance == 0.0 {
            return vec![];
        }
        match combo {
            AffixCombo::BrokenOnly | AffixCombo::NoPrefixNoSuffix => vec![(None, None, 1.0f64)],
            AffixCombo::PrefixOnly => Self::dedup(self.resolve_magic_prefix(level, affix_table_lookup, affix_lookup))
                .into_iter()
                .map(|(prefix, chance)| (Some(prefix), None, chance * combo_chance))
                .collect(),
            AffixCombo::SuffixOnly => Self::dedup(self.resolve_magic_suffix(level, affix_table_lookup, affix_lookup))
                .into_iter()
                .map(|(suffix, chance)| (None, Some(suffix), chance * combo_chance))
                .collect(),
            AffixCombo::RarePrefixOnly => Self::dedup(self.resolve_rare_prefix(level, affix_table_lookup, affix_lookup))
                .into_iter()
                .map(|(prefix, chance)| (Some(prefix), None, chance * combo_chance))
                .collect(),
            AffixCombo::RareSuffixOnly => Self::dedup(self.resolve_rare_suffix(level, affix_table_lookup, affix_lookup))
                .into_iter()
                .map(|(suffix, chance)| (None, Some(suffix), chance * combo_chance))
                .collect(),
            AffixCombo::BothPrefixSuffix => {
                let prefix_chances = Self::dedup(self.resolve_magic_prefix(level, affix_table_lookup, affix_lookup));
                let suffix_chances = Self::dedup(self.resolve_magic_suffix(level, affix_table_lookup, affix_lookup));
                prefix_chances
                    .into_iter()
                    .flat_map(|(prefix, prefix_chance)| {
                        suffix_chances.iter().map(move |(suffix, suffix_chance)| (Some(prefix), Some(*suffix), prefix_chance * suffix_chance * combo_chance))
                    }).collect()
            },
            AffixCombo::NormalPrefixRareSuffix => {
                let prefix_chances = Self::dedup(self.resolve_magic_prefix(level, affix_table_lookup, affix_lookup));
                let suffix_chances = Self::dedup(self.resolve_rare_suffix(level, affix_table_lookup, affix_lookup));
                prefix_chances
                    .into_iter()
                    .flat_map(|(prefix, prefix_chance)| {
                        suffix_chances.iter().map(move |(suffix, suffix_chance)| (Some(prefix), Some(*suffix), prefix_chance * suffix_chance * combo_chance))
                    }).collect()
            },
            AffixCombo::RarePrefixNormalSuffix => {
                let prefix_chances = Self::dedup(self.resolve_rare_prefix(level, affix_table_lookup, affix_lookup));
                let suffix_chances = Self::dedup(self.resolve_magic_suffix(level, affix_table_lookup, affix_lookup));
                prefix_chances
                    .into_iter()
                    .flat_map(|(prefix, prefix_chance)| {
                        suffix_chances.iter().map(move |(suffix, suffix_chance)| (Some(prefix), Some(*suffix), prefix_chance * suffix_chance * combo_chance))
                    }).collect()
            },
            AffixCombo::RareBothPrefixSuffix => {
                let prefix_chances = Self::dedup(self.resolve_rare_prefix(level, affix_table_lookup, affix_lookup));
                let suffix_chances = Self::dedup(self.resolve_rare_suffix(level, affix_table_lookup, affix_lookup));
                prefix_chances
                    .into_iter()
                    .flat_map(|(prefix, prefix_chance)| {
                        suffix_chances.iter().map(move |(suffix, suffix_chance)| (Some(prefix), Some(*suffix), prefix_chance * suffix_chance * combo_chance))
                    }).collect()
            },
        }
    }

    fn resolve_magic_prefix<'a>(
        &self,
        level: u32,
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(&'a Affix, f64)> {
        Self::resolve_affix_tables(level, &self.prefix_tables, affix_table_lookup, affix_lookup)
    }

    fn resolve_magic_suffix<'a>(
        &self,
        level: u32,
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(&'a Affix, f64)> {
        Self::resolve_affix_tables(level, &self.suffix_tables, affix_table_lookup, affix_lookup)
    }

    fn resolve_rare_prefix<'a>(
        &self,
        level: u32,
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(&'a Affix, f64)> {
        Self::resolve_affix_tables(level, &self.rare_prefix_tables, affix_table_lookup, affix_lookup)
    }

    fn resolve_rare_suffix<'a>(
        &self,
        level: u32,
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(&'a Affix, f64)> {
        Self::resolve_affix_tables(level, &self.rare_suffix_tables, affix_table_lookup, affix_lookup)
    }

    fn resolve_affix_tables<'a>(
        level: u32,
        rollable_tables: &[RollableItem],
        affix_table_lookup: &HashMap<String, AffixTable>,
        affix_lookup: &HashMap<String, &'a Affix>,
    ) -> Vec<(&'a Affix, f64)> {
        let total = rollable_tables.iter().map(|rollable| rollable.weight as f64).sum::<f64>();
        rollable_tables
            .iter()
            .filter(|rollable| rollable.level_range.contains(&level))
            .map(|rollable| (affix_table_lookup.get(&rollable.id).unwrap(), rollable.weight as f64 / total))
            .flat_map(|(affix_table, table_chance)| affix_table.resolve(level, affix_lookup).into_iter().map(move |(affix, affix_chance)| (affix, table_chance * affix_chance)))
            .collect::<Vec<_>>()
    }

    fn dedup(results: Vec<(&Affix, f64)>) -> Vec<(&Affix, f64)> {
        let mut deduplicated: Vec<(&Affix, f64)> = vec![];
        for (affix, chance) in results.into_iter() {
            if let Some((_, prev_chance)) = deduplicated.iter_mut().find(|(dup, _)| dup.id == affix.id) {
                *prev_chance += chance;
            } else {
                deduplicated.push((affix, chance));
            }
        }
        deduplicated
    }

    fn dedup_opt(results: Vec<(Option<&Affix>, f64)>) -> Vec<(Option<&Affix>, f64)> {
        let mut deduplicated: Vec<(Option<&Affix>, f64)> = vec![];
        for (affix, chance) in results.into_iter() {
            if let Some((_, prev_chance)) = deduplicated.iter_mut().find(|(dup, _)| dup.is_none() && affix.is_none() || dup.unwrap().id == affix.unwrap().id) {
                *prev_chance += chance;
            } else {
                deduplicated.push((affix, chance));
            }
        }
        deduplicated
    }
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
            if let Ok(affix_combo) = key.parse::<AffixCombo>() {
                combo_weights.set(affix_combo, value.as_float().expect(&format!("Affix combo {} had value {:?}", key, value)));
                continue;
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
                prefix_table_ranges[i] = min..prefix_table_ranges[i].end.max(min + 1);
            } else if key.starts_with(PREFIX_TABLE_MAX) {
                let i = key[PREFIX_TABLE_MAX.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(prefix_table_ranges, i, 0..1);
                let max = value.as_int().unwrap();
                prefix_table_ranges[i] = prefix_table_ranges[i].start..max;
            } else if key.starts_with(SUFFIX_TABLE_NAME) {
                let i = key[SUFFIX_TABLE_NAME.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(suffix_tables, i, "".to_string());
                suffix_tables[i] = value.as_string().unwrap();
            } else if key.starts_with(SUFFIX_TABLE_WEIGHT) {
                let i = key[SUFFIX_TABLE_WEIGHT.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(suffix_table_weights, i, 0f32);
                suffix_table_weights[i] = value.as_float().unwrap();
            } else if key.starts_with(SUFFIX_TABLE_MIN) {
                let i = key[SUFFIX_TABLE_MIN.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(suffix_table_ranges, i, 0..1);
                let min = value.as_int().unwrap();
                suffix_table_ranges[i] = min..suffix_table_ranges[i].end.max(min + 1);
            } else if key.starts_with(SUFFIX_TABLE_MAX) {
                let i = key[SUFFIX_TABLE_MAX.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(suffix_table_ranges, i, 0..1);
                let max = value.as_int().unwrap();
                suffix_table_ranges[i] = suffix_table_ranges[i].start..max;
            } else if key.starts_with(RARE_PREFIX_TABLE_NAME) {
                let i = key[RARE_PREFIX_TABLE_NAME.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(rare_prefix_tables, i, "".to_string());
                rare_prefix_tables[i] = value.as_string().unwrap();
            } else if key.starts_with(RARE_PREFIX_TABLE_WEIGHT) {
                let i = key[RARE_PREFIX_TABLE_WEIGHT.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(rare_prefix_table_weights, i, 0f32);
                rare_prefix_table_weights[i] = value.as_float().unwrap();
            } else if key.starts_with(RARE_PREFIX_TABLE_MIN) {
                let i = key[RARE_PREFIX_TABLE_MIN.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(rare_prefix_table_ranges, i, 0..1);
                let min = value.as_int().unwrap();
                rare_prefix_table_ranges[i] = min..rare_prefix_table_ranges[i].end.max(min + 1);
            } else if key.starts_with(RARE_PREFIX_TABLE_MAX) {
                let i = key[RARE_PREFIX_TABLE_MAX.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(rare_prefix_table_ranges, i, 0..1);
                let max = value.as_int().unwrap();
                rare_prefix_table_ranges[i] = rare_prefix_table_ranges[i].start..max;
            } else if key.starts_with(RARE_SUFFIX_TABLE_NAME) {
                let i = key[RARE_SUFFIX_TABLE_NAME.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(rare_suffix_tables, i, "".to_string());
                rare_suffix_tables[i] = value.as_string().unwrap();
            } else if key.starts_with(RARE_SUFFIX_TABLE_WEIGHT) {
                let i = key[RARE_SUFFIX_TABLE_WEIGHT.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(rare_suffix_table_weights, i, 0f32);
                rare_suffix_table_weights[i] = value.as_float().unwrap();
            } else if key.starts_with(RARE_SUFFIX_TABLE_MIN) {
                let i = key[RARE_SUFFIX_TABLE_MIN.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(rare_suffix_table_ranges, i, 0..1);
                let min = value.as_int().unwrap();
                rare_suffix_table_ranges[i] = min..rare_suffix_table_ranges[i].end.max(min + 1);
            } else if key.starts_with(RARE_SUFFIX_TABLE_MAX) {
                let i = key[RARE_SUFFIX_TABLE_MAX.len()..].parse::<usize>().unwrap() - 1;
                ensure_len!(rare_suffix_table_ranges, i, 0..1);
                let max = value.as_int().unwrap();
                rare_suffix_table_ranges[i] = rare_suffix_table_ranges[i].start..max;
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
