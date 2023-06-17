use std::{path::PathBuf, fs::File, io::{BufReader, Read, Write}};

use age::secrecy::Secret;
use anyhow::Result;
use inquire::required;
use owo_colors::OwoColorize;
use rand::Rng;
use spinoff::{Spinner, spinners, Color};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        
const PASSWORD_LEN: usize = 30;

pub fn encrypt_file() -> Result<()> {
    
    let file: PathBuf = inquire::Text::new("Enter the path to the file for encryption")
        .with_validator(required!())
        .with_help_message("Enter the path to the file you want encrypted")
        .prompt()?.into();

    if !file.exists() {
        println!("{}", "\nThe file does not exist\n".red());
        return Ok(());
    }

    let mut password = inquire::Password::new("Enter your password")
        .with_help_message("Leave empty to have one generated for you")
        .prompt()?;

    if password.is_empty() {
        let mut rng = rand::thread_rng();

        password = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        println!("{}{}{}", "Your generated password is: ".yellow(), &password.yellow(), "\n")
    }

    let spinner = Spinner::new(spinners::Dots, "Encrypting...".yellow().to_string(), Color::Yellow);

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
    let dest = format!("{}/{}.age", dir.display(), file.file_name().unwrap().to_str().unwrap());

    let mut writer = File::create(&dest)?;

    writer.write_all(encrypted.as_slice())?;

    spinner.stop();

    println!("{}", "\nFile successfully encrypted!\n".green());

    Ok(())
}


pub fn decrypt_file() -> Result<()> {
    
    let file: PathBuf = inquire::Text::new("Enter the path to the file for decryption")
        .with_validator(required!())
        .with_help_message("Enter the path to the file you want decrypted")
        .prompt()?.into();

    if !file.exists() {
            println!("{}", "\nThe file does not exist\n".red());
            return Ok(());
        }

    let password = inquire::Password::new("Enter your password")
        .with_validator(required!())
        .with_help_message("If you forget this, your file is gone FOREVER")
        .prompt()?;

    let spinner = Spinner::new(spinners::Dots, "Decrypting...".yellow().to_string(), Color::Yellow);

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
    let dest = format!("{}/{}", dir.display(), file.file_name().unwrap().to_str().unwrap().strip_suffix(".age").unwrap());

    let mut writer = File::create(&dest)?;

    writer.write_all(&decrypted)?;

    spinner.stop();

    println!("{}", "\nFile successfully decrypted!\n".green());

    Ok(())
}