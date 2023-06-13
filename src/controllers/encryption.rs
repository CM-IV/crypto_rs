use std::{path::PathBuf, fs::File, io::{BufReader, Read, Write}};

use age::secrecy::Secret;
use anyhow::Result;
use inquire::required;
use owo_colors::OwoColorize;

pub fn encrypt_file() -> Result<()> {
    
    let file: PathBuf = inquire::Text::new("Enter the path to the file for encryption")
        .with_validator(required!())
        .with_help_message("Enter the path to the file you want encrypted")
        .prompt()?.into();

    let password = inquire::Password::new("Enter your password")
        .with_validator(required!())
        .with_help_message("If you forget this, your file is gone FOREVER")
        .prompt()?;

    let f = File::open(file.as_path())?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encrypted = {
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(password));
    
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

    println!("{}", "\nFile successfully encrypted!\n".green());

    Ok(())
}


pub fn decrypt_file() -> Result<()> {
    
    let file: PathBuf = inquire::Text::new("Enter the path to the file for decryption")
        .with_validator(required!())
        .with_help_message("Enter the path to the file you want decrypted")
        .prompt()?.into();

    let password = inquire::Password::new("Enter your password")
        .with_validator(required!())
        .with_help_message("If you forget this, your file is gone FOREVER")
        .prompt()?;

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

    println!("{}", "\nFile successfully decrypted!\n".green());

    Ok(())
}