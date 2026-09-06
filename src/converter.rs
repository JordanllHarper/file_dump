use std::{collections::binary_heap::Iter, fmt};

pub struct Line {
    offset: String,
    output: String,
    output_ascii: String,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let diff = 39 - self.output.len();
        let mut chunk = self.output.clone();
        for _ in 0..diff {
            chunk.push(' ');
        }
        write!(f, "{}: {} {}", self.offset, chunk, self.output_ascii)
    }
}

impl Line {
    fn new(offset: String, output: String, output_ascii: String) -> Self {
        Self {
            offset,
            output,
            output_ascii,
        }
    }
}

type Part = (String, char);

fn get_parts(bytes: &[u8]) -> Vec<Part> {
    bytes
        .iter()
        .map(|b| {
            let byte_as_ascii = if b.is_ascii_graphic() || b.is_ascii_alphanumeric() || *b == b' ' {
                *b as char
            } else {
                '.'
            };

            (format!("{:02x}", b), byte_as_ascii)
        })
        .collect::<Vec<Part>>()
}

fn get_hex_and_text_iter(parts: &[Part]) -> impl Iterator<Item = (String, String)> {
    parts.chunks(2).filter_map(|each| {
        let first = each.first()?;
        let Some(last) = each.get(1) else {
            return Some((first.0.to_string(), first.1.to_string()));
        };

        Some((
            format!("{}{}", first.0, last.0),
            format!("{}{}", first.1, last.1),
        ))
    })
}

pub fn convert(bytes: &[u8]) -> Vec<Line> {
    if bytes.is_empty() {
        return Vec::new();
    }
    let parts = get_parts(bytes);
    let hex_and_text_iter = get_hex_and_text_iter(&parts);
    build_lines(hex_and_text_iter)
}

fn build_lines(hex_and_text_iter: impl Iterator<Item = (String, String)>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();

    let mut hex_buf = Vec::new();
    let mut c_buf = Vec::new();
    let mut offset = 0;

    for (i, (hex, c)) in hex_and_text_iter.enumerate() {
        if i > 0 && i % 8 == 0 {
            let hex_line = hex_buf.join(" ");
            lines.push(Line::new(
                format!("{:08x}", offset),
                hex_line,
                c_buf.join(""),
            ));

            hex_buf.clear();
            c_buf.clear();
            offset += 16
        }

        hex_buf.push(hex);
        c_buf.push(c);
    }

    lines.push(Line::new(
        format!("{:08x}", offset),
        hex_buf.join(" "),
        c_buf.join(""),
    ));

    lines
}

#[cfg(test)]
mod test {
    use crate::converter::convert;

    #[test]
    fn convert_to_hex_chunks_handles_16_chunks_correctly() {
        let input = b"[package]
name =";
        let result = convert(input);
        let line = result.first().expect("there should be 1 item");

        assert_eq!("00000000", line.offset);
        assert_eq!("5b70 6163 6b61 6765 5d0a 6e61 6d65 203d", line.output);
        assert_eq!("[package].name =", line.output_ascii);
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
        let result = convert(input);
        let line = result.first().expect("there should be 1 item");
        assert_eq!("00000000", line.offset);
        assert_eq!("5b70 6163 6b61 6765 5d0a 6e61 6d65 203d", line.output);
        assert_eq!("[package].name =", line.output_ascii);
    }

    #[test]
    fn convert_to_hex_chunks_handles_non_16_chunks_correctly() {
        let input = b"[package]";
        let result = convert(input);
        let line = result.first().expect("there should be 1 item");
        assert_eq!("00000000", line.offset);
        assert_eq!("5b70 6163 6b61 6765 5d", line.output);
        assert_eq!("[package]", line.output_ascii);
    }

    #[test]
    fn convert_to_hex_chunks_handles_empty_contents() {
        let input = b"";
        let result = convert(input);
        assert!(result.is_empty());
    }
}
