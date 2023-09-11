use crate::rand::Objects;

pub fn scaling(objects: &mut Objects) {
    for scaling in &mut objects.scaling.modified {
        for affinity in &mut scaling.stat_offsets {
            *affinity -= 1;
        }

        for affinity in &mut scaling.res_offsets {
            *affinity -= 1;
        }
    }
}
