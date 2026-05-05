use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, AeadCore},
    Aes256Gcm, Nonce, Key
};
use anyhow::{anyhow, Result};

pub fn encrypt(data: &str, master_key:&[u8]) -> Result<Vec<u8>>{

    let key = Key::<Aes256Gcm>::from_slice(master_key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); 
    let mut ciphertext = cipher.encrypt(&nonce, data.as_bytes()).map_err(|e| anyhow!("Erreur de chiffrement : {}", e))?;

    // on  crée un vecteur de 12 octets 
    let mut result: Vec<u8> = nonce.to_vec();
    // on fucionne nonce et message secret 
    result.append(&mut ciphertext);

    Ok(result)
}

pub fn decrypt(encrypt_data: &[u8], master_key:&[u8]) -> Result<String>{
    let key = Key::<Aes256Gcm>::from_slice(master_key);
    let cipher = Aes256Gcm::new(&key);

    let (nonce, encrypt_part) = encrypt_data.split_at(12);
    let nonce = Nonce::from_slice(nonce);

    // Déchiffrement
    let plaintext_bytes = cipher.decrypt(nonce, encrypt_part)
        .map_err(|e| anyhow!("Erreur de déchiffrement : {}", e))?;

    // Conversion des octets en texte String
    let plaintext = String::from_utf8(plaintext_bytes)
        .map_err(|_e| anyhow!("Format de texte invalide (UTF-8)"))?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_success() {
        // La clé doit faire exactement 32 octets pour AES-256
        let master_key = b"a_very_secure_master_key_32_bits"; 
        let secret_data = "MaCleApiCryptoCom123";

        // Test du chiffrement
        let encrypted = encrypt(secret_data, master_key)
            .expect("Le chiffrement ne devrait pas échouer");
        
        // On vérifie que le résultat n'est pas le texte en clair
        assert_ne!(encrypted, secret_data.as_bytes());
        // On vérifie qu'on a bien [Nonce (12)] + [Ciphertext + Tag]
        assert!(encrypted.len() > 12);

        // test du déchiffrement
        let decrypted = decrypt(&encrypted, master_key)
            .expect("Le déchiffrement ne devrait pas échouer");
        
        // On vérifie qu'on retrouve exactement l'original
        assert_eq!(decrypted, secret_data);
    }

    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let master_key = b"a_very_secure_master_key_32_bits";
        let wrong_key = b"wrong_key_but_still_32_bits_size";
        let secret_data = "donnees_confidentielles";

        let encrypted = encrypt(secret_data, master_key).unwrap();
        
        let result = decrypt(&encrypted, wrong_key);
        assert!(result.is_err(), "Le déchiffrement aurait dû échouer avec une mauvaise clé");
    }

    #[test]
    fn test_corrupted_data_fails() {
        let master_key = b"a_very_secure_master_key_32_bits";
        let secret_data = "message";

        let mut encrypted = encrypt(secret_data, master_key).unwrap();
        
        if let Some(byte) = encrypted.last_mut() {
            *byte = *byte ^ 0xFF; 
        }

        let result = decrypt(&encrypted, master_key);
        assert!(result.is_err(), "Les données corrompues ne doivent pas être déchiffrées");
    }

    #[test]
    fn test_data_too_short_fails() {
        let master_key = b"a_very_secure_master_key_32_bits";
        let too_short_data = vec![0u8; 10]; 

        let result = decrypt(&too_short_data, master_key);
        assert!(result.is_err());
    }
}