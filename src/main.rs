use std::fs;

use clap::Parser;

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
    let chunks = convert_to_chunks(&contents);
    for (offset, mut chunk, str_chunk) in chunks {
        let diff = 39 - chunk.len();
        for _ in 0..diff {
            chunk.push(' ');
        }
        println!("{}: {} {}", offset, chunk, str_chunk);
    }

    Ok(())
}

type Line = (String, String, String);

fn convert_to_chunks(bytes: &[u8]) -> Vec<Line> {
    if bytes.is_empty() {
        return Vec::new();
    }
    let parts = bytes
        .iter()
        .map(|b| {
            let byte_as_ascii = if b.is_ascii_graphic() || b.is_ascii_alphanumeric() || *b == b' ' {
                *b as char
            } else {
                '.'
            };

            (format!("{:02x}", b), byte_as_ascii)
        })
        .collect::<Vec<(String, char)>>();

    let hexs_and_chars = parts.chunks(2).filter_map(|each| {
        let first = each.first()?;
        let Some(last) = each.get(1) else {
            return Some((first.0.to_string(), first.1.to_string()));
        };

        Some((
            format!("{}{}", first.0, last.0),
            format!("{}{}", first.1, last.1),
        ))
    });

    let mut formatted = Vec::new();

    let mut hex_buf = Vec::new();
    let mut c_buf = Vec::new();
    let mut offset = 0;

    for (i, (hex, c)) in hexs_and_chars.enumerate() {
        if i > 0 && i % 8 == 0 {
            let hex_line = hex_buf.join(" ");
            formatted.push((format!("{:08x}", offset), hex_line, c_buf.join("")));

            hex_buf.clear();
            c_buf.clear();
            offset += 16
        }

        hex_buf.push(hex);
        c_buf.push(c);
    }

    formatted.push((format!("{:08x}", offset), hex_buf.join(" "), c_buf.join("")));

    formatted
}

#[cfg(test)]
mod test {
    use crate::convert_to_chunks;

    #[test]
    fn convert_to_hex_chunks_handles_16_chunks_correctly() {
        let input = b"[package]
name =";
        let result = convert_to_chunks(input);
        let (offset, hex, literal) = result.first().expect("there should be 1 item");
        assert_eq!("00000000", offset);
        assert_eq!("5b70 6163 6b61 6765 5d0a 6e61 6d65 203d", hex);
        assert_eq!("[package].name =", literal);
    }
    #[test]
    fn convert_to_hex_chunks_handles_multiple_lines_of_chunks() {
        let input = b"[package]
name = \"hex_dump\"
version = \"0.1.0\"
edition = \"2024\"

[dependencies]
clap = { version = \"4.6.6\", features = [\"derive\"] }
";
        let result = convert_to_chunks(input);
        let (offset, hex, literal) = result.first().expect("there should be 1 item");
        assert_eq!("00000000", offset);
        assert_eq!("5b70 6163 6b61 6765 5d0a 6e61 6d65 203d", hex);
        assert_eq!("[package].name =", literal);
    }

    #[test]
    fn convert_to_hex_chunks_handles_non_16_chunks_correctly() {
        let input = b"[package]";
        let result = convert_to_chunks(input);
        let (offset, hex, literal) = result.first().expect("there should be 1 item");
        assert_eq!("00000000", offset);
        assert_eq!("5b70 6163 6b61 6765 5d", hex);
        assert_eq!("[package]", literal);
    }

    #[test]
    fn convert_to_hex_chunks_handles_empty_contents() {
        let input = b"";
        let result = convert_to_chunks(input);
        assert!(result.is_empty());
    }
}
