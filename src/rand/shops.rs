use super::Objects;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::Xoshiro256StarStar;

use std::collections::BTreeSet;

pub fn patch(objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
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
