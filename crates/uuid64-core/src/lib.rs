use std::fmt;
use uuid::Uuid;

const BASE64_CHARS: &[u8; 64] =
    b"-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz";

// Reverse lookup: ASCII value -> index in BASE64_CHARS, or 0xFF if invalid.
const DECODE_TABLE: [u8; 128] = {
    let mut table = [0xFFu8; 128];
    let mut i = 0usize;
    while i < 64 {
        table[BASE64_CHARS[i] as usize] = i as u8;
        i += 1;
    }
    table
};

#[derive(Debug)]
pub enum UuidError {
    InvalidBase64Length(usize),
    InvalidBase64Character(char),
    InvalidUuid(uuid::Error),
}

impl fmt::Display for UuidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBase64Length(len) => {
                write!(f, "expected base64 length 22, got {len}")
            }
            Self::InvalidBase64Character(ch) => {
                write!(f, "invalid base64 character: {ch}")
            }
            Self::InvalidUuid(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for UuidError {}

pub struct Uuid64;

impl Uuid64 {
    #[must_use]
    pub fn new_v4() -> String {
        Self::encode(Uuid::new_v4())
    }

    #[must_use]
    pub fn new_v7() -> String {
        Self::encode(Uuid::now_v7())
    }

    pub fn decode(encoded: &str) -> Result<Uuid, UuidError> {
        let bytes = Self::base64_to_bytes(encoded)?;
        // from_slice only fails if the slice isn't 16 bytes, which we guarantee.
        Ok(Uuid::from_bytes(bytes))
    }

    pub fn decode_uuid(encoded: &str) -> Result<String, UuidError> {
        Ok(Self::decode(encoded)?.to_string())
    }

    #[must_use]
    pub fn encode(uuid: Uuid) -> String {
        Self::bytes_to_base64(uuid.as_bytes())
    }

    pub fn encode_uuid(uuid: &str) -> Result<String, UuidError> {
        let uuid = Uuid::parse_str(uuid).map_err(UuidError::InvalidUuid)?;
        Ok(Self::encode(uuid))
    }

    fn bytes_to_base64(bytes: &[u8; 16]) -> String {
        // Pack 16 bytes (128 bits) into 22 base64 chars (132 bits) with 4 zero pad bits at the end.
        // Process in 3-byte (24-bit) groups → 4 base64 chars each, same as standard base64.
        // 16 bytes = 5 groups of 3 (15 bytes → 20 chars) + 1 remaining byte → 2 chars.
        let mut result = String::with_capacity(22);
        let mut i = 0;
        while i + 3 <= 16 {
            let b0 = bytes[i] as u32;
            let b1 = bytes[i + 1] as u32;
            let b2 = bytes[i + 2] as u32;
            let chunk = (b0 << 16) | (b1 << 8) | b2;
            result.push(BASE64_CHARS[((chunk >> 18) & 0x3f) as usize] as char);
            result.push(BASE64_CHARS[((chunk >> 12) & 0x3f) as usize] as char);
            result.push(BASE64_CHARS[((chunk >> 6) & 0x3f) as usize] as char);
            result.push(BASE64_CHARS[(chunk & 0x3f) as usize] as char);
            i += 3;
        }
        // Remaining 1 byte → 2 chars (top 6 bits, then bottom 2 bits padded to 6)
        let b0 = bytes[15] as u32;
        result.push(BASE64_CHARS[((b0 >> 2) & 0x3f) as usize] as char);
        result.push(BASE64_CHARS[((b0 << 4) & 0x3f) as usize] as char);
        result
    }

    fn base64_to_bytes(data: &str) -> Result<[u8; 16], UuidError> {
        if data.len() != 22 {
            return Err(UuidError::InvalidBase64Length(data.len()));
        }
        let mut out = [0u8; 16];
        let chars: Vec<u8> = data
            .bytes()
            .map(|b| {
                DECODE_TABLE
                    .get(b as usize)
                    .copied()
                    .filter(|&v| v != 0xFF)
                    .ok_or(UuidError::InvalidBase64Character(b as char))
            })
            .collect::<Result<_, _>>()?;

        // 20 chars → 5 groups of 4 chars → 5 groups of 3 bytes (bytes 0..14)
        for i in 0..5 {
            let c = i * 4;
            let chunk = ((chars[c] as u32) << 18)
                | ((chars[c + 1] as u32) << 12)
                | ((chars[c + 2] as u32) << 6)
                | (chars[c + 3] as u32);
            out[i * 3] = (chunk >> 16) as u8;
            out[i * 3 + 1] = (chunk >> 8) as u8;
            out[i * 3 + 2] = chunk as u8;
        }
        // Last 2 chars → 1 byte (byte 15)
        out[15] = ((chars[20] as u8) << 2) | ((chars[21] as u8) >> 4);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_v4() {
        let encoded = Uuid64::new_v4();

        let uuid = Uuid64::decode(&encoded).unwrap();

        assert_eq!(
            encoded,
            Uuid64::encode(uuid),
        );
    }

    #[test]
    fn round_trip_v7() {
        let encoded = Uuid64::new_v7();

        let uuid = Uuid64::decode(&encoded).unwrap();

        assert_eq!(
            encoded,
            Uuid64::encode(uuid),
        );
    }

    #[test]
    fn encode_uuid_roundtrip() {
        let original = Uuid::new_v4().to_string();
        let encoded = Uuid64::encode_uuid(&original).unwrap();
        let decoded = Uuid64::decode_uuid(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn encode_uuid_invalid() {
        let err = Uuid64::encode_uuid("not-a-uuid").unwrap_err();
        // Exercises Display for UuidError::InvalidUuid
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn decode_invalid_length() {
        let err = Uuid64::decode("tooshort").unwrap_err();
        assert_eq!(err.to_string(), "expected base64 length 22, got 8");
    }

    #[test]
    fn decode_invalid_character() {
        let err = Uuid64::decode("!!!!!!!!!!!!!!!!!!!!!!").unwrap_err();
        assert!(matches!(err, UuidError::InvalidBase64Character('!')));
        assert_eq!(err.to_string(), "invalid base64 character: !");
    }
}
