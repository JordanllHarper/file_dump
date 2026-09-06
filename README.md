# file_dump: an xxd clone written in Rust!

This project takes a file and produces a readable hexadecimal output of the bytes in the file.

## Installation - build from source

1. Install the Rust toolchain via [rustup.rs](https://rustup.rs/) or skip if you have this already.
2. Clone the repository.
3. Run cargo build!
    - This will produce a binary you can add to your path.

## Usage

`file_dump <FILE>`

This will produce an output such as the following (which is from the project's `Cargo.toml`):

```
00000000: 5b70 6163 6b61 6765 5d0a 6e61 6d65 203d  [package].name =
00000010: 2022 6865 785f 6475 6d70 220a 7665 7273   "hex_dump".vers
00000020: 696f 6e20 3d20 2230 2e31 2e30 220a 6564  ion = "0.1.0".ed
00000030: 6974 696f 6e20 3d20 2232 3032 3422 0a0a  ition = "2024"..
00000040: 5b64 6570 656e 6465 6e63 6965 735d 0a63  [dependencies].c
00000050: 6c61 7020 3d20 7b20 7665 7273 696f 6e20  lap = { version
00000060: 3d20 2234 2e36 2e36 222c 2066 6561 7475  = "4.6.6", featu
00000070: 7265 7320 3d20 5b22 6465 7269 7665 225d  res = ["derive"]
00000080: 207d 0a                                   }.
```
The first column is the offset of the current line in a hexadecimal format.

The second column is the raw hex bytes. Each line is 32 values formatted as 2 hex values per line chunk.

The final column is the ASCII text of the file which corresponds to the hexadecimal bytes.

If a character is not displayable, such as a newline, it is replaced with a '.'.

## Further improvements

- [ ] remaining xxd features
- [ ] other customization features

## Remaining xxd features

- [ ] Reading from stdin if no file is specified or if '-' is specified
- [ ] Sending to an outfile if specified
- [ ] Specifying '-' as an outfile
- [ ] Convert a hexdump file to binary
- [ ] Option parameters work with decimal, hexadecimal or octal notation



