trait ChunkSpec {
    const HEADER: [u8; 4];
}

struct Chunk_IHDR {
    width: u32,    /* 0 < width <= 2^31 */
    height: u32,   /* 0 < width <= 2^31 */
    bit_depth: u8, /*1, 2, 4, 8, 16 */
    color_type: ColorType,
    compression_method: CompressionMethod,
    filter_method: FilterMethod,
    iterlace_method: u8,
}

#[repr(u8)]
enum ColorType {
    GRY = 0,
    RGB = 2,
    PLT = 3,
    GRYA = 4,
    RGBA = 6,
}

#[repr(u8)]
enum CompressionMethod {
    DeflateInflate = 0,
}

#[repr(u8)]
enum FilterMethod {
    Adaptive = 0,
}

#[repr(u8)]
enum InterlaceMethod {
    None = 0,
    Adam7 = 1,
}

impl ColorType {
    fn pallate_used(self) -> bool {
        ((self as u8) & 0x1) == 0x1
    }

    fn color_used(self) -> bool {
        ((self as u8) & 0x2) == 0x2
    }

    fn alpha_used(self) -> bool {
        ((self as u8) & 0x4) == 0x4
    }

    fn allowed_bit_depth(self, depth: u8) -> bool {
        match self {
            ColorType::GRY => {
                (depth == 0x1)
                    || (depth == 0x2)
                    || (depth == 0x4)
                    || (depth == 0x8)
                    || (depth == 0x10)
            }
            ColorType::RGB => (depth == 0x8) || (depth == 0x10),
            ColorType::PLT => (depth == 0x1) || (depth == 0x2) || (depth == 0x4) || (depth == 0x8),
            ColorType::GRYA => (depth == 0x8) || (depth == 0x10),
            ColorType::RGBA => (depth == 0x8) || (depth == 0x10),
        }
    }
}

impl Chunk_IHDR {
    fn sample_depth(self) -> u8 {
        match self.color_type {
            ColorType::PLT => 8,
            other => self.bit_depth,
        }
    }
}

struct Chunk_IDAT {}
struct Chunk_IEND {}

impl ChunkSpec for Chunk_IHDR {
    const HEADER: [u8; 4] = [73, 68, 65, 82];
}

impl ChunkSpec for Chunk_IDAT {
    const HEADER: [u8; 4] = [73, 68, 65, 84];
}

impl ChunkSpec for Chunk_IEND {
    const HEADER: [u8; 4] = [73, 68, 65, 68];
}
