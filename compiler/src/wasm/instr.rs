use super::super::CellSize;
use super::*;

pub fn local_get(x: u32) -> Vec<u8> {
    let mut ret = vec![0x20];
    ret.append(&mut u32_leb128(x));
    ret
}

pub fn i32_const(n: i32) -> Vec<u8> {
    let mut ret = vec![0x41];
    ret.append(&mut i32_leb128(n));
    ret
}

pub fn i32_add() -> [u8; 1] {
    [0x6a]
}

pub fn i32_eqz() -> [u8; 1] {
    [0x45]
}

pub fn i32_and() -> [u8; 1] {
    [0x71]
}

pub fn local_set(x: u32) -> Vec<u8> {
    let mut ret = vec![0x21];
    ret.append(&mut u32_leb128(x));
    ret
}

pub fn local_tee(x: u32) -> Vec<u8> {
    let mut ret = vec![0x22];
    ret.append(&mut u32_leb128(x));
    ret
}

/*pub fn i32_load(align: u32, offset: u32) -> Vec<u8> {
    let mut ret = vec![0x28];
    ret.append(&mut u32_leb128(align));
    ret.append(&mut u32_leb128(offset));
    ret
}

pub fn i32_store(align: u32, offset: u32) -> Vec<u8> {
    let mut ret = vec![0x36];
    ret.append(&mut u32_leb128(align));
    ret.append(&mut u32_leb128(offset));
    ret
}*/

pub fn call(x: u32) -> Vec<u8> {
    let mut ret = vec![0x10];
    ret.append(&mut u32_leb128(x));
    ret
}

pub enum BlockType {
    Void,
    //I32,
    //I64,
    //F32,
    //F64,
}

impl BlockType {
    pub fn code(&self) -> u8 {
        match self {
            BlockType::Void => 0x40,
            //BlockType::I32 => 0x7f,
            //BlockType::I64 => 0x7e,
            //BlockType::F32 => 0x7d,
            //BlockType::F64 => 0x7c,
        }
    }
}

pub fn wasm_loop(rt: BlockType) -> [u8; 2] {
    [0x03, rt.code()]
}

pub fn wasm_if(rt: BlockType) -> [u8; 2] {
    [0x04, rt.code()]
}

pub fn wasm_else() -> [u8; 1] {
    [0x05]
}

pub fn br(l: u32) -> Vec<u8> {
    let mut ret = vec![0x0c];
    ret.append(&mut u32_leb128(l));
    ret
}

pub fn end() -> [u8; 1] {
    [0x0b]
}

pub trait SizeDependentInstructions {
    fn isz_const(&self, n: i64) -> Vec<u8>;
    fn isz_add(&self) -> Vec<u8>;
    fn isz_load(&self) -> Vec<u8>;
    fn isz_store(&self) -> Vec<u8>;
    /// Hey, did you know that `if`s and stuff in Wasm only accept `i32`s and not `i64`s?
    fn ne_zero(&self) -> Vec<u8>;
    fn isz_to_i8(&self) -> Vec<u8>;
    fn i8_to_isz(&self) -> Vec<u8>;
}

impl SizeDependentInstructions for CellSize {
    fn isz_const(&self, n: i64) -> Vec<u8> {
        match self {
            CellSize::I8 => {
                let mut ret = vec![0x41]; // i32.const
                ret.append(&mut i32_leb128(truncate_i64(n) & 0xff));
                ret
            }
            CellSize::I16 => {
                let mut ret = vec![0x41]; // i32.const
                ret.append(&mut i32_leb128(truncate_i64(n) & 0xffff));
                ret
            }
            CellSize::I32 => {
                let mut ret = vec![0x41]; // i32.const
                ret.append(&mut i32_leb128(truncate_i64(n)));
                ret
            }
            CellSize::I64 => {
                let mut ret = vec![0x42]; // i64.const
                ret.append(&mut i64_leb128(n));
                ret
            }
        }
    }
    fn isz_add(&self) -> Vec<u8> {
        match self {
            CellSize::I64 => vec![0x7c], // i64.add
            _ => vec![0x6a],             // i32.add
        }
    }

    fn isz_load(&self) -> Vec<u8> {
        match self {
            CellSize::I8 => vec![0x2d, 0x00, 0x00], // i32.load8_u align=1 offset=0
            CellSize::I16 => vec![0x2f, 0x01, 0x00], // i32.load16_u align=2 offset=0
            CellSize::I32 => vec![0x28, 0x02, 0x00], // i32.load align=4 offset=0
            CellSize::I64 => vec![0x29, 0x03, 0x00], // i32.load align=8 offset=0
        }
    }

    fn isz_store(&self) -> Vec<u8> {
        match self {
            CellSize::I8 => vec![0x3a, 0x00, 0x00], // i32.store8 align=1 offset=0
            CellSize::I16 => vec![0x3b, 0x01, 0x00], // i32.store16 align=2 offset=0
            CellSize::I32 => vec![0x36, 0x02, 0x00], // i32.store align=4 offset=0
            CellSize::I64 => vec![0x37, 0x03, 0x00], // i64.store align=8 offset=0
        }
    }

    fn ne_zero(&self) -> Vec<u8> {
        match self {
            CellSize::I64 => vec![0x50, 0x45], // i64.eqz; i32.eqz // invert it twice
            _ => vec![],
        }
    }

    fn isz_to_i8(&self) -> Vec<u8> {
        let mut ret = vec![];
        match self {
            CellSize::I64 => {
                ret.push(0xa7); // i32.wrap_i64
            },
            _ => {}
        }
        match self {
            CellSize::I8 => {}
            _ => {
                ret.push(0x41); // i32.const
                ret.append(&mut i32_leb128(0xff));
                ret.push(0x71); // i32.and
            }
        }
        ret
    }

    fn i8_to_isz(&self) -> Vec<u8> {
        match self {
            CellSize::I64 => vec![0xad], // i64.extend_i32_u
            _ => vec![],
        }
    }
}
