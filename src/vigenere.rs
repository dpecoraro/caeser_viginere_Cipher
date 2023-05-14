
pub fn vigenere_decrypt(text: &str, key: &str) -> Result<String, &'static str> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("Key must be non-empty and only contain alphabetic characters");
    }

    let key = key.chars().collect::<Vec<char>>();
    let mut key_index = 0;

    Ok(text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = (key[key_index % key.len()].to_ascii_lowercase() as u8 - b'a') % 26;
                key_index += 1;
                let mut shifted = c as u8 - shift;
                if shifted < first {
                    shifted += 26;
                }
                shifted as char
            } else {
                c
            }
        })
        .collect())
}

pub fn vigenere_encrypt(text: &str, key: &str) -> Result<String, &'static str> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("Key must be non-empty and only contain alphabetic characters");
    }

    let key = key.chars().collect::<Vec<char>>();
    let mut key_index = 0;

    Ok(text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = (key[key_index % key.len()].to_ascii_lowercase() as u8 - b'a') % 26;
                key_index += 1;
                (first + (c as u8 - first + shift) % 26) as char
            } else {
                c
            }
        })
        .collect()
    )
}