use std::{
    convert::{TryFrom, TryInto},
    fmt::{self, Display},
    str::FromStr,
};

use crate::{throw_string_error, Error, Result};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ChunkType {
    ctype: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self> {
        let ret = ChunkType { ctype: value };
        if ret.is_valid() {
            return Ok(ret);
        } else {
            return Err(throw_string_error("Invalid chunk type"));
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if !s.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(throw_string_error("Character value out of range"));
        }
        if s.len() != 4 {
            return Err(throw_string_error("String length incorrect size"));
        }
        return Ok(ChunkType {
            ctype: s.as_bytes().try_into()?,
        });
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.ctype).unwrap())
    }
}

#[allow(dead_code)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.ctype;
    }

    fn is_valid(&self) -> bool {
        bytes_alphabetic(self.ctype) && self.is_reserved_bit_valid()
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

fn bytes_alphabetic(value: [u8; 4]) -> bool {
    return value
        .iter()
        .all(|&b| ((b >= 65) && (b <= 90)) || (b >= 97) && (b <= 122));
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
