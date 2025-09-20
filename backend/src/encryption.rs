use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::RngCore;
use std::fs::{File, self};
use std::io::{Read, Write};
use generic_array::GenericArray;

// Define the AES-256-CBC mode with PKCS7 padding
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

/// Encrypts files in the specified directory using AES-256-CBC encryption.
///
/// # Arguments
///
/// * `key` - A slice of bytes representing the encryption key.
/// * `directory` - A string slice representing the path to the directory containing files to encrypt.
pub fn encrypt_files(key: &[u8], directory: &str) {
    let mut rng = rand::thread_rng();

    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            // Generate a new random IV for each file
            let mut iv = GenericArray::<u8, <Aes256Cbc as BlockMode<Aes256, Pkcs7>>::IvSize>::default();
            rng.fill_bytes(&mut iv);

            // Create a new cipher instance per file
            let cipher = Aes256Cbc::new_from_slices(key, &iv).unwrap();

            // Read file content
            let mut file = File::open(&path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();

            // Encrypt the file content
            let mut ciphertext = cipher.encrypt_vec(&buffer);

            // Prepend the IV to the ciphertext so it can be used for decryption later
            let mut output = iv.to_vec();
            output.append(&mut ciphertext);

            // Write the encrypted data (IV + ciphertext) back to the file
            let mut encrypted_file = File::create(&path).unwrap();
            encrypted_file.write_all(&output).unwrap();
        }
    }
}

