use std::{
    convert::TryFrom,
    fmt::{self, write, Display},
    str::FromStr,
};

/*
 * From the PNG Spec:
 *
 * A 4-byte chunk type code. For convenience in description and in examining
 * PNG files, type codes are restricted to consist of uppercase and lowercase
 * ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal). However, encoders
 * and decoders must treat the codes as fixed binary values, not character
 * strings. For example, it would not be correct to represent the type code
 * IDAT by the EBCDIC equivalents of those letters. Additional naming
 * conventions for chunk types are discussed in the next section.
 *
 * Chunk Naming Conventions:
 * Ancillary bit: bit 5 of first byte
 *      0 (uppercase) = critical, 1 (lowercase) = ancillary.
 * Private bit: bit 5 of second byte
 *      0 (uppercase) = public, 1 (lowercase) = private.
 * Reserved bit: bit 5 of third byte
 *      Must be 0 (uppercase) in files conforming to this version of PNG.
 * Safe-to-copy bit: bit 5 of fourth byte
 *      0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
 *
 */

#[derive(Eq, PartialEq, Debug)]
pub struct ChunkType {
    ctype: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> std::result::Result<Self, Self::Error> {
        if !value
            .iter()
            .all(|&b| ((b >= 65) && (b <= 90)) || (b >= 97) && (b <= 122))
        {
            return Result::Err("Byte value out of range");
        }

        return Result::Ok(ChunkType { ctype: value });
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.chars().all(|c| c.is_ascii_alphabetic()) {
            return Result::Err("Character value out of range");
        }
        if s.len() != 4 {
            return Result::Err("String length incorrect size");
        }
        let mut ctype: [u8; 4] = Default::default();
        ctype.copy_from_slice(s.as_bytes());
        return Result::Ok(ChunkType { ctype });
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match std::str::from_utf8(&self.ctype) {
            Err(_e) => Err(std::fmt::Error {}),
            Ok(s) => write!(f, "{}", s),
        }
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        return self.ctype;
    }

    fn is_valid(&self) -> bool {
        let in_range: bool = self
            .ctype
            .iter()
            .all(|&b| ((b >= 65) && (b <= 90)) || (b >= 97) && (b <= 122));
        return in_range && self.is_reserved_bit_valid();
    }

    fn is_critical(&self) -> bool {
        return (self.ctype[0] >> 5) & 0b1 == 0b0;
    }
    fn is_public(&self) -> bool {
        return (self.ctype[1] >> 5) & 0b1 == 0b0;
    }
    fn is_reserved_bit_valid(&self) -> bool {
        return (self.ctype[2] >> 5) & 0b1 == 0b0;
    }
    fn is_safe_to_copy(&self) -> bool {
        return (self.ctype[3] >> 5) & 0b1 == 0b1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
