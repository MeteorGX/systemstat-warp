mod cipher_256;
mod cipher_512;

pub use cipher_256::*;
pub use cipher_512::*;

pub trait CipherParser {
    fn cipher(&self) -> Option<String>;
}