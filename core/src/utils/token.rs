use aes_gcm::{
  aead::{generic_array::GenericArray, AeadCore, OsRng},
  AeadInPlace, Aes256Gcm, KeyInit,
};

use crate::error::PolestarResult;

use super::encrypt::{read_nonce, read_token, write_nonce, write_token};

pub fn encrypt_token(original_token: &[u8]) -> PolestarResult<()> {
  let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
  let _ = write_nonce(&nonce);

  let nonce = GenericArray::from_slice(&nonce);
  let cipher = Aes256Gcm::new_from_slice(crate::KEY).expect("Invalid key length");
  let mut buffer: Vec<u8> = Vec::new();
  buffer.extend_from_slice(original_token);
  let _ = cipher.encrypt_in_place(nonce, b"", &mut buffer);
  let _ = write_token(&buffer);
  Ok(())
}

pub fn decrypt_token(key: &[u8]) -> PolestarResult<String> {
  let encrypt_token = read_token()?;
  let b_nonce = read_nonce()?;
  let mut buffer: Vec<u8> = Vec::new();
  buffer.extend_from_slice(&encrypt_token);
  let nonce = GenericArray::from_slice(&b_nonce);
  let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key length");
  let _ = cipher.decrypt_in_place(nonce, b"", &mut buffer);
  let token = String::from_utf8(buffer.to_vec())?;
  Ok(token)
}

pub fn del_token() -> PolestarResult<()> {
  let _ = super::encrypt::del_token();
  let _ = super::encrypt::del_nonce();
  Ok(())
}
