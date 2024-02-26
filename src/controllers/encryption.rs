use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use age::secrecy::Secret;
use anyhow::Result;
use camino::Utf8PathBuf;
use inquire::required;
use owo_colors::OwoColorize;
use rand::{distributions::Uniform, prelude::Distribution};

const WORD_LIST: &str = include_str!("../assets/wordlist.txt");

const PASSWORD_LEN: usize = 10;

pub fn encrypt_file() -> Result<()> {
    let file: Utf8PathBuf = inquire::Text::new("Enter the path to the file for encryption")
        .with_validator(required!())
        .with_help_message("Enter the path to the file you want encrypted")
        .prompt()?
        .into();

    if !file.is_file() {
        println!(
            "{}",
            "\nThe file either does not exist or this isn't a file\n".red()
        );
        return Ok(());
    }

    let mut password = inquire::Password::new("Enter your password")
        .with_help_message("Leave empty to have one generated for you")
        .prompt()?;

    if password.is_empty() {
        let mut rng = rand::thread_rng();

        let between = Uniform::from(0..7775);

        password = (0..PASSWORD_LEN)
            .map(|_| {
                WORD_LIST
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
            "\n{}{}\n",
            "Your generated password is: ".yellow(),
            &password.yellow()
        );
    }

    println!("{}", "Encrypting...".yellow());

    let encrypted = {
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(password));

        let f = File::open(file.as_path())?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(&buffer)?;
        writer.finish()?;

        encrypted
    };

    let dir = dirs::download_dir().expect("Couldn't get downloads dir!");
    let dest = format!("{}/{}.age", dir.display(), file.file_name().unwrap());

    let mut writer = File::create(&dest)?;

    writer.write_all(encrypted.as_slice())?;

    println!("{}", "\nFile successfully encrypted!\n".green());

    Ok(())
}

pub fn decrypt_file() -> Result<()> {
    let file: Utf8PathBuf = inquire::Text::new("Enter the path to the file for decryption")
        .with_validator(required!())
        .with_help_message("Enter the path to the file you want decrypted")
        .prompt()?
        .into();

    if !file.is_file() {
        println!(
            "{}",
            "\nThe file either does not exist or this isn't a file\n".red()
        );
        return Ok(());
    }

    let password = inquire::Password::new("Enter your password")
        .with_validator(required!())
        .with_help_message("If you forget this, your file is gone FOREVER")
        .prompt()?;

    println!("{}", "Decrypting...".yellow());

    let f = File::open(file.as_path())?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let decrypted = {
        let decryptor = match age::Decryptor::new(&buffer[..])? {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };

        let mut decrypted = vec![];
        if let Ok(mut reader) = decryptor.decrypt(&Secret::new(password), None) {
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
        file.file_name().unwrap().strip_suffix(".age").unwrap()
    );

    let mut writer = File::create(&dest)?;

    writer.write_all(&decrypted)?;

    println!("{}", "\nFile successfully decrypted!\n".green());

    Ok(())
}
