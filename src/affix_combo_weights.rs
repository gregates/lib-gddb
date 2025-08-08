use crate::{ChallengeLayer, Difficulty, MobClass};

#[derive(Default, Debug, Clone)]
pub struct AffixComboWeights {
    broken: f32,
    none: f32,
    magic_suffix_only: f32,
    magic_prefix_only: f32,
    magic_both: f32,
    rare_prefix_only: f32,
    rare_suffix_only: f32,
    rare_prefix_magic_suffix: f32,
    magic_prefix_rare_suffix: f32,
    rare_both: f32,
}

pub struct AffixComboModifiers {
    difficulty: Difficulty,
    challenge_layer: ChallengeLayer,
    mob_class: MobClass,
    chest: bool,
    weights: AffixComboWeights,
}

impl AffixComboWeights {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if the key was recognized, otherwise false
    pub fn set(&mut self, key: &str, weight: f32) -> bool {
        match key {
            "brokenOnly" => self.broken = weight,
            "noPrefixNoSuffix" => self.none = weight,
            "suffixOnly" => self.magic_suffix_only = weight,
            "prefixOnly" => self.magic_prefix_only = weight,
            "bothPrefixSuffix" => self.magic_both = weight,
            "rarePrefixNormalSuffix" => self.rare_prefix_magic_suffix = weight,
            "normalPrefixRareSuffix" => self.magic_prefix_rare_suffix = weight,
            "rarePrefixOnly" => self.rare_prefix_only = weight,
            "rareSuffixOnly" => self.rare_suffix_only = weight,
            "rareBothPrefixSuffix" => self.rare_both = weight,
            _ => return false,
        }
        true
    }
}
