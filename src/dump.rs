use std::fs;

use crate::rand::read_objects;

pub fn dump(path: &std::path::PathBuf) {
    let objects = read_objects(path);

    let rom_name = path.file_name().unwrap().to_str().unwrap();

    let enemy_stats_string = serde_json::to_string_pretty(&objects.enemy_stats.original).unwrap();
    let encounter_string = serde_json::to_string_pretty(&objects.encounters.original).unwrap();
    let digivolution_string =
        serde_json::to_string_pretty(&objects.digivolution_data.original).unwrap();
    let item_shop_string = serde_json::to_string_pretty(&objects.item_shop_data.original).unwrap();
    let digivolution_condition_string =
        serde_json::to_string_pretty(&objects.dv_cond.original).unwrap();
    let move_data_string = serde_json::to_string_pretty(&objects.move_data.original).unwrap();

    fs::create_dir_all(format!("dump/{rom_name}")).unwrap();

    fs::write(
        format!("dump/{rom_name}/enemy_stats.json"),
        enemy_stats_string,
    )
    .unwrap();

    fs::write(format!("dump/{rom_name}/encounters.json"), encounter_string).unwrap();

    fs::write(
        format!("dump/{rom_name}/digivolutions.json"),
        digivolution_string,
    )
    .unwrap();

    fs::write(format!("dump/{rom_name}/item_shops.json"), item_shop_string).unwrap();

    fs::write(
        format!("dump/{rom_name}/digivolution_conditions.json"),
        digivolution_condition_string,
    )
    .unwrap();

    fs::write(format!("dump/{rom_name}/move_data.json"), move_data_string).unwrap();
}
