use std::{fs::File, io::{BufReader, Read, Write}};
use camino::Utf8PathBuf;

use age::{secrecy::Secret, DecryptError};
use clap::{Parser, Subcommand};
use anyhow::Result;
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
}

impl CryptoRS {
    fn generate_password() -> Result<String> {

        // Algorithm to generate random password phrase

        let mut rng = rand::thread_rng();

        let between = Uniform::from(0..2048);

        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                WORDLIST
                    .lines()
                    .nth(between.sample(&mut rng))
                    .expect("index in range")
            })
            .fold(String::new(), |acc, p| {
                if acc.is_empty() {
                    acc + p
                } else {
                    acc + "-" + p
                }
            });

        println!(
            "{}{}{}",
            "Your generated password is: ".yellow(),
            &password.yellow(),
            "\n"
        );

        Ok(password)
    }
    fn encrypt_file(file: &Utf8PathBuf, pass: String) -> Result<()> {

        let encrypted = {
            let encryptor = age::Encryptor::with_user_passphrase(Secret::new(pass));

            let f = File::open(file.as_path())?;

            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer)?;

            println!("{}", "\nEncrypting...".yellow());

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

        let mut writer = File::create(&dest)?;

        writer.write_all(encrypted.as_slice())?;

        println!("{}", "\nFile successfully encrypted!\n".green());

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
                _ => unreachable!(),
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
    
        let mut writer = File::create(&dest)?;
    
        writer.write_all(&decrypted)?;
    
        println!("{}", "\nFile successfully decrypted!\n".green());
    
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
                        if let Ok(_) = err.downcast::<DecryptError>() {
                            println!("{}", "\nAn error occured during decryption, is the file a valid '.age' file?\n".red());
                            return Ok(());
                        }
                    }
                }
            }
        }

        Ok(())

    }
}