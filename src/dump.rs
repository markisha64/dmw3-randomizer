use std::fs;

use binwrite::BinWrite;

use crate::rand::read_objects;

pub async fn dump(path: &std::path::PathBuf) {
    let objects = read_objects(path).await;

    let rom_name = path.file_name().unwrap().to_str().unwrap();

    fs::create_dir_all(format!("dump/{rom_name}")).unwrap();

    let mut enemy_stats_bytes = Vec::new();
    let mut encounter_bytes = Vec::new();
    let mut digivolution_bytes = Vec::new();
    let mut rookie_bytes = Vec::new();
    let mut item_shop_bytes = Vec::new();
    let mut digivolution_condition_bytes = Vec::new();
    let mut move_data_bytes = Vec::new();

    let _ = &objects
        .enemy_stats
        .original
        .write(&mut enemy_stats_bytes)
        .unwrap();

    let _ = &objects
        .encounters
        .original
        .write(&mut encounter_bytes)
        .unwrap();

    let _ = &objects
        .digivolution_data
        .original
        .write(&mut digivolution_bytes)
        .unwrap();

    let _ = &objects
        .rookie_data
        .original
        .write(&mut rookie_bytes)
        .unwrap();

    let _ = &objects
        .item_shop_data
        .original
        .write(&mut item_shop_bytes)
        .unwrap();

    let _ = &objects
        .dv_cond
        .original
        .write(&mut digivolution_condition_bytes)
        .unwrap();

    let _ = &objects
        .move_data
        .original
        .write(&mut move_data_bytes)
        .unwrap();

    fs::write(format!("dump/{rom_name}/enemy_stats"), enemy_stats_bytes).unwrap();

    fs::write(format!("dump/{rom_name}/encounters"), encounter_bytes).unwrap();

    fs::write(format!("dump/{rom_name}/digivolutions"), digivolution_bytes).unwrap();

    fs::write(format!("dump/{rom_name}/rookies"), rookie_bytes).unwrap();

    fs::write(format!("dump/{rom_name}/item_shops"), item_shop_bytes).unwrap();

    fs::write(
        format!("dump/{rom_name}/digivolution_conditions"),
        digivolution_condition_bytes,
    )
    .unwrap();

    fs::write(format!("dump/{rom_name}/move_data"), move_data_bytes).unwrap();
}
