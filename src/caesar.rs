pub fn caesar_cipher(text: &str, shift: u8) -> String {
    let mut ciphered_text = String::new();

    for c in text.chars() {
        if c.is_alphabetic() {
            let ascii_offset = if c.is_lowercase() { b'a' } else { b'A' };
            let shifted_char = ((((c as u8) - ascii_offset + shift) % 26) + ascii_offset) as char;
            ciphered_text.push(shifted_char);
        } else {
            ciphered_text.push(c);
        }
    }

    ciphered_text
}

pub fn caesar_decipher(text: &str, shift: u8) -> String {
    let deciphered_text: String = text
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() { b'a' } else { b'A' };
                (((c as i8 - base as i8 - shift as i8 + 26) % 26) + base as i8) as u8 as char
            } else {
                c
            }
        })
        .collect();

    deciphered_text
}
