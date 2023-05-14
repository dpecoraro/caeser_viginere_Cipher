use ceaser_cipher::vigenere::{vigenere_encrypt, vigenere_decrypt};
use ceaser_cipher::caesar::*;

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