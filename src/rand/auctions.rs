use anyhow::Context;
use dmw3_consts::{AUCTION_COUNT, OINKMON_AUCTION_IDX};
use dmw3_structs::ScriptConditionStep;
use rand_xoshiro::{rand_core::RngCore, Xoshiro256StarStar};

use crate::{json::Auction, objects::Objects, rand::shops::shoppable};

pub const DEFAULT_AUCTION_PRICES: [[u32; 5]; 16] = [
    [400, 500, 600, 700, 800],
    [400, 500, 600, 700, 800],
    [800, 1100, 1300, 1500, 1600],
    [800, 1100, 1300, 1500, 1600],
    [1600, 2000, 2300, 2600, 2700],
    [1600, 2000, 2300, 2600, 2700],
    [2700, 3100, 3500, 3900, 4000],
    [2700, 3100, 3500, 3900, 4000],
    [4000, 4500, 5500, 5900, 6000],
    [4000, 4500, 5500, 5900, 6000],
    [7000, 8000, 8300, 8600, 8700],
    [7000, 8000, 8300, 8600, 8700],
    [10000, 10900, 11200, 11400, 11500],
    [16000, 16900, 17000, 17400, 17500],
    [23000, 23500, 23800, 23900, 24000],
    [30000, 31000, 31500, 31900, 32000],
];

fn auction_items(preset: &Auction, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let mut pool = shoppable(objects, &preset.auction_items_pool);

    for auction_set in &mut objects.auction_items.modified {
        match &mut auction_set.item {
            ScriptConditionStep::Step {
                value,
                condition_type: _condition_type,
                flag: _flag,
            } => *value = pool.remove((rng.next_u64() % pool.len() as u64) as usize),
            ScriptConditionStep::EndStep => unreachable!("EndStep shouldn't be in mapped"),
        };
    }
}

fn full_prices(objects: &Objects) -> [[u32; 5]; 16] {
    let mut result = [[0; 5]; 16];

    for j in 0..5 {
        result[0][j] = objects.bits_checks.modified[0] * (500 + (j as u32) * 125) / 1000;
        result[1][j] = objects.bits_checks.modified[0] * (500 + (j as u32) * 125) / 1000;
        result[2][j] = objects.bits_checks.modified[1] * (500 + (j as u32) * 125) / 1000;
        result[3][j] = objects.bits_checks.modified[1] * (500 + (j as u32) * 125) / 1000;
        result[4][j] = objects.bits_checks.modified[2] * (500 + (j as u32) * 125) / 1000;
        result[5][j] = objects.bits_checks.modified[2] * (500 + (j as u32) * 125) / 1000;
        result[6][j] = objects.bits_checks.modified[3] * (500 + (j as u32) * 125) / 1000;
        result[7][j] = objects.bits_checks.modified[3] * (500 + (j as u32) * 125) / 1000;
        result[8][j] = objects.bits_checks.modified[4] * (500 + (j as u32) * 125) / 1000;
        result[9][j] = objects.bits_checks.modified[4] * (500 + (j as u32) * 125) / 1000;
        result[10][j] = objects.bits_checks.modified[5] * (500 + (j as u32) * 125) / 1000;
        result[11][j] = objects.bits_checks.modified[5] * (500 + (j as u32) * 125) / 1000;
        result[12][j] = objects.bits_checks.modified[6] * (500 + (j as u32) * 125) / 1000;
        result[13][j] = objects.bits_checks.modified[7] * (500 + (j as u32) * 125) / 1000;
        result[14][j] = objects.bits_checks.modified[8] * (500 + (j as u32) * 125) / 1000;
        result[15][j] = objects.bits_checks.modified[9] * (500 + (j as u32) * 125) / 1000;
    }

    result
}

fn auction_text(preset: &Auction, objects: &mut Objects) -> anyhow::Result<()> {
    let prices = match preset.auction_values {
        true => &full_prices(objects),
        false => &DEFAULT_AUCTION_PRICES,
    };

    for lang in objects.executable.languages() {
        // 15 normal and 1 Oinkmon
        for i in 0..AUCTION_COUNT {
            let item = match objects.auction_items.modified[i].item {
                ScriptConditionStep::Step {
                    value,
                    condition_type: _condition_type,
                    flag: _flag,
                } => value,
                ScriptConditionStep::EndStep => unreachable!("EndStep shouldn't be in mapped"),
            };

            let item_name = &objects
                .items
                .files
                .get(lang)
                .context("failed to get by lang")?
                .file
                .files[item as usize];

            let cutscene_text = objects
                .cargo_tower_text
                .get_mut(lang)
                .context("missing language")?;

            cutscene_text[i * 4] = lang.auction_intro_text(item_name.as_slice(), &prices[i]);
            cutscene_text[i * 4 + 1] = lang.auction_bid_text(prices[i][4]);
            cutscene_text[i * 4 + 2] = lang.auction_win_text(item_name.as_slice(), 0, prices[i][4]);
            cutscene_text[i * 4 + 3] = lang.auction_pass_text();
        }

        // oinkmon auction
        let cutscene_text = objects
            .cargo_tower_text
            .get_mut(lang)
            .context("missing language")?;

        cutscene_text[60] = lang.oinkmon_intro_text(&prices[OINKMON_AUCTION_IDX]);
        cutscene_text[61] = lang.oinkmon_bid_text(prices[OINKMON_AUCTION_IDX][4]);
        cutscene_text[62] = lang.oinkmon_win_text(prices[OINKMON_AUCTION_IDX][4]);
        cutscene_text[63] = lang.oinkmon_pass_text();
    }

    Ok(())
}

fn auction_values(preset: &Auction, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    for i in 0..10 {
        let new_value = preset.auction_values_min
            + rng.next_u32() % (preset.auction_values_max - preset.auction_values_min + 1);

        objects.bits_checks.modified[i] = new_value;
        objects.bits_subtracts.modified[i] = new_value;
    }
}

pub fn patch(
    preset: &Auction,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    if preset.auction_items {
        auction_items(preset, objects, rng);
    }

    if preset.auction_values {
        auction_values(preset, objects, rng);
    }

    if preset.auction_items || preset.auction_items {
        auction_text(preset, objects)?;
    }

    Ok(())
}
