use super::Hasher;

pub struct LongitudinalRedundancyCheck;

impl Hasher for LongitudinalRedundancyCheck {
    fn hash_name(&self) -> &'static str {
        "LRC"
    }

    fn active_bits(&self) -> &'static u32 {
        &8
    }

    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut lrc = 0u8;
        for byte in input {
            lrc = lrc.wrapping_add(*byte);
        }
        lrc = (lrc ^ 0xFF).wrapping_add(1);
        vec![lrc]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::decode;

    #[test]
    fn hash_validate() {
        let hasher = LongitudinalRedundancyCheck;

        assert_eq!(hasher.hash(b"abc"), decode("da").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("98").unwrap());
    }
}