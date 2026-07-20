pub mod affix;
pub mod affix_combo_weights;
pub mod affix_table;
pub mod arc;
pub mod arz;
pub mod ascension_affix_swap;
mod buf_read_ext;
pub mod item;
pub mod loot_table;
pub mod rarity;
pub mod rollable;
pub mod tags;
mod util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Difficulty {
    Normal,
    Elite,
    Ultimate,
}

impl Difficulty {
    pub fn parse_prefix(s: &str) -> (Option<Difficulty>, &str) {
        if let Some(trailing) = s.strip_prefix("Ultimate") {
            return (Some(Self::Ultimate), trailing);
        }
        if let Some(trailing) = s.strip_prefix("Elite") {
            return (Some(Self::Elite), trailing);
        }
        (None, s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MobClass {
    Common,
    Champion,
    Hero,
    Boss,
}

#[derive(Debug)]
pub struct UnexpectedMobClass;

impl std::str::FromStr for MobClass {
    type Err = UnexpectedMobClass;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mob = match s {
            "Common" => MobClass::Common,
            "Champion" => MobClass::Champion,
            "Hero" => MobClass::Hero,
            "Boss" => MobClass::Boss,
            _ => return Err(UnexpectedMobClass),
        };
        Ok(mob)
    }
}

pub enum ChallengeLayer {
    Easy,
    Hard,
    RogueLike,
    EndlessDungeon,
}
