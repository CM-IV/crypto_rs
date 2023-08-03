# crypto_rs

*DO NOT FORGET YOUR PASSWORD, I AM NOT RESPONSIBLE IF YOU DO!!!*

A simple program for **Linux** that encrypts/decrypts files via a generated 10 word passphrase.  The encrypted/decrypted files are written to the user's `Downloads` folder.  Once the file is encrypted, you will see a `.age` file extension added to the end of the file in the `Downloads` folder.  This is the file you can backup to Google Drive, Deta Drive, Dropbox, etc.

File encryption will generate a 10 word long passphrase for you using a wordlist with 2048 total words in it.  Keep track of these generated passcodes as they are needed for decrypting your `.age` files later on.

This branch of the repository uses the `fltk-rs` crate, so a lightweight GUI is used in order to work with `crypto_rs`.  This branch also features SHA-512 hashing via the `hmac-sha512` crate.  You can use this functionality via the `hash` tab file picker in order to generate a hash string both before encryption and after an encrypted file has been decrypted to ensure file integrity.

## build instructions

1. Ensure that the latest Rust version is installed - find that [here](https://www.rust-lang.org/learn/get-started)
2. `git clone -b fltk` this repo
3. Consult [fltk book](https://fltk-rs.github.io/fltk-book/Setup.html) for distro specific setup
4. Run `RUSTFLAGS="-C target-cpu=native" cargo build --release` (or just `cargo build --release`)
5. Your executable should be in the `/target/release` directory

![](https://ik.imagekit.io/xbkhabiqcy9/img/image_DH_2bqEH7.png?updatedAt=1691073812608)