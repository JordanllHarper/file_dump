mod converter;
use std::{
    fs,
    io::{self, Read, stdin},
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
    infile: Option<String>,
    outfile: Option<String>,

    /// Number of cols to render per line. Default is 16.
    #[arg(short, long)]
    cols: Option<usize>,
}

fn main() {
    let args = Cli::parse();
    if let Err(e) = run(args) {
        println!("Failed with error: {}", e);
    }
}
fn run(args: Cli) -> Result<(), std::io::Error> {
    let contents = if let Some(filepath) = args.infile
        && filepath != "-"
    {
        fs::read(filepath)?
    } else {
        let mut buf = Vec::new();
        stdin().read_to_end(&mut buf)?;
        buf
    };
    let lines = convert(&contents, args.cols.unwrap_or(16));
    if let Some(outfile) = args.outfile
        && outfile != "-"
    {
        let raw = lines
            .iter()
            .map(|line| format!("{}\n", line))
            .collect::<String>();
        fs::write(outfile, raw)?;
    } else {
        for line in lines {
            println!("{}", line);
        }
    }

    Ok(())
}
