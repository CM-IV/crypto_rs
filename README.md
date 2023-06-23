# crypto_rs

*DO NOT FORGET YOUR PASSWORD, I AM NOT RESPONSIBLE IF YOU DO!!!*

A simple terminal program for **Linux** that encrypts/decrypts files via password.  The encrypted/decrypted files are written to the user's `Downloads` folder.  Once the file is encrypted, you will see a `.age` file extension added to the end of the file in the `Downloads` folder.  This is the file you can backup to Google Drive, Deta Drive, Dropbox, etc.

This branch of the repository uses the `clap` crate, so instead of a terminal UI executed within a loop the user can quickly enter flags/arguments in order to work with `crypto_rs`.  This branch also features SHA-512 hashing via the `hmac-sha512` crate.  You can use this functionality via the `hash` command in order to generate a hash string both before encryption and after an encrypted file has been decrypted in order to ensure file integrity.

## build instructions

1. Ensure that the latest Rust version is installed - find that [here](https://www.rust-lang.org/learn/get-started)
2. `git clone` this repo
3. Run `RUSTFLAGS="-C target-cpu=native" cargo build --release` (or just `cargo build --release`)
4. Your executable should be in the `/target/release` directory

5. ![image](https://github.com/CM-IV/crypto_rs/assets/44551614/9b1e4f81-61cb-41c1-8aaa-8e6ce935f6fb)
