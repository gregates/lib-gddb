mod arc;
mod tags;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_text_zh_item_tags() {
        let mut arc = arc::Archive::open("resources/Text_ZH.arc").unwrap();
        let item_tags = arc.get("tags_items.txt").unwrap();
        let item_tags = tags::parse(&item_tags.data).unwrap();
        let tag = "tagBlueprint_WeaponF007";
        assert_eq!(
            item_tags.get(tag).map(|s| s.as_str()),
            Some("设计图：收割者的防御")
        );
        let tag = "tagShieldB016";
        assert_eq!(
            item_tags.get(tag).map(|s| s.as_str()),
            Some("贝恩·加戈斯的碎片")
        );
    }

    #[test]
    fn parse_text_en_item_tags() {
        let mut arc = arc::Archive::open("resources/Text_EN.arc").unwrap();
        let item_tags = arc.get("tags_items.txt").unwrap();
        let item_tags = tags::parse(&item_tags.data).unwrap();
        let tag = "tagBlueprint_WeaponF007";
        assert_eq!(
            item_tags.get(tag).map(|s| s.as_str()),
            Some("Blueprint: Harvest's Defender")
        );
        let tag = "tagShieldB016";
        assert_eq!(
            item_tags.get(tag).map(|s| s.as_str()),
            Some("Bane'Gargoth's Shard")
        );
    }
}
