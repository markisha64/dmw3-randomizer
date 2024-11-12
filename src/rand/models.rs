use rand_xoshiro::{rand_core::RngCore, Xoshiro256StarStar};
use rlen::{rlen_decode, rlen_encode};
use tim::Tim;

use crate::{json::Randomizer, rand::Objects};

use super::ModelObject;

fn hue(
    preset: &Randomizer,
    model_objects: &mut Vec<ModelObject>,
    rng: &mut Xoshiro256StarStar,
    last_n_rows: usize,
    row_skip: usize,
) -> anyhow::Result<()> {
    for model in model_objects {
        let mut texture_packed = dmw3_pack::Packed::from(
            model.packed.files[model.header.texture_offset as usize].clone(),
        );

        let texture_raw = match rlen_decode(&texture_packed.files[0][..]) {
            Ok(file) => file,
            Err(_) => texture_packed.files[0].clone(),
        };

        let mut texture_tim = Tim::from(texture_raw);

        let mut hue_shift = 0.0;
        for _ in 0..preset.shuffles {
            hue_shift = (rng.next_u32() % 360) as f64;
        }

        for i in 0..64 {
            for j in 0..last_n_rows {
                let l0 = (i + (256 - last_n_rows * row_skip + j * row_skip) * 64) * 2;

                let color = &texture_tim.image.bytes[l0..l0 + 2];

                let raw = u16::from_le_bytes([color[0], color[1]]);

                let r = raw & 0x1f;
                let g = (raw >> 5) & 0x1f;
                let b = (raw >> 10) & 0x1f;
                let stp = raw >> 15;

                if r == 0 && g == 0 && b == 0 && stp == 0 {
                    continue;
                }

                let r_norm = ((r * 255) / 0x1f) as u8;
                let g_norm = ((g * 255) / 0x1f) as u8;
                let b_norm = ((b * 255) / 0x1f) as u8;

                let r = r_norm as f64 / 255.0;
                let g = g_norm as f64 / 255.0;
                let b = b_norm as f64 / 255.0;

                let max = r.max(g).max(b);
                let min = r.min(g).min(b);
                let delta = max - min;

                let h = if delta == 0.0 {
                    0.0
                } else if max == r {
                    60.0 * (((g - b) / delta) % 6.0)
                } else if max == g {
                    60.0 * (((b - r) / delta) + 2.0)
                } else {
                    60.0 * (((r - g) / delta) + 4.0)
                };

                let h = if h < 0.0 { h + 360.0 } else { h };

                let s = if max == 0.0 { 0.0 } else { delta / max };

                let v = max;

                let new_hue = (h + hue_shift).rem_euclid(360.0);

                let c = v * s;
                let x = c * (1.0 - ((new_hue / 60.0) % 2.0 - 1.0).abs());
                let m = v - c;

                let (r, g, b) = match new_hue {
                    0.0..=60.0 => (c, x, 0.0),
                    60.0..=120.0 => (x, c, 0.0),
                    120.0..=180.0 => (0.0, c, x),
                    180.0..=240.0 => (0.0, x, c),
                    240.0..=300.0 => (x, 0.0, c),
                    300.0..=360.0 => (c, 0.0, x),
                    _ => (0.0, 0.0, 0.0), // Fallback case (shouldn't happen with valid input)
                };

                let mut r = ((((r + m) * 255.0).round() as u16) * 0x1f) / 255;
                let mut g = ((((g + m) * 255.0).round() as u16) * 0x1f) / 255;
                let mut b = ((((b + m) * 255.0).round() as u16) * 0x1f) / 255;

                if r == g && g == b && b == 0 {
                    r += 1;
                    g += 1;
                    b += 1;
                }

                let new_c: u16 = (b << 10) | (g << 5) | r | stp << 15;

                let new_c_bytes = new_c.to_le_bytes();

                texture_tim.image.bytes[l0..l0 + 2].copy_from_slice(&new_c_bytes);
            }
        }

        let new_tim: Vec<u8> = texture_tim.into();

        let recoded = rlen_encode(&new_tim);

        texture_packed.files[0] = recoded;

        model.packed.files[model.header.texture_offset as usize] = texture_packed.into();
    }

    Ok(())
}

pub fn patch(
    preset: &Randomizer,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    if preset.models.hue_enabled {
        hue(preset, &mut objects.model_objects, rng, 4, 8)?;
    }

    if preset.models.stage_hue_enabled {
        hue(preset, &mut objects.stage_model_objects, rng, 8, 1)?;
    }

    Ok(())
}
