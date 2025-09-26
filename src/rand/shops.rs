use crate::rand::Objects;
use anyhow::Context;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::{ShopItems, Shops};
use std::collections::BTreeSet;

pub fn patch(
    preset: &Shops,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    let shoppable = shoppable(objects, preset);

    match preset.limit_shop_items_enabled {
        true => {
            randomize_limited(&preset.limit_shop_items, objects, rng, shoppable)?;
        }
        false => {
            randomize_existing(objects, rng, shoppable);
        }
    }

    if preset.sell_price {
        randomize_sell_price(preset, objects, rng);
    }

    let len = objects.item_shop_data.modified.len();
    for i in 1..len {
        objects.item_shop_data.modified[i].buy_price =
            2 * objects.item_shop_data.modified[i].sell_price;
    }

    Ok(())
}

fn randomize_sell_price(preset: &Shops, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let min_price = preset.min_sell_price;
    let range = preset.max_sell_price - min_price + 1;

    let len = objects.item_shop_data.modified.len();
    for i in 1..len {
        let item = &mut objects.item_shop_data.modified[i];

        if item.sell_price != 0 && !(i as u16 == dmw3_consts::TNT_BALL_ID && preset.keep_tnt) {
            item.sell_price = min_price as u16 + (rng.next_u64() % range as u64) as u16;
        }
    }
}

fn shoppable(objects: &mut Objects, preset: &Shops) -> BTreeSet<u16> {
    let len = objects.item_shop_data.original.len();

    let mut shoppable: BTreeSet<u16> = BTreeSet::new();

    match preset.items_only {
        ShopItems::Buyable => {
            for i in 1..len {
                if objects.item_shop_data.original[i].buy_price > 0 {
                    shoppable.insert(i as u16);
                }
            }
        }
        ShopItems::Sellable => {
            for i in 1..len {
                if objects.item_shop_data.original[i].sell_price > 0 {
                    shoppable.insert(i as u16);
                }
            }
        }
    }

    shoppable
}

fn randomize_limited(
    limit: &u8,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
    shoppable: BTreeSet<u16>,
) -> anyhow::Result<()> {
    let mut ptr = objects.shops.modified.first().context("empty shops")?.items;
    for i in 0..objects.shops.original.len() {
        let mut shoppable_arr = Vec::from_iter(shoppable.clone().into_iter());
        let shop = &mut objects.shops.modified[i];
        let limit_deref = *limit;

        // limit + 1 (because of blank item)
        let lp1 = (limit_deref + 1) as usize;

        shop.items = ptr;
        shop.item_count = limit_deref as u32;

        for j in 0..limit_deref as usize {
            objects.shop_items.modified[i * lp1 + j] =
                shoppable_arr.remove((rng.next_u64() % shoppable_arr.len() as u64) as usize);
        }

        objects.shop_items.modified[i * lp1 + limit_deref as usize] = 0;
        ptr.value += lp1 as u32 * 2;
    }

    Ok(())
}

fn randomize_existing(
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
    shoppable: BTreeSet<u16>,
) {
    let mut shoppable_arr = Vec::from_iter(shoppable.clone());
    for item in &mut objects.shop_items.modified {
        if *item == 0 {
            shoppable_arr = Vec::from_iter(shoppable.clone().into_iter());
            continue;
        }

        *item = shoppable_arr.remove((rng.next_u64() % shoppable_arr.len() as u64) as usize);
    }
}
