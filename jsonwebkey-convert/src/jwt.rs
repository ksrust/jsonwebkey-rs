use super::*;
use jsonwebtoken::DecodingKey;

pub trait ToDecodingKey {
    fn to_decoding_key(&'_ self) -> DecodingKey;
}

impl ToDecodingKey for RSAPublicKey {
    fn to_decoding_key(&'_ self) -> DecodingKey {
        DecodingKey::from_rsa_components(&self.n.base64, &self.e.base64).unwrap()
    }
}
