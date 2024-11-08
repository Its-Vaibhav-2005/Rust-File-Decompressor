# Rust File Decompressor

A simple command-line tool for decompressing files using Rust, leveraging the [`flate2`](https://github.com/rust-lang/flate2-rs) library for efficient file decompression. This tool decompresses the specified file and saves it with a new name provided by the user.

## Features

- **Fast and Lightweight:** Built with Rust, ensuring high performance and memory safety.
- **Decompression Algorithm:** Uses the DEFLATE algorithm, provided by the `flate2` Rust crate.
- **Easy CLI Interface:** Simple command-line syntax for quick decompression.

## Installation

To use this decompressor, you need to have Rust and Cargo installed. You can install Rust by following the instructions on [rust-lang.org](https://www.rust-lang.org/).

Once Rust is installed, clone this repository and navigate to its directory.

```bash
git clone https://github.com/Its-Vaibhav-2005/Rust-File-Compressor.git
cd Rust-File-Decompressor
```

## Usage

To decompress a file, use the following command:

`cargo run ./path/to/input_filename ./path/to/output_filename`

- **`input_filename`**: Path to the file you want to decompress.
- **`output_filename`**: Path and name for the decompressed output file.

### Example

`cargo run ./data/sample.txt.gz ./data/sample.txt`

The above command decompresses `sample.txt.gz` and creates a decompressed file named `sample.txt`.

## Dependencies

This project uses the following dependency:

- [`flate2`](https://github.com/rust-lang/flate2-rs) - A compression library that supports DEFLATE and other algorithms, used here for decompression.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or suggestions.

## Acknowledgements

- [Rust Programming Language](https://www.rust-lang.org/)
- [`flate2` crate](https://github.com/rust-lang/flate2-rs) for providing the decompression functionality.
