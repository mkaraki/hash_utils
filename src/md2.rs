use super::Hasher;

pub struct Md2;

impl Hasher for Md2 {
    fn hash_name(&self) -> &'static str {
        "md2"
    }
    fn active_bits(&self) -> &'static u32 {
        &128
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use md2::{Md2, Digest};

        let mut hasher = Md2::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_properties() {
        let md2 = Md2;

        assert_eq!(md2.hash_name(), "md2");
        assert_eq!(md2.active_bits(), &128);
    }

    #[test]
    fn hash_validate() {
        use hex::decode;
        let md2 = Md2;

        // Defined in RFC 1319, page 16: A.5 Test suite
        assert_eq!(md2.hash(b""), decode("8350e5a3e24c153df2275c9f80692773").unwrap());
        assert_eq!(md2.hash(b"a"), decode("32ec01ec4a6dac72c0ab96fb34c0b5d1").unwrap());
        assert_eq!(md2.hash(b"abc"), decode("da853b0d3f88d99b30283a69e6ded6bb").unwrap());
        assert_eq!(md2.hash(b"message digest"), decode("ab4f496bfb2a530b219ff33031fe06b0").unwrap());
        assert_eq!(md2.hash(b"abcdefghijklmnopqrstuvwxyz"), decode("4e8ddff3650292ab5a4108c3aa47940b").unwrap());
        assert_eq!(md2.hash(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"), decode("da33def2a42df13975352846c30338cd").unwrap());
        assert_eq!(md2.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("d5976f79d83d3a0dc9806c3c66f3efd8").unwrap());
    }
}
