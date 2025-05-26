use std::collections::HashMap;

use crate::{
    json::{CardGame, Randomizer},
    rand::Objects,
    util::shuffle,
};
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

pub fn patch(
    preset: &Randomizer,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    if preset.card_game.buy_price {
        pricing(&preset.card_game, objects, rng);
    }

    if preset.card_game.shop_items {
        shop_items(preset, objects, rng);
    }

    if preset.card_game.boosters {
        boosters(objects, rng);
    }

    Ok(())
}

fn pricing(preset: &CardGame, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    for pricing in &mut objects.card_pricing.modified {
        let min_price = preset.min_card_buy_price;
        let range = preset.max_card_buy_price - min_price + 1;

        pricing.pricing = (min_price + (rng.next_u64() % range as u64) as i64) as i16;
    }
}

fn shop_items(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let pool = Vec::from_iter(1..315);
    let mut randomized_pool = pool.clone();
    shuffle(&mut randomized_pool, preset.shuffles, rng);

    randomized_pool.truncate(objects.card_pricing.original.len());

    for i in 0..randomized_pool.len() {
        objects.card_pricing.modified[i].card_id = randomized_pool[i] as i16;
    }

    let mut randomized = randomized_pool.clone();
    shuffle(&mut randomized, preset.shuffles, rng);
    let mut j = 0;

    for i in 0..objects.card_shops.original.len() {
        if randomized.len() - j < 6 {
            j = 0;
            randomized = randomized_pool.clone();
            shuffle(&mut randomized, preset.shuffles, rng);
        }

        for k in 0..6 {
            objects.card_shop_items.modified[i * 8 + k] = randomized[j + k];
        }

        j += 6;
    }
}

fn boosters(objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let pool = Vec::from_iter(1..315);
    for i in 0..objects.booster_data.modified.len() {
        for j in 0..6 {
            let mut mapped: HashMap<u32, i32> = HashMap::new();
            let mut pool_cloned = pool.clone();

            for k in 0..16 {
                mapped
                    .entry(objects.booster_data_items.original[i * 16 * 6 + j * 16 + k])
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }

            for key in mapped.keys() {
                let new_card =
                    pool_cloned.remove((rng.next_u64() % pool_cloned.len() as u64) as usize);
                for k in 0..16 {
                    if objects.booster_data_items.modified[i * 16 * 6 + j * 16 + k] == *key {
                        objects.booster_data_items.modified[i * 16 * 6 + j * 16 + k] = new_card;
                    }
                }
            }
        }
    }
}
