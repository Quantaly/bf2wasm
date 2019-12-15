use compiler;
use compiler::parser::ParseError;
use exitcode;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    infile: PathBuf,
    #[structopt(parse(from_os_str))]
    outfile: Option<PathBuf>,
}

fn main() {
    //simple_logger::init().unwrap();
    std::process::exit(_main());
}

fn _main() -> i32 {
    let opt = Cli::from_args();
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

    match compiler::compile_wasm(ast, &mut io::BufWriter::new(outfile)) {
        Err(e) => {
            eprintln!("IO error: {}", e);
            return exitcode::IOERR;
        }
        _ => (),
    }

    exitcode::OK
}
