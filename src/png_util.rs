use std::{
    convert::TryFrom,
    fs::File,
    io::{self, Read},
};

use crate::{chunk_type::ChunkType, png::Png, Error, Result};

/*
 * General purpose helper functions for PNG analysis
 */

pub fn read_png(filepath: &str) -> Result<Png> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut f = File::open(filepath)?;
    f.read_to_end(&mut buffer)?;
    Png::try_from(&buffer[..])
}

pub fn chunk_headers(png: Png) -> Vec<ChunkType> {
    let k = png
        .chunks()
        .iter()
        .map(|c| c.chunk_type())
        .cloned()
        .collect();
    return k;
}

/* Produce a vector of the ASCII representations of chunk headers in a png file */
pub fn chunk_headers_show(png: Png) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for chunk in png.chunks() {
        print!("{:?}", &chunk.chunk_type().bytes());
        ret.push(String::from(
            std::str::from_utf8(&chunk.chunk_type().bytes()[..]).unwrap(),
        ));
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let p = read_png("./data/png/transparent.png");
        assert!(p.is_ok());
        println!("{:?}", chunk_headers_show(p.unwrap()));
    }
}
