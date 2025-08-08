pub mod affix;
pub mod affix_combo_weights;
pub mod affix_table;
pub mod arc;
pub mod arz;
mod buf_read_ext;
pub mod loot_table;
pub mod rollable;
pub mod tags;
mod util;

pub enum Difficulty {
    Normal,
    Elite,
    Ultimate,
}

pub enum MobClass {
    Common,
    Champion,
    Hero,
    Boss,
}

pub enum ChallengeLayer {
    Easy,
    Hard,
    RogueLike,
    EndlessDungeon,
}
