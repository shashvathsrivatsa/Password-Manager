use typenum::U16;
use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

pub fn encrypt(password: &str) -> Vec<u8> {
    let key = GenericArray::from([0_u8; 16]);
    let cipher = Aes128::new(&key);

    let password_bytes = password.as_bytes();
    let mut encrypted_password: Vec<u8> = Vec::new();

    for chunk in password_bytes.chunks(16) {

        let mut padded_chunk = [0_u8; 16];
        padded_chunk[..chunk.len()].copy_from_slice(chunk);

        let mut block = GenericArray::<u8, U16>::default();
        block.copy_from_slice(&padded_chunk);

        cipher.encrypt_block(&mut block);
        encrypted_password.extend_from_slice(&block);
    }

    encrypted_password
}

pub fn decrypt(byte_sequence: Vec<u8>) -> String {
    let key = GenericArray::from([0_u8; 16]);
    let cipher = Aes128::new(&key);

    let mut decrypted_password: Vec<u8> = Vec::new();

    for chunk in byte_sequence.chunks(16) {

        let mut block = GenericArray::<u8, U16>::default();
        block.copy_from_slice(&chunk);

        cipher.decrypt_block(&mut block);
        decrypted_password.extend_from_slice(&block);
    }

    String::from_utf8_lossy(&decrypted_password).to_string()
}


