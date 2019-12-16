use compiler;
use compiler::parser::ParseError;
use compiler::{CellSize, CompilerOptions};
use exitcode;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    /// The file to read input from
    infile: PathBuf,
    #[structopt(parse(from_os_str))]
    /// The file to write output to
    /// 
    /// Defaults to the same name as the input file, but with ".wasm" as the extension.
    outfile: Option<PathBuf>,
    #[structopt(short, long)]
    /// The minimum number of cells potentially available to the program
    /// 
    /// The program may have more cells, but not fewer. Defaults to 32,768.
    num_cells: Option<u32>,
    #[structopt(short = "s", long, parse(try_from_str = try_parse_cell_size))]
    /// The size of a cell, in bits - one of "8", "16", "32", "64"
    /// 
    /// Defaults to 32.
    cell_size: Option<CellSize>,
}

fn try_parse_cell_size(src: &str) -> Result<CellSize, String> {
    if src == "8" {
        Ok(CellSize::I8)
    } else if src == "16" {
        Ok(CellSize::I16)
    } else if src == "32" {
        Ok(CellSize::I32)
    } else if src == "64" {
        Ok(CellSize::I64)
    } else {
        Err(format!(
            "invalid cell size {}: must be one of 8, 16, 32, 64",
            src
        ))
    }
}

fn main() {
    //simple_logger::init().unwrap();
    std::process::exit(_main());
}

fn _main() -> i32 {
    let opt = Cli::from_args();

    if let Some(size) = opt.num_cells {
        if size == 0 {
            eprintln!("please do not specify --num-cells=0");
            return exitcode::USAGE;
        }
    }

    let infile = match File::open(&opt.infile) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("could not open infile: {}", e);
            return exitcode::NOINPUT;
        }
    };
    let outfile = match File::create(match opt.outfile {
        Some(path) => path,
        None => {
            let mut outpath = opt.infile.clone();
            outpath.set_extension("wasm");
            outpath
        }
    }) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("could not open outfile: {}", e);
            return exitcode::CANTCREAT;
        }
    };

    let ast = match compiler::parse(&mut io::BufReader::new(infile)) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            return match e {
                ParseError::SyntaxError(_) => exitcode::DATAERR,
                ParseError::IoError(_) => exitcode::IOERR,
            };
        }
    };

    let default_options: CompilerOptions = Default::default();
    match compiler::compile_wasm(
        &ast,
        &CompilerOptions {
            num_cells: opt.num_cells.unwrap_or(default_options.num_cells),
            cell_size: opt.cell_size.unwrap_or(default_options.cell_size),
        },
        &mut io::BufWriter::new(outfile),
    ) {
        Err(e) => {
            eprintln!("IO error: {}", e);
            return exitcode::IOERR;
        }
        _ => (),
    }

    exitcode::OK
}
