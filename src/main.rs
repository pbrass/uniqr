use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `uniq`
struct Args {
    /// Input file  
    #[arg(value_name = "FILE", default_value = "-")]
    file: String,

    /// print the counts
    #[arg(short('c'), long("count"))]
    count: usize,
}

fn main() {
    println!("Hello, world!");
}
