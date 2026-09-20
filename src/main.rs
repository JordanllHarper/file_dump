mod converter;
use std::{
    error::Error,
    fs,
    io::{Read, stdin},
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

#[derive(Debug)]
enum FileDumpError {
    InvalidCol(usize),
}

impl std::fmt::Display for FileDumpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileDumpError::InvalidCol(cols) => write!(f, "invalid number of columns: {}", cols),
            // TODO: More errors
        }
    }
}

impl Error for FileDumpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let contents = if let Some(filepath) = args.infile
        && filepath != "-"
    {
        fs::read(filepath)?
    } else {
        let mut buf = Vec::new();
        stdin().read_to_end(&mut buf)?;
        buf
    };
    let cols = args.cols.unwrap_or(16);
    if cols == 0 {
        return Err(Box::new(FileDumpError::InvalidCol(cols)));
    }
    let lines = convert(&contents, cols);
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
