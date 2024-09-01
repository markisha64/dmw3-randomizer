use rand_xoshiro::{rand_core::RngCore, Xoshiro256StarStar};
use rlen::{rlen_decode, rlen_encode};
use tim::Tim;

use crate::{json::Randomizer, rand::Objects};

pub fn patch(preset: &Randomizer, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    dbg!(objects.model_objects.len());
    for model in &mut objects.model_objects {
        dbg!(&model.file_name);
        let texture_packed = dmw3_pack::Packed::try_from(
            model
                .packed
                .get_file(model.header.texture_offset as usize)
                .unwrap(),
        )
        .unwrap();

        let texture_raw = match rlen_decode(&texture_packed.get_file(0).unwrap()[..]) {
            Ok(file) => file,
            Err(_) => texture_packed.get_file(0).unwrap().into(),
        };

        let mut texture_tim = Tim::from(texture_raw);

        let hue_shift = (rng.next_u32() as f64) / (rng.next_u32() as f64);

        for i in 0..64 {
            for j in 0..4 {
                let l = (i + (224 + j * 8) * 64) * 2;

                let color = &texture_tim.image.bytes[l..l + 2];

                let raw = u16::from_le_bytes([color[0], color[1]]);

                let _stp = (raw >> 15) > 0;

                let r = raw & 0x1f;
                let g = (raw >> 5) & 0x1f;
                let b = (raw >> 10) & 0x1f;

                if r == 0 && g == 0 && b == 0 {
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

                let new_h = (h + hue_shift).rem_euclid(360.0);

                let c = v * s;
                let x = c * (1.0 - ((new_h / 60.0) % 2.0 - 1.0).abs());
                let m = v - c;

                let (r, g, b) = match new_h {
                    0.0..=60.0 => (c, x, 0.0),
                    60.0..=120.0 => (x, c, 0.0),
                    120.0..=180.0 => (0.0, c, x),
                    180.0..=240.0 => (0.0, x, c),
                    240.0..=300.0 => (x, 0.0, c),
                    300.0..=360.0 => (c, 0.0, x),
                    _ => (0.0, 0.0, 0.0), // Fallback case (shouldn't happen with valid input)
                };

                let r = ((r + m) * 255.0).round() as u8;
                let g = ((g + m) * 255.0).round() as u8;
                let b = ((b + m) * 255.0).round() as u8;

                let new_c: u16 = (((b as u16 * 0x1f) / 255) << 10)
                    | (((g as u16 * 0x1f) / 255) << 5)
                    | ((r as u16 * 0x1f) / 255);

                let new_c_bytes = new_c.to_le_bytes();

                texture_tim.image.bytes[l..l + 2]
                    .copy_from_slice(&[new_c_bytes[0], new_c_bytes[1]]);
            }
        }

        let new_tim: Vec<u8> = texture_tim.into();

        let recoded = rlen_encode(&new_tim);

        let offset = model
            .packed
            .get_offset(model.header.texture_offset as usize)
            .unwrap() as usize;

        let assumed_length = model.packed.assumed_length[model.header.texture_offset as usize];

        let recoded_length = recoded.len() + 8;

        let ending = Vec::from(&model.packed.buffer[offset + assumed_length..]);

        let new_size = model.packed.buffer.len() + recoded_length - assumed_length;

        model.packed.buffer.resize(new_size, 0);

        model.packed.buffer[(offset + 8)..(offset + recoded.len() + 8)]
            .copy_from_slice(&recoded[..]);

        model.packed.buffer[(offset + recoded.len() + 8)..].copy_from_slice(&ending[..]);

        for idx in model.packed.iter() {
            let mut offset = model.packed.get_offset(idx).unwrap() as usize;

            if offset >= offset + 8 + assumed_length {
                offset += recoded_length - assumed_length;

                model.packed.buffer[idx * 4..(idx + 1) * 4]
                    .copy_from_slice(&(offset as u32).to_le_bytes());
            }
        }
    }
}
