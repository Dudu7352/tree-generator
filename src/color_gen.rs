use rand::{rngs::ThreadRng, Rng};

pub fn randomised_color(rng: &mut ThreadRng, base: (u8, u8, u8), dev: u8) -> String {
    format!(
        "#{:0>2x}{:0>2x}{:0>2x}",
        base.0 + rng.gen_range(0..dev),
        base.1 + rng.gen_range(0..dev),
        base.2 + rng.gen_range(0..dev)
    )
}

pub fn flower_hex(rng: &mut ThreadRng) -> String {
    randomised_color(rng, (0x00, 0x99, 0x00), 0x44)
}

pub fn branch_hex(rng: &mut ThreadRng) -> String {
    randomised_color(rng, (0x69, 0x19, 0x19), 30)
}
