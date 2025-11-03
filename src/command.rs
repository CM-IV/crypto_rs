use blake3::Hasher;
use camino::Utf8PathBuf;
use std::io::BufWriter;
use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use age::{secrecy::SecretString, DecryptError, Identity};
use anyhow::Result;
use clap::{Parser, Subcommand};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use owo_colors::OwoColorize;
use rand::{distr::Uniform, prelude::Distribution};

const WORDLIST: &str = include_str!("./assets/wordlist.txt");

const PASSWORD_LEN: usize = 10;

#[derive(Parser)]
#[clap(
    author = "CM-IV <chuck@civdev.xyz>",
    version,
    long_about = r#"
File encryption software
By CM-IV <chuck@civdev.xyz>
"#
)]
pub struct CryptoRS {
    #[clap(subcommand, value_enum)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// File encryption, a password is generated for you
    Encrypt {
        /// The path to the file
        path: Utf8PathBuf,
    },
    /// File decryption, provide the generated password
    Decrypt {
        /// Required password to decrypt a file
        #[arg(short, long)]
        pass: String,

        /// The path to the file
        path: Utf8PathBuf,
    },
    /// Use Blake3 to generate a hash for the unencrypted file
    Hash {
        /// The path to the file for Blake3 hash
        path: Utf8PathBuf,
    },
}

impl CryptoRS {
    fn generate_password() -> Result<String> {
        // Algorithm to generate random password phrase

        let mut rng = rand::rng();

        let between = Uniform::new(0, 2047).expect("Uniform");

        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                WORDLIST
                    .lines()
                    .nth(between.sample(&mut rng))
                    .expect("index in range")
            })
            .collect::<Vec<_>>()
            .join("-");

        Ok(password)
    }
    fn encrypt_file(file: &Utf8PathBuf, pass: String) -> Result<()> {
        let input_file = File::open(file)?;
        let dir = dirs::download_dir().expect("Couldn't get downloads dir!");
        let output_path = format!("{}/{}.age", dir.display(), file.file_name().unwrap());
        let output_file = File::create(output_path)?;

        println!("\n{}\n", "Encrypting...".yellow());

        let encryptor = age::Encryptor::with_user_passphrase(SecretString::from(pass.as_str()));
        let mut writer = encryptor.wrap_output(BufWriter::new(output_file))?;

        // Use 64kb buffered reading
        let mut reader = BufReader::with_capacity(64 * 1024, input_file);
        let mut buffer = [0; 64 * 1024];

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            writer.write_all(&buffer[..bytes_read])?;
        }

        writer.finish()?;

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Your Password"])
            .add_row(vec![pass]);

        println!("{}", table.green());
        println!("\n{}\n", "Done!".green());

        Ok(())
    }
    fn decrypt_file(file: &Utf8PathBuf, pass: String) -> Result<()> {
        let input_file = File::open(file)?;
        let dir = dirs::download_dir().expect("Couldn't get downloads dir!");
        let output_path = format!(
            "{}/{}",
            dir.display(),
            file.file_name().unwrap().strip_suffix(".age").unwrap()
        );
        let output_file = File::create(output_path)?;

        println!("{}", "\nDecrypting...".yellow());

        let decryptor = age::Decryptor::new(BufReader::with_capacity(64 * 1024, input_file))?;
        let identity: Box<dyn Identity> = Box::new(age::scrypt::Identity::new(pass.into()));

        let mut reader = decryptor.decrypt(std::iter::once(identity.as_ref()))?;
        let mut writer = BufWriter::with_capacity(64 * 1024, output_file);
        let mut buffer = [0; 64 * 1024];

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            writer.write_all(&buffer[..bytes_read])?;
        }

        writer.flush()?;

        println!("{}", "\nFile successfully decrypted!\n".green());

        Ok(())
    }
    fn gen_file_hash(file: &Utf8PathBuf) -> Result<()> {
        println!("{}", "\nHashing...\n".yellow());

        let f = File::open(file)?;
        let mut reader = BufReader::with_capacity(64 * 1024, f);

        let mut hasher = Hasher::new();
        let mut buffer = [0; 64 * 1024];

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        // BLAKE3 has built-in hex formatting
        let hash_str = hasher.finalize().to_hex().to_string();

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["BLAKE3 Hash"])
            .add_row(vec![hash_str]);

        println!("{}", table.green());

        Ok(())
    }

    pub fn exec(self) -> Result<()> {
        match self.command {
            Command::Encrypt { path } => {
                if !path.is_file() {
                    println!(
                        "{}",
                        "\nYou cannot use two commands at once and the path must lead to a file\n"
                            .red()
                    );
                    return Ok(());
                }

                let generated_pass = Self::generate_password()?;

                Self::encrypt_file(&path, generated_pass)?;
            }
            Command::Decrypt { pass, path } => {
                if !path.is_file() {
                    println!("{}", "\nYou cannot use two commands at once and the path must lead to a '.age' file\n".red());
                    return Ok(());
                }

                let result = Self::decrypt_file(&path, pass);

                match result {
                    Ok(()) => {
                        return Ok(());
                    }
                    Err(err) => {
                        if err.downcast::<DecryptError>().is_ok() {
                            println!("{}", "\nAn error occured during decryption, is the file a valid '.age' file?\n".red());
                            return Ok(());
                        }
                    }
                }
            }
            Command::Hash { path } => {
                if !path.is_file() {
                    println!(
                        "{}",
                        "\nYou cannot use two commands at once and the path must lead to a file\n"
                            .red()
                    );
                    return Ok(());
                }

                Self::gen_file_hash(&path)?;
            }
        }

        Ok(())
    }
}
