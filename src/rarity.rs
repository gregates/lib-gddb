use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rarity {
    Unknown,
    Common,
    Magic,
    Rare,
    Legendary
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "err: unknown value for affix rarity"),
            Self::Common => write!(f, "common"),
            Self::Magic => write!(f, "magic"),
            Self::Rare => write!(f, "rare"),
            Self::Legendary => write!(f, "legendary"),
        }
    }
}

impl FromStr for Rarity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Common" | "common" => Ok(Self::Common),
            "Magical" | "magical" => Ok(Self::Magic),
            "Rare" | "rare" => Ok(Self::Rare),
            "Legendary" | "legendary" => Ok(Self::Rare),
            _ => Ok(Self::Unknown),
        }
    }
}
