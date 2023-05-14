use crate::vigenere_encrypt;
use crate::vigenere_decrypt;

#[test]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere_encrypt() {
        let result = vigenere_encrypt("Hello, World!", "KEY").unwrap();
        assert_eq!(result, "Rijvs, Ftqfd!");
    }

    #[test]
    fn test_vigenere_decrypt() {
        let result = vigenere_decrypt("Rijvs, Ftqfd!", "KEY").unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_vigenere_encrypt_empty_key() {
        let result = vigenere_encrypt("Hello, World!", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_vigenere_decrypt_empty_key() {
        let result = vigenere_decrypt("Rijvs, Ftqfd!", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_vigenere_encrypt_non_alphabetic_key() {
        let result = vigenere_encrypt("Hello, World!", "KEY1");
        assert!(result.is_err());
    }

    #[test]
    fn test_vigenere_decrypt_non_alphabetic_key() {
        let result = vigenere_decrypt("Rijvs, Ftqfd!", "KEY1");
        assert!(result.is_err());
    }
}