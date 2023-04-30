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
    let mut key = [139, 126, 86, 117, 220, 68, 15, 204, 19, 236, 104, 88, 159, 36, 208, 96, 147, 42, 151, 142, 119, 190, 46, 62, 5, 96, 179, 175, 124, 238, 139, 184];
    let mut nonce = [217, 190, 79, 64, 1, 171, 39, 85, 8, 221, 229, 225, 84, 144, 247, 133, 247, 188, 255, 36, 174, 157, 189, 203];
    let mut all_files: Vec<DirEntry> = Vec::new();
    for entry in WalkDir::new("C:\\Users\\logan\\Documents\\testDarkRust\\Test3").into_iter().filter_map(|e| e.ok()) {
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