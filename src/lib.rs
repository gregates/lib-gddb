pub mod arc;
pub mod arz;
mod buf_read_ext;
pub mod tags;

#[cfg(test)]
mod tests {
    use crate::arc::Archive;
    use crate::arz::Database;
    use crate::tags;

    #[test]
    fn parse_text_zh_item_tags() {
        let mut arc = Archive::open("resources/vanilla/Text_ZH.arc").unwrap();
        let item_tags = arc.get("tags_items.txt").unwrap();
        let item_tags = tags::parse(&item_tags.data).unwrap();
        let tag = "tagBlueprint_WeaponF007";
        assert_eq!(item_tags.get(tag).map(|s| s.as_str()), Some("设计图：收割者的防御"));
        let tag = "tagShieldB016";
        assert_eq!(item_tags.get(tag).map(|s| s.as_str()), Some("贝恩·加戈斯的碎片"));
    }

    #[test]
    fn parse_text_en_item_tags() {
        let mut arc = Archive::open("resources/vanilla/Text_EN.arc").unwrap();
        let item_tags = arc.get("tags_items.txt").unwrap();
        let item_tags = tags::parse(&item_tags.data).unwrap();
        let tag = "tagBlueprint_WeaponF007";
        assert_eq!(
            item_tags.get(tag).map(|s| s.as_str()),
            Some("Blueprint: Harvest's Defender")
        );
        let tag = "tagShieldB016";
        assert_eq!(item_tags.get(tag).map(|s| s.as_str()), Some("Bane'Gargoth's Shard"));
    }

    #[test]
    fn parse_vanilla_game_database() {
        let mut arz = Database::open("database/database.arz").unwrap();

        let raws = arz.iter_records().unwrap().collect::<Vec<_>>();

        for raw in raws.into_iter() {
            arz.resolve(raw.unwrap()).unwrap();
        }
    }

    #[test]
    fn parse_aom_game_database() {
        let mut arz = Database::open("database/GDX1.arz").unwrap();

        let raws = arz.iter_records().unwrap().collect::<Vec<_>>();

        for raw in raws.into_iter() {
            arz.resolve(raw.unwrap()).unwrap();
        }
    }

    #[test]
    fn parse_fg_game_database() {
        let mut arz = Database::open("database/GDX2.arz").unwrap();

        let raws = arz.iter_records().unwrap().collect::<Vec<_>>();

        for raw in raws.into_iter() {
            arz.resolve(raw.unwrap()).unwrap();
        }
    }
}
