use crate::rand::Objects;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use crate::json::Shops;
use std::collections::BTreeSet;

pub fn patch(preset: &Shops, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    match preset.limit_shop_items {
        Some(limit) => {
            randomize_limited(&limit, objects, rng);
        }
        None => {
            randomize_existing(objects, rng);
        }
    }
}

fn randomize_limited(limit: &u8, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let mut shoppable: BTreeSet<u16> = BTreeSet::new();

    for item in &objects.shop_items.original {
        if *item != 0 {
            shoppable.insert(*item);
        }
    }

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

fn randomize_existing(objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let mut shoppable: BTreeSet<u16> = BTreeSet::new();

    for item in &objects.shop_items.original {
        if *item != 0 {
            shoppable.insert(*item);
        }
    }

    let mut shoppable_arr = Vec::from_iter(shoppable.clone().into_iter());
    for item in &mut objects.shop_items.modified {
        if *item == 0 {
            shoppable_arr = Vec::from_iter(shoppable.clone().into_iter());
            continue;
        }

        *item = shoppable_arr.remove((rng.next_u64() % shoppable_arr.len() as u64) as usize);
    }
}
