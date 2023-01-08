use std::io::Read;


pub fn get_byte_from_input() -> u8 {
    match std::io::stdin().bytes() .next().and_then(|result| result.ok()) {
        None => return 0,
        Some(v) => return v
    }
}