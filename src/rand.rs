use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use std::path::PathBuf;

use crate::json::Preset;
use crate::json::TNTStrategy;
use crate::objects::fix_lba;
use crate::objects::read_objects;
use crate::objects::write_objects;
use crate::objects::Objects;

pub use dmw3_structs;

mod card_game;
mod encounters;
mod fixes;
pub mod maps;
mod models;
mod parties;
mod party_exp_bits;
mod scaling;
mod shops;

pub async fn patch(path: &PathBuf, preset: &Preset) -> anyhow::Result<Objects> {
    let mut objects = read_objects(path).await?;

    let mut rng = Xoshiro256StarStar::seed_from_u64(preset.randomizer.seed);

    if preset.randomizer.encounters.enabled {
        encounters::patch(&preset.randomizer, &mut objects, &mut rng)?;
    }

    if preset.randomizer.parties.enabled {
        parties::patch(&preset.randomizer, &mut objects, &mut rng)?;
    }

    if preset.scaling.enabled {
        scaling::patch(&preset.scaling, &mut objects, &mut rng)?;
    }

    if preset.fixes.scaling {
        fixes::scaling(&mut objects);
    }

    if preset.randomizer.shops.enabled {
        shops::patch(&preset.randomizer.shops, &mut objects, &mut rng)?;
    }

    if preset.randomizer.encounters.strategy == TNTStrategy::Ironmon {
        shops::tnt_ironmon(&mut objects);
    }

    if preset.randomizer.card_game.enabled {
        card_game::patch(&preset.randomizer, &mut objects, &mut rng)?;
    }

    if preset.randomizer.maps.enabled {
        maps::patch(&preset.randomizer, &mut objects, &mut rng)?;
    }

    if preset.randomizer.models.enabled {
        models::patch(&preset.randomizer, &mut objects, &mut rng)?;
    }

    if preset.party_exp_bits.enabled {
        party_exp_bits::patch(&preset.party_exp_bits, &mut objects)?;
    }

    // update all files on disk
    write_objects(path, &mut objects).await?;

    fix_lba(path, &mut objects).await?;

    Ok(objects)
}
