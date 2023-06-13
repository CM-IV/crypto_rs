# crypto-rs

*DO NOT FORGET YOUR PASSWORD, I AM NOT RESPONSIBLE IF YOU DO!!!*

A simple terminal program for **Linux** that encrypts/decrypts files via password.  The encrypted/decrypted files are written to the user's `Downloads` folder.  Once the file is encrypted, you will see a `.age` file descriptor added to the end of the file in the `Downloads` folder.  This is the file you can backup to Google Drive, Deta Drive, Dropbox, etc.

File Operation Menu --> Encrypt file --> Enter file path --> Enter password --> Profit

File Operation Menu --> Decrypt file --> Enter file path -->  Enter password --> Profit

## build instructions

1. Ensure that the latest Rust version is installed - find that [here](https://www.rust-lang.org/learn/get-started)
2. `git clone` this repo
3. Run `RUSTFLAGS="-C target-cpu=native" cargo build --release` (or just `cargo build --release`)
4. Your executable should be in the `/target/release` directory

![image](https://github.com/CM-IV/crypto-rs/assets/44551614/738f57a3-8f14-48ea-a0f8-e78eef95e84f)
