
fn caesar_cipher(text: &str, shift: u8) -> String {
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

fn vigenere_encrypt(text: &str, key: &str) -> Result<String, &'static str> {
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

fn caesar_decipher(text: &str, shift: u8) -> String {
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

fn vigenere_decrypt(text: &str, key: &str) -> Result<String, &'static str> {
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
fn main() {
    let plaintext = "Hello, World!";
    let shift = 3;
    let key = "key";
    
    let ciphertext = caesar_cipher(plaintext, shift);
    let deciphertext = caesar_decipher(&ciphertext, shift);

    match vigenere_encrypt(plaintext, key) {
        Ok(encrypted) => match vigenere_decrypt(&encrypted, key) {
            Ok(decrypted) => {
                println!("Original: {}", plaintext);
                println!("Encrypted: {}", encrypted);
                println!("Decrypted: {}", decrypted);
            },
            Err(e) => println!("Error in decryption: {}", e),
        },
        Err(e) => println!("Error in encryption: {}", e),
    }

    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {}", ciphertext);
    println!("DecipherText: {}", deciphertext);
}