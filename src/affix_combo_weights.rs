const AFFIX_COMBOS_IN_ORDER: [AffixCombo; 10] = [
    AffixCombo::BrokenOnly,
    AffixCombo::NoPrefixNoSuffix,
    AffixCombo::SuffixOnly,
    AffixCombo::PrefixOnly,
    AffixCombo::BothPrefixSuffix,
    AffixCombo::RarePrefixOnly,
    AffixCombo::RareSuffixOnly,
    AffixCombo::RarePrefixNormalSuffix,
    AffixCombo::NormalPrefixRareSuffix,
    AffixCombo::RareBothPrefixSuffix,
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AffixCombo {
    BrokenOnly,
    NoPrefixNoSuffix,
    SuffixOnly,
    PrefixOnly,
    BothPrefixSuffix,
    RarePrefixOnly,
    RareSuffixOnly,
    RarePrefixNormalSuffix,
    NormalPrefixRareSuffix,
    RareBothPrefixSuffix,
}

#[derive(Debug)]
pub struct UnknownAffixCombo;

impl std::str::FromStr for AffixCombo {
    type Err = UnknownAffixCombo;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = match s {
            "brokenOnly" => Self::BrokenOnly,
            "noPrefixNoSuffix" => Self::NoPrefixNoSuffix,
            "suffixOnly" => Self::SuffixOnly,
            "prefixOnly" => Self::PrefixOnly,
            "bothPrefixSuffix" => Self::BothPrefixSuffix,
            "rarePrefixOnly" => Self::RarePrefixOnly,
            "rareSuffixOnly" => Self::RareSuffixOnly,
            "rarePrefixNormalSuffix" => Self::RarePrefixNormalSuffix,
            "normalPrefixRareSuffix" => Self::NormalPrefixRareSuffix,
            "rareBothPrefixSuffix" => Self::RareBothPrefixSuffix,
            _ => return Err(UnknownAffixCombo),
        };
        Ok(parsed)
    }
}

impl std::fmt::Display for AffixCombo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::BrokenOnly => write!(f, "brokenOnly"),
            Self::NoPrefixNoSuffix => write!(f, "noPrefixNoSuffix"),
            Self::SuffixOnly => write!(f, "suffixOnly"),
            Self::PrefixOnly => write!(f, "prefixOnly"),
            Self::BothPrefixSuffix => write!(f, "bothPrefixSuffix"),
            Self::RarePrefixOnly => write!(f, "rarePrefixOnly"),
            Self::RareSuffixOnly => write!(f, "rareSuffixOnly"),
            Self::RarePrefixNormalSuffix => write!(f, "rarePrefixNormalSuffix"),
            Self::NormalPrefixRareSuffix => write!(f, "normalPrefixRareSuffix"),
            Self::RareBothPrefixSuffix => write!(f, "rareBothPrefixSuffix"),
        }
    }
}

impl AffixCombo {
    pub fn index(&self) -> usize {
        match self {
            Self::BrokenOnly => 0,
            Self::NoPrefixNoSuffix => 1,
            Self::SuffixOnly => 2,
            Self::PrefixOnly => 3,
            Self::BothPrefixSuffix => 4,
            Self::RarePrefixOnly => 5,
            Self::RareSuffixOnly => 6,
            Self::RarePrefixNormalSuffix => 7,
            Self::NormalPrefixRareSuffix => 8,
            Self::RareBothPrefixSuffix => 9,
        }
    }

    pub fn from_index(i: usize) -> Self {
        AFFIX_COMBOS_IN_ORDER[i]
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        AFFIX_COMBOS_IN_ORDER.iter().copied()
    }
}

#[derive(Debug, Clone)]
pub struct AffixComboWeights([f32; 10]);

impl Default for AffixComboWeights {
    fn default() -> Self {
        Self([1f32; 10])
    }
}

impl AffixComboWeights {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: AffixCombo, weight: f32) {
        self.0[key.index()] = weight;
    }

    pub fn get(&self, key: AffixCombo) -> f32 {
        self.0[key.index()]
    }

    pub fn normalize(&self) -> Self {
        let total = self.0.iter().sum::<f32>();
        let mut new = [0f32; 10];
        for i in 0..10 {
            new[i] = self.0[i] / total;
        }
        AffixComboWeights(new)
    }
}

impl std::ops::Mul for &AffixComboWeights {
    type Output = AffixComboWeights;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new = [0f32; 10];
        for i in 0..self.0.len() {
            new[i] = self.0[i] * rhs.0[i];
        }
        AffixComboWeights(new)
    }
}
