pub fn caesar(cipher: &str, shift: u8) -> String {
    cipher
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (first + (((c as u8) + shift - first) % 26)) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn print_caesar() {
    let returned_data = caesar("Ceasar", 13);
    println!("{:?}", returned_data);
}

