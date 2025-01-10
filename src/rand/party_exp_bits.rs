use crate::{json::PartyExpBits, rand::Objects};

pub fn patch(preset: &PartyExpBits, objects: &mut Objects) -> anyhow::Result<()> {
    for party_exp_bits in objects.party_exp_bits.modified.iter_mut() {
        party_exp_bits.dv_exp = ((party_exp_bits.dv_exp as f64) * preset.dv_exp_modifier) as u32;
        party_exp_bits.exp = ((party_exp_bits.exp as f64) * preset.exp_modifier) as u32;
        party_exp_bits.bits = ((party_exp_bits.bits as f64) * preset.bits_modifier) as u32;
    }

    Ok(())
}
