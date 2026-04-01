# Enseal

**Enseal** is a high-performance, security-focused CLI tool designed to protect your `.env` files using industry-standard encryption. Built with Rust, it allows you to safely commit your environment configurations to version control without exposing sensitive secrets.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Rust](https://img.shields.io/badge/rust-%23E32F26.svg?style=flat&logo=rust&logoColor=white)

---

## Features

- **Secure Encryption**: Utilizes strong encryption algorithms to protect your .env files.
- **User-Friendly CLI**: Intuitive command-line interface for easy encryption and decryption of .env files.
- **Cross-Platform**: Compatible with Windows, macOS, and Linux.
- **Fast Performance**: Leveraging Rust's performance capabilities to ensure quick encryption and decryption processes.
- **Open Source**: Available on GitHub for contributions and improvements.

## 🛠️ Tech Stack

- **Language**: Rust
- **Encryption**: `aes-gcm`
- **Key Derivation**: `argon2`
- **TUI**: `inquire`, `indicatif`, `colored`

## License

Enseal is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
