mod converter;
use std::{
    fmt::{self},
    fs,
};

use clap::Parser;

use crate::converter::convert;

/// Dump hex bytes of a given file in chunks to stdout. An xxd clone.
///
/// The columns output as follows:
///
/// 1 - The current offset in hexidecimal format.
///
/// 2 - The hex values formatted in chunks of 2.
///
/// 3 - The raw text of the file which the hex references. If not a displayable symbol, will be displayed
///     as ".". e.g. a newline or a space.
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Cli {
    filepath: String,
}

fn main() {
    let args = Cli::parse();
    if let Err(e) = run(args) {
        println!("Failed with error: {}", e);
    }
}
fn run(args: Cli) -> Result<(), std::io::Error> {
    let contents = fs::read(args.filepath)?;
    let lines = convert(&contents);
    for line in lines {
        println!("{}", line);
    }

    Ok(())
}
