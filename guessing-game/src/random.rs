use rand::Rng;

pub fn generate_target(start: u32, end: u32) -> u32 {
    return rand::thread_rng().gen_range(start..end + 1);
}
