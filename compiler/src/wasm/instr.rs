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

pub fn local_set(x: u32) -> Vec<u8> {
    let mut ret = vec![0x21];
    ret.append(&mut u32_leb128(x));
    ret
}

pub fn i32_load(align: u32, offset: u32) -> Vec<u8> {
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
}

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

pub fn br(l: u32) -> Vec<u8> {
    let mut ret = vec![0x0c];
    ret.append(&mut u32_leb128(l));
    ret
}

pub fn end() -> [u8; 1] {
    [0x0b]
}