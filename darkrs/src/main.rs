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
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);
    let mut all_files: Vec<DirEntry> = Vec::new();
    for entry in WalkDir::new("C:\\Users\\logan\\Documents\\testDarkRust\\testWord").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            all_files.push(entry)
        }
        else {
            println!("Full Path: {}", entry.path().display());
            println!("This is a directory");
        }
    }
    
    println!("\nHere are all the files: ");
    for entry in all_files{
        println!("{}", entry.path().display());
        if let Some(filepath) = entry.path().as_os_str().to_str() {
            encrypt_sm_file(filepath, &key, &nonce);
            decrypt_sm_file((filepath.to_owned() + ".encrypted").as_str(), &key, &nonce);
        }
        
    }
}

fn encrypt_sm_file(filepath: &str, key: &[u8; 32], nonce: &[u8; 24]) -> Result<(), anyhow::Error> {
    let contents = fs::read(filepath)?;
    let cipher = XChaCha20Poly1305::new(key.into());
    let encrypted_file = cipher.encrypt(nonce.into(), contents.as_ref()).map_err(|err| anyhow!("Encrypting small file: {}", err))?;
    fs::write(filepath.to_owned() + ".encrypted", encrypted_file)?;
    Ok(())
}

fn decrypt_sm_file(filepath: &str, key: &[u8; 32], nonce: &[u8; 24]) -> Result<(), anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let contents = fs::read(filepath)?;
    let decrypted_file = cipher
        .decrypt(nonce.into(), contents.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;
    fs::write(filepath.to_owned() + ".decrypted", decrypted_file);
    Ok(())
}