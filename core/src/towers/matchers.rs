// @todo: replace with compile-time hashmap?

pub fn match_size(id: u16) -> u8 {
    match id {
        0 => 2,
        _ => panic!("Invalid tower id {}", id),
    }
}
