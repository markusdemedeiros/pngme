use std::{
    convert::TryFrom,
    fmt::{self, Display},
};

use crc::Crc;

use crate::{chunk_type::ChunkType, throw_string_error, Error, Result};

#[derive(Eq, PartialEq, Debug)]
pub struct Chunk {
    clength: u32,
    ctype: ChunkType,
    cdata: Vec<u8>,
    ccrc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 4 {
            return Err(throw_string_error("Insufficient data to read size"));
        }
        let clength: u32 = u32::from_be_bytes([value[0], value[1], value[2], value[3]]);

        if value.len() != usize::try_from(clength).unwrap() + 12 {
            return Err(throw_string_error("Malsized chunk"));
        }
        let ctype: ChunkType = ChunkType::try_from([value[4], value[5], value[6], value[7]])?;
        let cdata: Vec<u8> = value[8..value.len() - 4].to_vec();
        let ccrc: u32 = u32::from_be_bytes([
            value[value.len() - 4],
            value[value.len() - 3],
            value[value.len() - 2],
            value[value.len() - 1],
        ]);

        let crc: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        if ccrc != crc.checksum(&value[4..value.len() - 4]) {
            return Err(throw_string_error("Chunk does not match checksum"));
        }

        return Ok(Chunk {
            clength,
            ctype,
            cdata,
            ccrc,
        });
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ Chunk (show data not implemented) ]")
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let ctype = chunk_type;
        let cdata: Vec<u8> = data;
        let clength: u32 = u32::try_from(cdata.len()).unwrap();
        let crc: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let tocrc: Vec<u8> = ctype
            .bytes()
            .iter()
            .cloned()
            .chain(cdata.iter().cloned())
            .collect();
        let ccrc: u32 = crc.checksum(&tocrc);
        return Chunk {
            clength,
            ctype,
            cdata,
            ccrc,
        };
    }
    pub fn length(&self) -> u32 {
        return self.clength;
    }
    pub fn chunk_type(&self) -> &ChunkType {
        return &self.ctype;
    }
    pub fn data(&self) -> &[u8] {
        return self.cdata.as_slice();
    }
    pub fn crc(&self) -> u32 {
        return self.ccrc;
    }
    pub fn data_as_string(&self) -> Result<String> {
        match String::from_utf8(self.cdata.clone()) {
            Ok(s) => Ok(s),
            Err(_e) => panic!(),
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        return self
            .clength
            .to_be_bytes()
            .iter()
            .cloned()
            .chain(self.ctype.bytes().iter().cloned())
            .chain(self.data().iter().cloned())
            .chain(self.ccrc.to_be_bytes().iter().cloned())
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
