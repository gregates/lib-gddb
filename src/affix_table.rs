use std::ops::Range;

use crate::arz::Record;

const NAME: &str = "randomizerName";
const WEIGHT: &str = "randomizerWeight";
const MIN: &str = "randomizerLevelMin";
const MAX: &str = "randomizerLevelMax";

#[derive(Debug, Clone)]
pub struct AffixTable {
    pub id: String,
    pub affixes: Vec<String>,
    pub weights: Vec<f32>,
    pub ranges: Vec<Range<u32>>,
}

impl From<&Record> for AffixTable {
    fn from(record: &Record) -> Self {
        let estimated_len = record.data.len() / 4;
        let mut affixes = vec!["".to_string(); estimated_len];
        let mut weights = vec![0f32; estimated_len];
        let mut ranges = vec![0..1; estimated_len];
        for (key, value) in record.data.iter() {
            if key.starts_with(NAME) {
                let i = key[NAME.len()..].parse::<usize>().unwrap() - 1;
                if i > affixes.len() {
                    affixes.extend_from_slice(&vec!["".to_string(); i - affixes.len()])
                };
                affixes[i] = value.as_string().unwrap();
            } else if key.starts_with(WEIGHT) {
                let i = key[WEIGHT.len()..].parse::<usize>().unwrap() - 1;
                if i > weights.len() {
                    weights.extend_from_slice(&vec![0f32; i - weights.len()])
                };
                weights[i] = value.as_float().unwrap();
            } else if key.starts_with(MIN) {
                let i = key[MIN.len()..].parse::<usize>().unwrap() - 1;
                if i > ranges.len() {
                    ranges.extend_from_slice(&vec![0..1; i - ranges.len()])
                };
                let min = value.as_int().unwrap();
                ranges[i] = min..ranges[i].end.min(min + 1);
            } else if key.starts_with(MAX) {
                let i = key[MAX.len()..].parse::<usize>().unwrap() - 1;
                if i > ranges.len() {
                    ranges.extend_from_slice(&vec![0..1; i - ranges.len()])
                };
                let max = value.as_int().unwrap();
                ranges[i] = ranges[i].start..max;
            }
        }

        Self {
            id: record.id.clone(),
            affixes,
            weights,
            ranges,
        }
    }
}
