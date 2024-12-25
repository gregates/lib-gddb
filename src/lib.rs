pub mod affix;
pub mod affix_table;
pub mod arc;
pub mod arz;
mod buf_read_ext;
pub mod tags;

#[cfg(test)]
mod tests {
    use crate::affix::{Affix, PREFIX_PATH, SUFFIX_PATH};
    use crate::arc::Archive;
    use crate::arz::{Database, DatabaseValue};
    use crate::tags;

    const DB_GD: &str = "database/1.2.1.3/database.arz";
    const DB_AOM: &str = "database/1.2.1.3/GDX1.arz";
    const DB_FG: &str = "database/1.2.1.3/GDX2.arz";

    const TAGS_GD: &str = "resources/1.2.1.3/Text_EN.arc";
    const TAGS_AOM: &str = "resources/1.2.1.3/Text_EN_gdx1.arc";
    const TAGS_FG: &str = "resources/1.2.1.3/Text_EN_gdx2.arc";

    #[test]
    fn parse_text_zh_item_tags() {
        let mut arc = Archive::open("resources/1.2.1.3/Text_ZH.arc").unwrap();
        let item_tags = arc.get("tags_items.txt").unwrap();
        let item_tags = tags::parse(&item_tags.data).unwrap();
        let tag = "tagBlueprint_WeaponF007";
        assert_eq!(item_tags.get(tag).map(|s| s.as_str()), Some("设计图：收割者的防御"));
        let tag = "tagShieldB016";
        assert_eq!(item_tags.get(tag).map(|s| s.as_str()), Some("贝恩·加戈斯的碎片"));
    }

    #[test]
    fn parse_text_en_item_tags() {
        let mut arc = Archive::open(TAGS_GD).unwrap();
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
        let mut arz = Database::open(DB_GD).unwrap();

        let raws = arz.iter_records().unwrap().collect::<Vec<_>>();

        for raw in raws.into_iter() {
            arz.resolve(raw.unwrap()).unwrap();
        }
    }

    #[test]
    fn parse_aom_game_database() {
        let mut arz = Database::open(DB_AOM).unwrap();

        let raws = arz.iter_records().unwrap().collect::<Vec<_>>();

        for raw in raws.into_iter() {
            arz.resolve(raw.unwrap()).unwrap();
        }
    }

    #[test]
    fn parse_fg_game_database() {
        let mut arz = Database::open(DB_FG).unwrap();

        let raws = arz.iter_records().unwrap().collect::<Vec<_>>();

        for raw in raws.into_iter() {
            arz.resolve(raw.unwrap()).unwrap();
        }
    }

    #[test]
    fn parse_affixes() {
        let tags = [TAGS_GD, TAGS_AOM, TAGS_FG]
            .into_iter()
            .enumerate()
            .map(|(i, path)| {
                let mut arc = Archive::open(path).unwrap();
                let filename = if i > 0 {
                    format!("tagsgdx{}_items.txt", i)
                } else {
                    "tags_items.txt".to_string()
                };
                let item_tags = arc.get(filename.as_str()).unwrap();
                tags::parse(&item_tags.data).unwrap()
            })
            .reduce(|mut acc, tags| {
                acc.extend(tags.into_iter());
                acc
            })
            .unwrap();

        let mut prefixes = vec![];
        let mut prefix_tables = vec![];
        let mut suffixes = vec![];
        let mut suffix_tables = vec![];
        for path in [DB_GD, DB_AOM, DB_FG].into_iter() {
            let mut arz = Database::open(path).unwrap();

            let raws = arz.iter_records().unwrap().collect::<Result<Vec<_>, _>>().unwrap();

            for raw in raws.into_iter() {
                let id = arz.record_id(&raw).unwrap();
                if id.starts_with(PREFIX_PATH) {
                    let resolved = arz.resolve(raw).unwrap();
                    if id.contains("prefixtables") {
                        assert_eq!(
                            DatabaseValue::String("LootRandomizerTable".to_string()),
                            resolved.data.get("Class").cloned().unwrap(),
                        );
                        prefix_tables.push(resolved);
                    } else {
                        assert_eq!(
                            DatabaseValue::String("LootRandomizer".to_string()),
                            resolved.data.get("Class").cloned().unwrap(),
                        );
                        prefixes.push(resolved);
                    }
                } else if id.starts_with(SUFFIX_PATH) {
                    let resolved = arz.resolve(raw).unwrap();
                    if id.contains("suffixtables") {
                        assert_eq!(
                            DatabaseValue::String("LootRandomizerTable".to_string()),
                            resolved.data.get("Class").cloned().unwrap(),
                        );
                        suffix_tables.push(resolved);
                    } else {
                        assert_eq!(
                            DatabaseValue::String("LootRandomizer".to_string()),
                            resolved.data.get("Class").cloned().unwrap(),
                        );
                        suffixes.push(resolved);
                    }
                }
            }
        }

        let demonic = prefixes
            .into_iter()
            .map(|record| Affix::from(record))
            .filter(|affix| affix.localize(&tags).as_str() == "Demonic")
            .collect::<Vec<_>>();

        assert!(demonic.len() > 0);
    }
}
