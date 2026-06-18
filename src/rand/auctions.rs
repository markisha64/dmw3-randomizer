use rand_xoshiro::{rand_core::RngCore, Xoshiro256StarStar};

use crate::{json::Auction, objects::Objects, rand::shops::shoppable};

pub fn auction_items(preset: &Auction, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let mut pool = shoppable(objects, &preset.auction_items_pool);

    for auction_set in &mut objects.auction_items.modified {
        auction_set.item = pool.remove((rng.next_u64() % pool.len() as u64) as usize);
    }
}

pub fn patch(
    preset: &Auction,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    if preset.enabled && preset.auction_items {
        auction_items(preset, objects, rng);
    }

    Ok(())
}
