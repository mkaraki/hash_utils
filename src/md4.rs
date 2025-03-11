use super::Hasher;

pub struct Md4;

impl Hasher for Md4 {
    fn hash_name(&self) -> &'static str {
        "md4"
    }
    fn active_bits(&self) -> &'static u32 {
        &128
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use md4::{Md4, Digest};

        let mut hasher = Md4::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_properties() {
        let md4 = Md4;

        assert_eq!(md4.hash_name(), "md4");
        assert_eq!(md4.active_bits(), &128);
    }

    #[test]
    fn it_works() {
        use hex::decode;
        let md4 = Md4;

        // Defined in RFC 1320, page 19: A.5 Test suite
        assert_eq!(md4.hash(b""), decode("31d6cfe0d16ae931b73c59d7e0c089c0").unwrap());
        assert_eq!(md4.hash(b"a"), decode("bde52cb31de33e46245e05fbdbd6fb24").unwrap());
        assert_eq!(md4.hash(b"abc"), decode("a448017aaf21d8525fc10ae87aa6729d").unwrap());
        assert_eq!(md4.hash(b"message digest"), decode("d9130a8164549fe818874806e1c7014b").unwrap());
        assert_eq!(md4.hash(b"abcdefghijklmnopqrstuvwxyz"), decode("d79e1c308aa5bbcdeea8ed63df412da9").unwrap());
        assert_eq!(md4.hash(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"), decode("043f8582f241db351ce627e153e7f0e4").unwrap());
        assert_eq!(md4.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("e33b4ddc9c38f2199c3e7b164fcc0536").unwrap());
    }
}