use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    offset: String,
    output: String,
    output_ascii: String,
    trailing_padding: usize,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = self.output.clone();
        for _ in 0..self.trailing_padding {
            output.push(' ');
        }
        write!(f, "{}: {} {}", self.offset, output, self.output_ascii)
    }
}

impl Line {
    fn new(offset: &str, output: &str, output_ascii: &str, trailing_padding: usize) -> Self {
        Self {
            offset: offset.to_string(),
            output: output.to_string(),
            output_ascii: output_ascii.to_string(),
            trailing_padding,
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

pub fn convert(bytes: &[u8], cols: usize) -> Vec<Line> {
    if bytes.is_empty() {
        return Vec::new();
    }
    let parts = get_parts(bytes);
    let hex_and_text_iter = get_hex_and_text_iter(&parts).collect::<Vec<(String, String)>>();
    build_lines(&hex_and_text_iter, cols.checked_div(2).unwrap_or(8), cols)
}

fn build_lines(hex_text_pairs: &[(String, String)], chunks: usize, cols: usize) -> Vec<Line> {
    hex_text_pairs
        .chunks(chunks)
        .enumerate()
        .map(|(chunk_idx, chunk)| {
            let output = chunk
                .iter()
                .map(|each| each.0.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            let output_ascii = chunk
                .iter()
                .map(|each| each.1.to_string())
                .collect::<Vec<String>>()
                .join("");

            // include the last trailing space
            let spaces = chunks - 1;
            let occupied_cols = 4 * chunks;
            let max_len = occupied_cols + spaces;
            let diff = max_len - output.len();

            Line::new(
                &format!("{:08x}", chunk_idx * cols),
                &output,
                &output_ascii,
                diff,
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::converter::{Line, convert};
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn convert_to_hex_chunks_handles_16_chunks_correctly() {
        let input = b"[package]
name =";
        let result = convert(input, 16);
        let line = result.first().expect("there should be 1 item");

        assert_eq!("00000000", line.offset);
        assert_eq!("5b70 6163 6b61 6765 5d0a 6e61 6d65 203d", line.output);
        assert_eq!("[package].name =", line.output_ascii);
    }

    #[test]
    fn convert_to_hex_chunks_handles_multiple_lines_of_chunks() {
        let input = b"[package]
name = \"file_dump\"
version = \"0.1.0\"
edition = \"2024\"

[dependencies]
clap = { version = \"4.6.6\", features = [\"derive\"] }
";
        let expected_output = vec![
            Line::new(
                "00000000",
                "5b70 6163 6b61 6765 5d0a 6e61 6d65 203d",
                "[package].name =",
                0,
            ),
            Line::new(
                "00000010",
                "2022 6669 6c65 5f64 756d 7022 0a76 6572",
                " \"file_dump\".ver",
                0,
            ),
            Line::new(
                "00000020",
                "7369 6f6e 203d 2022 302e 312e 3022 0a65",
                "sion = \"0.1.0\".e",
                0,
            ),
            Line::new(
                "00000030",
                "6469 7469 6f6e 203d 2022 3230 3234 220a",
                "dition = \"2024\".",
                0,
            ),
            Line::new(
                "00000040",
                "0a5b 6465 7065 6e64 656e 6369 6573 5d0a",
                ".[dependencies].",
                0,
            ),
            Line::new(
                "00000050",
                "636c 6170 203d 207b 2076 6572 7369 6f6e",
                "clap = { version",
                0,
            ),
            Line::new(
                "00000060",
                "203d 2022 342e 362e 3622 2c20 6665 6174",
                " = \"4.6.6\", feat",
                0,
            ),
            Line::new(
                "00000070",
                "7572 6573 203d 205b 2264 6572 6976 6522",
                "ures = [\"derive\"",
                0,
            ),
            Line::new("00000080", "5d20 7d0a", "] }.", 30),
        ];
        let result = convert(input, 16);
        assert_eq!(expected_output, result);
    }

    #[test]
    fn convert_to_hex_chunks_handles_different_col_values() {
        let input = b"[package]
name = \"file_dump\"
version = \"0.1.0\"
edition = \"2024\"
";
        let expected_output = vec![
            Line::new("00000000", "5b70 6163 6b61 6765", "[package", 0),
            Line::new("00000008", "5d0a 6e61 6d65 203d", "].name =", 0),
            Line::new("00000010", "2022 6669 6c65 5f64", " \"file_d", 0),
            Line::new("00000018", "756d 7022 0a76 6572", "ump\".ver", 0),
            Line::new("00000020", "7369 6f6e 203d 2022", "sion = \"", 0),
            Line::new("00000028", "302e 312e 3022 0a65", "0.1.0\".e", 0),
            Line::new("00000030", "6469 7469 6f6e 203d", "dition =", 0),
            Line::new("00000038", "2022 3230 3234 220a", " \"2024\".", 0),
        ];
        let result = convert(input, 8);
        assert_eq!(expected_output, result);
    }

    #[test]
    fn convert_to_hex_chunks_handles_non_16_chunks_correctly() {
        let input = b"[package]";
        let result = convert(input, 16);
        let line = result.first().expect("there should be 1 item");
        assert_eq!("00000000", line.offset);
        assert_eq!("5b70 6163 6b61 6765 5d", line.output);
        assert_eq!("[package]", line.output_ascii);
    }

    #[test]
    fn convert_to_hex_chunks_handles_empty_contents() {
        let input = b"";
        let result = convert(input, 16);
        assert!(result.is_empty());
    }
}
