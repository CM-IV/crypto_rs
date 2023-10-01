use std::{fs::File, io::{BufReader, Read, Write}, str::FromStr};
use camino::Utf8PathBuf;

use age::{secrecy::{Secret, SecretString}, DecryptError};
use clap::{Parser, Subcommand};
use anyhow::Result;
use comfy_table::{Table, presets::UTF8_FULL, modifiers::UTF8_ROUND_CORNERS, ContentArrangement};
use owo_colors::OwoColorize;
use rand::{distributions::Uniform, prelude::Distribution};

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
        path: Utf8PathBuf
    },
    /// File decryption, provide the generated password
    Decrypt {
        /// Required password to decrypt a file
        #[arg(short, long)]
        pass: String,

        /// The path to the file
        path: Utf8PathBuf
    },
    /// Use SHA512 to generate a hash for the unencrypted file
    Hash {
        /// The path to the file for SHA512 hash
        path: Utf8PathBuf
    }
}

impl CryptoRS {
    fn generate_password() -> Result<String> {
        // Algorithm to generate random password phrase

        let mut rng = rand::thread_rng();

        let between = Uniform::from(0..2048);

        let password: String = (0..PASSWORD_LEN)
            .map(|_| WORDLIST.lines().nth(between.sample(&mut rng)).expect("index in range"))
            .collect::<Vec<_>>()
            .join("-");

        Ok(password)
    }
    fn encrypt_file(file: &Utf8PathBuf, pass: String) -> Result<()> {

        let encrypted = {
            let encryptor = age::Encryptor::with_user_passphrase(SecretString::from_str(pass.as_str()).unwrap());

            let f = File::open(file.as_path())?;

            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer)?;

            println!("\n{}\n", "Encrypting...".yellow());

            let mut encrypted = vec![];
            let mut writer = encryptor.wrap_output(&mut encrypted)?;
            writer.write_all(&buffer)?;
            writer.finish()?;

            encrypted
        };

        let dir = dirs::download_dir().expect("Couldn't get downloads dir!");
        let dest = format!(
            "{}/{}.age",
            dir.display(),
            file.file_name().unwrap()
        );

        let mut writer = File::create(dest)?;

        writer.write_all(encrypted.as_slice())?;

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
    
        let f = File::open(file.as_path())?;
    
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
    
        let decrypted = {
            let decryptor = match age::Decryptor::new_buffered(&buffer[..])? {
                age::Decryptor::Passphrase(d) => d,
                _ => return Ok(()),
            };
    
            let mut decrypted = vec![];

            println!("{}", "\nDecrypting...".yellow());
    
            if let Ok(mut reader) = decryptor.decrypt(&Secret::new(pass), None) {
                reader.read_to_end(&mut decrypted)?;
            } else {
                println!("{}", "\nYour password is incorrect!\n".red());
                return Ok(());
            };
    
            decrypted
        };
    
        let dir = dirs::download_dir().expect("Couldn't get downloads dir!");
        let dest = format!(
            "{}/{}",
            dir.display(),
            file.file_name()
                .unwrap()
                .strip_suffix(".age")
                .unwrap()
        );
    
        let mut writer = File::create(dest)?;
    
        writer.write_all(&decrypted)?;
    
        println!("{}", "\nFile successfully decrypted!\n".green());
    
        Ok(())
    }
    fn gen_file_hash(file: &Utf8PathBuf) -> Result<()> {
        let f = File::open(file.as_path())?;
    
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let slice = buffer.as_slice();

        let hash_bytes = hmac_sha512::Hash::hash(slice);

        let hex_chars: Vec<String> = hash_bytes.iter().map(|byte| format!("{:02x}", byte)).collect();
        let hash_str = hex_chars.join("");

        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["SHA512 Hash"])
            .add_row(vec![hash_str]);

        println!("{}", table.green());
        
        Ok(())
    }
    pub fn exec(self) -> Result<()> {
        match self.command {
            Command::Encrypt { path } => {

                if !path.is_file() {
                    println!("{}", "\nYou cannot use two commands at once and the path must lead to a file\n".red());
                    return Ok(());
                }

                let generated_pass = Self::generate_password()?;

                Self::encrypt_file(&path, generated_pass)?;
            },
            Command::Decrypt { pass, path } => {

                if !path.is_file() {
                    println!("{}", "\nYou cannot use two commands at once and the path must lead to a '.age' file\n".red());
                    return Ok(());
                }

                let result = Self::decrypt_file(&path, pass);

                match result {
                    Ok(()) => {
                        return Ok(());
                    },
                    Err(err) => {
                        if err.downcast::<DecryptError>().is_ok() {
                            println!("{}", "\nAn error occured during decryption, is the file a valid '.age' file?\n".red());
                            return Ok(());
                        }
                    }
                }
            },
            Command::Hash { path } => {
                if !path.is_file() {
                    println!("{}", "\nYou cannot use two commands at once and the path must lead to a file\n".red());
                    return Ok(());
                }

                Self::gen_file_hash(&path)?;
            },
        }

        Ok(())

    }
}