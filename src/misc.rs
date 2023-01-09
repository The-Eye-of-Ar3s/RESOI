use std::io::Read;


pub fn get_byte_from_input() -> u8 { // Get a single byte from stdin
    match std::io::stdin().bytes().next().and_then(|result| result.ok()) {
        None => return 0,
        Some(v) => {
            if v == 4 {
                return 0;
            } else {
                return v;
            };
        }
    }
}