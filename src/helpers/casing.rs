use std::collections::HashMap;

pub fn map_game(game: &str) -> String {
    let mut game_mapping = HashMap::new();
    game_mapping.insert("honkai-star-rail", "Honkai: Star Rail");
    game_mapping.insert("goddess-of-victory-nikke", "Goddess of Victory: Nikke");
    game_mapping.insert("project-sekai", "Project SEKAI COLORFUL STAGE!");
    game_mapping.insert("sino-alice", "SINoALICE");
    game_mapping.insert("cookie-run", "Cookie Run: Kingdom");

    game_mapping
        .get(game)
        .map_or_else(|| fix_casing(game), |s| s.to_string())
}

pub fn map_asset_type(asset_type: &str) -> String {
    let mut asset_type_mapping = HashMap::new();
    asset_type_mapping.insert("tcg", "TCG");
    asset_type_mapping.insert("tcg-sheets", "TCG Sheets");

    asset_type_mapping
        .get(asset_type)
        .map_or_else(|| fix_casing(asset_type), |s| s.to_string())
}

pub fn fix_casing(s: &str) -> String {
    s.split(|c: char| c == '-' || c == ' ')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
