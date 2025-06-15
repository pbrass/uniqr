use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use anyhow::Result;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `uniq`
struct Args {
    /// Input file
    #[arg(value_name = "INFILE", default_value = "-")]
    in_file: String,

    /// Output file
    #[arg(value_name = "OUTFILE")]
    out_file: Option<String>,

    /// print the counts
    #[arg(short('c'), long("count"))]
    count: bool,
}

fn open_read(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


fn open_write(filename: Option<String>) -> Result<Box<dyn Write>> {
    match filename {
        Some(filename) => Ok(Box::new(BufWriter::new(File::create(filename)?))),
        None => Ok(Box::new(BufWriter::new(io::stdout()))),
    }
}

fn run(mut _args: Args) -> Result<()> {
    let filename  = _args.in_file;
    match open_read(&filename) {
        Err(err) => {
            eprintln!("{filename}: Failed to open {err}");
            std::process::exit(1);
        },
        Ok(mut h_file) => {
            let mut buf = String::new();
            let mut rept: usize = 0;
            let mut line_bytes = h_file.read_line(&mut buf)?;
            let mut out_file = open_write(_args.out_file)?;
            buf = buf.trim_end().to_string();
            let mut oldbuf = buf.clone();
            loop {
                if oldbuf.len() >= 0 {
                    if buf != oldbuf {
                        if _args.count {
                            write!(out_file,"{rept:>7} ")?
                        }

                        write!(out_file,"{oldbuf}\n")?;
                        oldbuf = buf.clone();
                        rept = 1;
                    } else {
                        rept += 1;
                    }
                }
                if line_bytes == 0 {
                    break;
                }
                buf.clear();
                line_bytes = h_file.read_line(&mut buf)?;
                buf = buf.trim_end().to_string();
            }
        }
}
Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
