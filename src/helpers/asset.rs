use crate::helpers::casing::{map_asset_type, map_game};

pub fn format_assets(assets: &[serde_json::Value]) -> String {
    let mut content = String::new();
    assets.iter().take(5).for_each(|asset| {
        let asset_name = asset["name"]
            .as_str()
            .unwrap()
            .to_string()
            .replace(".png", "");
        let url = format!(
            "https://wanderer.moe/asset/{}",
            asset["id"].as_u64().unwrap()
        );
        let game = map_game(asset["game"].as_str().unwrap());
        let asset_category = map_asset_type(asset["asset"].as_str().unwrap());
        let uploaded_date = asset["uploadedDate"].as_str().unwrap().to_string();
        content.push_str(&format!(
            "**{}** ({}: {}) \n<{}>\nUploaded {}\n\n",
            asset_name, game, asset_category, url, uploaded_date
        ));
    });
    content
}

pub fn format_asset(asset: &serde_json::Value) -> String {
    if let Some(asset) = asset.as_object() {
        let asset_name = asset["name"]
            .as_str()
            .unwrap()
            .to_string()
            .replace(".png", "");
        let url = format!(
            "https://wanderer.moe/asset/{}",
            asset["id"].as_u64().unwrap()
        );
        let game = map_game(asset["game"].as_str().unwrap());
        let asset_category = map_asset_type(asset["asset"].as_str().unwrap());
        let uploaded_date = asset["uploadedDate"].as_str().unwrap().to_string();
        format!(
            "**{}** ({}: {}) \n<{}>\nUploaded {}\n\n",
            asset_name, game, asset_category, url, uploaded_date
        )
    } else {
        "Invalid asset format".to_string()
    }
}
