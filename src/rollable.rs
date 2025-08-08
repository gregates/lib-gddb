use std::ops::Range;

#[derive(Debug, Clone)]
pub struct RollableItem {
    pub id: String,
    pub weight: f32,
    pub level_range: Range<u32>,
}
