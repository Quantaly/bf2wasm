mod instr;

use super::BrainfuckSyntax::*;
use super::{BrainfuckSyntax, CellSize, CompilerOptions};
use instr::*;
use leb128;
use std::convert::{TryFrom, TryInto};
use std::io;
use std::io::prelude::*;

const WASM_PAGE_SIZE: u32 = 65536;

const WASM_PRELUDE: &'static [u8] = &[
    0x00, 0x61, 0x73, 0x6d, // magic
    0x01, 0x00, 0x00, 0x00, // version
];

const TYPE_SECTION: &'static [u8] = &[
    0x01, // id
    0x0c, // byte length
    0x03, // vec length
    0x60, 0x00, 0x01, 0x7f, // type 0: () -> (i32)
    0x60, 0x01, 0x7f, 0x00, // type 1: (i32) -> ()
    0x60, 0x00, 0x00, // type 2: () -> ()
];

const IMPORT_SECTION: &'static [u8] = &[
    0x02, // id
    0x22, // byte length
    0x02, // vec length
    // import func 0: io.get_value() -> (i32)
    0x02, 0x69, 0x6f, // mod: `io`
    0x0a, 0x72, 0x65, 0x61, 0x64, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, // nm: `read_value`
    0x00, 0x00, // func() -> (i32) [type 0]
    // import func 1: io.put_value(i32) -> ()
    0x02, 0x69, 0x6f, // mod: `io`
    0x0b, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x75,
    0x65, // nm: `write_value`
    0x00, 0x01, // func(i32) -> () [type 1]
];

const FUNCTION_SECTION: &'static [u8] = &[
    0x03, // id
    0x02, // byte length
    0x01, // vec length
    // func 2: () -> ()
    0x02, // type 2
];

/* memory section */
//0x05, // id
//0x04, // byte length
//0x01, // vec length
// memory 0
//0x01, 0x01, 0x01, // min 1, max 1

const START_SECTION: &'static [u8] = &[
    0x08, // id
    0x01, // byte length
    0x02, // function 2
];

// TODO: The return value's Err case should be an enum encompassing both IO errors and integer overflows
pub fn compile_wasm(
    ast: &Vec<BrainfuckSyntax>,
    options: &CompilerOptions,
    output: &mut impl Write,
) -> io::Result<()> {
    output.write_all(WASM_PRELUDE)?;
    output.write_all(TYPE_SECTION)?;
    output.write_all(IMPORT_SECTION)?;
    output.write_all(FUNCTION_SECTION)?;

    /* memory section */
    let pages = u32_leb128(num_pages(options));
    let mem_section_len = u32_leb128((pages.len() * 2 + 2).try_into().unwrap());
    output.write_all(&[0x05])?; // memory section id
    output.write_all(&mem_section_len)?; // memory section length
    output.write_all(&[
        0x01, // memory vector length
        0x01, // upper-bounded size
    ])?;
    output.write_all(&pages)?; // initial size
    output.write_all(&pages)?; // maximum size

    output.write_all(START_SECTION)?;

    /* code section */
    let mut func = vec![
        0x01, // locals vector length
        0x01, 0x7f, // one local of type i32
    ];
    emit_func_body(&ast, &options.cell_size, &mut func)?;
    func.push(end()[0]);
    let func_len = u32_leb128(func.len().try_into().unwrap());

    let section_len = u32_leb128((func.len() + func_len.len() + 1).try_into().unwrap());

    output.write_all(&[0x0a])?; // code section id
    output.write_all(&section_len)?; // code section length
    output.write_all(&[0x01])?; // function vector length
    output.write_all(&func_len)?; // function 2 byte length (0 and 1 are the imports)
    output.write_all(&func)?; // function 2 contents

    Ok(())
}

fn emit_func_body(
    ast: &Vec<BrainfuckSyntax>,
    sz: &CellSize,
    output: &mut impl Write,
) -> io::Result<()> {
    for syntax in ast {
        match syntax {
            MovePointer(value) => {
                output.write_all(&local_get(0))?;
                output.write_all(&i32_const(
                    (*value) * i32::try_from(sz.byte_length()).unwrap(), // stay aligned - we know this will work, largest byte_length is 8 (fits in an i32)
                ))?;
                output.write_all(&i32_add())?;
                output.write_all(&local_set(0))?;
            }
            ModifyValue(value) => {
                output.write_all(&local_get(0))?;
                output.write_all(&local_get(0))?;
                output.write_all(&sz.isz_load())?;
                output.write_all(&sz.isz_const(*value))?;
                output.write_all(&sz.isz_add())?;
                output.write_all(&sz.isz_store())?;
            }
            Output => {
                output.write_all(&local_get(0))?;
                output.write_all(&sz.isz_load())?;
                output.write_all(&sz.isz_to_i32())?;
                output.write_all(&call(1))?; // <io.write_value>
            }
            Input => {
                output.write_all(&local_get(0))?;
                output.write_all(&call(0))?; // <io.read_value>
                output.write_all(&sz.i32_to_isz())?;
                output.write_all(&sz.isz_store())?;
            }
            Loop(contents) => {
                output.write_all(&wasm_loop(BlockType::Void))?;
                output.write_all(&local_get(0))?;
                output.write_all(&sz.isz_load())?;
                output.write_all(&sz.ne_zero())?;
                output.write_all(&wasm_if(BlockType::Void))?;
                emit_func_body(contents, sz, output)?;
                output.write_all(&br(1))?;
                output.write_all(&end())?;
                output.write_all(&end())?;
            }
            NoOp => {}
        }
    }
    Ok(())
}

fn num_pages(options: &CompilerOptions) -> u32 {
    let cell_bytes = options.cell_size.byte_length();
    let cells_per_page = WASM_PAGE_SIZE / cell_bytes;
    let whole_pages = options.num_cells / cells_per_page;
    if options.num_cells % cells_per_page > 0 {
        whole_pages + 1
    } else {
        whole_pages
    }
}

fn truncate_i64(value: i64) -> i32 {
    let bytes = value.to_le_bytes();
    i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn i32_leb128(value: i32) -> Vec<u8> {
    let mut ret = Vec::with_capacity(5);
    leb128::write::signed(&mut ret, value.into()).unwrap();
    ret
}

fn u32_leb128(value: u32) -> Vec<u8> {
    let mut ret = Vec::with_capacity(5);
    leb128::write::unsigned(&mut ret, value.into()).unwrap();
    ret
}

fn i64_leb128(value: i64) -> Vec<u8> {
    let mut ret = Vec::with_capacity(10);
    leb128::write::signed(&mut ret, value).unwrap();
    ret
}
