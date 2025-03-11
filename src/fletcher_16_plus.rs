use super::Hasher;

pub struct Fletcher16;

impl Hasher for Fletcher16 {
    fn hash_name(&self) -> &'static str {
        "Fletcher-16"
    }
    fn active_bits(&self) -> &'static u32 {
        &16
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        fletcher::calc_fletcher16(data).to_be_bytes().to_vec()
    }
}

pub struct Fletcher32;

impl Hasher for Fletcher32 {
    fn hash_name(&self) -> &'static str {
        "Fletcher-32"
    }
    fn active_bits(&self) -> &'static u32 {
        &32
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let u16_data: Vec<u16> = data.chunks(2).map(|chunk| {
            let mut bytes = [0u8; 2];
            bytes[0] = chunk[0];
            if chunk.len() == 2 {
                bytes[1] = chunk[1];
            }
            u16::from_be_bytes(bytes)
        }).collect();
        fletcher::calc_fletcher32(&*u16_data).to_be_bytes().to_vec()
    }
}

pub struct Fletcher64;

impl Hasher for Fletcher64 {
    fn hash_name(&self) -> &'static str {
        "Fletcher-64"
    }
    fn active_bits(&self) -> &'static u32 {
        &64
    }
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let u32_data: Vec<u32> = data.chunks(4).map(|chunk| {
            let mut bytes = [0u8; 4];
            for (i, byte) in chunk.iter().enumerate() {
                bytes[i] = *byte;
            }
            u32::from_be_bytes(bytes)
        }).collect();
        fletcher::calc_fletcher64(&*u32_data).to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fletcher_16_hash_validate() {
        use hex::decode;
        let hasher = Fletcher16;

        assert_eq!(hasher.hash(b"abc"), decode("4c27").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("e178").unwrap());
    }

    #[test]
    fn fletcher_32_hash_validate() {
        use hex::decode;
        let hasher = Fletcher32;

        assert_eq!(hasher.hash(b"abc"), decode("024a0126").unwrap());
        assert_eq!(hasher.hash(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"), decode("974a1068").unwrap());
    }
}
