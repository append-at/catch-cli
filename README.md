# catch-cli

[![License: MIT](https://img.shields.io/github/license/saltstack/salt)](https://opensource.org/license/apache-2-0)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

`catch-cli` is a command-line interface (CLI) tool designed to facilitate file uploads to [Catch](https://trycatch.ai) when GitHub integration is restricted due to privacy, security policies, or other limitations.

## Overview
In scenarios where direct GitHub integration with trycatch.ai is not possible, catch-cli provides a seamless solution for uploading local files to the platform. This tool bridges the gap between your local source files and trycatch.ai, ensuring that you can continue to leverage the platform's capabilities even when faced with integration constraints.

## Installation

The binary for `catch-cli` typically does not require direct installation by users. It is designed to be easily integrated into your workflow without complex setup procedures.

## Requirements

- Rust (version 1.80 or higher)

## Building from Source

1. Clone the repository:
   ```
   git clone https://github.com/append-at/catch-cli.git
   cd catch-cli
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The binary will be available in `target/release/catch-cli`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.

## Support

If you encounter any problems or have any questions, please open an issue on the GitHub repository.
