use crate::CipherParser;
use sha2::Digest;

pub struct Cipher256<'a>(&'a [u8]);

impl<'a> Cipher256<'a> {
    pub fn create(bytes: &'a [u8]) -> Self {
        Self {
            0: bytes
        }
    }
}


impl<'a> CipherParser for Cipher256<'a> {
    fn cipher(&self) -> Option<String> {
        let mut cipher = sha2::Sha256::new();
        cipher.update(self.0);
        Some(format!("{:x}", cipher.finalize()))
    }
}