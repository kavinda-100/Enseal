use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use anyhow::{Result, anyhow};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use rand::{RngCore, rngs::OsRng};
use zeroize::Zeroize;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

/// Encrypts data using AES-256-GCM with a key derived via Argon2id.
pub fn encrypt(data: &[u8], mut password: String) -> Result<Vec<u8>> {
    // 1. Generate a random Salt
    let mut salt_bytes = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt_bytes);
    let salt = SaltString::encode_b64(&salt_bytes).map_err(|_| anyhow!("Failed to encode salt"))?;

    // 2. Derive Key using Argon2id
    let mut derived_key = [0u8; 32];
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("KDF error: {}", e))?;

    if let Some(output) = hash.hash {
        let hash_bytes = output.as_bytes();
        derived_key.copy_from_slice(&hash_bytes[..32.min(hash_bytes.len())]);
    } else {
        return Err(anyhow!("Failed to get hash"));
    }

    // 3. Setup AES-GCM
    let cipher =
        Aes256Gcm::new_from_slice(&derived_key).map_err(|_| anyhow!("Invalid key length"))?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 4. Encrypt
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|_| anyhow!("Encryption failed"))?;

    // 5. Cleanup password/key from memory
    password.zeroize();
    derived_key.zeroize();

    // 6. Pack result: [Salt (16)] + [Nonce (12)] + [Ciphertext]
    let mut result = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&salt_bytes);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypts data using the derived key.
pub fn decrypt(data: &[u8], mut password: String) -> Result<Vec<u8>> {
    if data.len() < SALT_LEN + NONCE_LEN {
        return Err(anyhow!("Encrypted file is too short or corrupted"));
    }

    // 1. Unpack Salt and Nonce
    let salt_bytes = &data[..SALT_LEN];
    let nonce_bytes = &data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    let salt = SaltString::encode_b64(salt_bytes).map_err(|_| anyhow!("Failed to decode salt"))?;

    // 2. Re-derive Key
    let mut derived_key = [0u8; 32];
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| anyhow!("Invalid password or corrupted salt"))?;

    if let Some(output) = hash.hash {
        let hash_bytes = output.as_bytes();
        derived_key.copy_from_slice(&hash_bytes[..32.min(hash_bytes.len())]);
    } else {
        return Err(anyhow!("Failed to get hash"));
    }

    // 3. Decrypt
    let cipher =
        Aes256Gcm::new_from_slice(&derived_key).map_err(|_| anyhow!("Invalid key length"))?;
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow!("Decryption failed! Wrong password or tampered file."))?;

    // 4. Cleanup
    password.zeroize();
    derived_key.zeroize();

    Ok(plaintext)
}
