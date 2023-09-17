use crate::rand::Objects;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::{ShopItems, Shops};
use std::collections::BTreeSet;

pub fn patch(preset: &Shops, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let shoppable = shoppable(objects, preset);

    match preset.limit_shop_items {
        Some(limit) => {
            randomize_limited(&limit, objects, rng, shoppable);
        }
        None => {
            randomize_existing(objects, rng, shoppable);
        }
    }

    let len = objects.item_shop_data.modified.len();
    for i in 1..len - 1 {
        objects.item_shop_data.modified[i].buy_price =
            2 * objects.item_shop_data.modified[i].sell_price;
    }
}

fn shoppable(objects: &mut Objects, preset: &Shops) -> BTreeSet<u16> {
    let len = objects.item_shop_data.original.len();

    let mut shoppable: BTreeSet<u16> = BTreeSet::new();

    match preset.items_only {
        ShopItems::Buyable => {
            for i in 1..len - 1 {
                if objects.item_shop_data.original[i].buy_price > 0 {
                    shoppable.insert(i as u16);
                }
            }
        }
        ShopItems::Sellable => {
            for i in 1..len - 1 {
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
) {
    let mut ptr = objects.shops.modified.first().unwrap().items.clone();
    for i in 0..objects.shops.original.len() {
        let mut shoppable_arr = Vec::from_iter(shoppable.clone().into_iter());
        let shop = &mut objects.shops.modified[i];
        let limit_deref = *limit;

        shop.items = ptr.clone();
        shop.item_count = limit_deref as u32;

        for j in 0..limit_deref as usize {
            objects.shop_items.modified[i * 9 + j] =
                shoppable_arr.remove((rng.next_u64() % shoppable_arr.len() as u64) as usize);
        }

        objects.shop_items.modified[i * 9 + limit_deref as usize] = 0;
        ptr.value += (limit_deref as u32 + 1) * 2;
    }
}

fn randomize_existing(
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
    shoppable: BTreeSet<u16>,
) {
    let mut shoppable_arr = Vec::from_iter(shoppable.clone().into_iter());
    for item in &mut objects.shop_items.modified {
        if *item == 0 {
            shoppable_arr = Vec::from_iter(shoppable.clone().into_iter());
            continue;
        }

        *item = shoppable_arr.remove((rng.next_u64() % shoppable_arr.len() as u64) as usize);
    }
}
