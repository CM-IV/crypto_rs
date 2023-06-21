//! # crypto_rs
//!
//! `crypto_rs` is a Linux terminal program for file encryption.
//!
//! ```bash
//! File encryption software
//! By CM-IV <chuck@civdev.xyz>
//!
//!
//! Usage: crypto_rs <COMMAND>
//!
//! Commands:
//!   encrypt  File encryption, a password is generated for you
//!   decrypt  File decryption, provide the generated password
//!   help     Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -h, --help
//!           Print help (see a summary with '-h')
//!
//!   -V, --version
//!           Print version
//! ```
//! 

mod command;

#[doc(hidden)]
pub use command::CryptoRS;