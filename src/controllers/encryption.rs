use std::{path::PathBuf, fs::File, io::{BufReader, Read, Write}};

use age::secrecy::Secret;
use anyhow::Result;
use owo_colors::OwoColorize;
use rand::{distributions::Uniform, prelude::Distribution};
use spinoff::{Spinner, spinners, Color};

const WORDLIST: &str = include_str!("../assets/wordlist.txt");
        
const PASSWORD_LEN: usize = 10;

pub fn encrypt_file(file: &PathBuf) -> Result<()> {

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
        .fold(String::new(), | acc, p | {
            if acc.is_empty() {
                acc + p
            } else {
                acc + "-" + p
            }
        });

    println!("{}{}{}", "Your generated password is: ".yellow(), &password.yellow(), "\n");

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


pub fn decrypt_file(file: &PathBuf, pass: String) -> Result<()> {

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
        if let Ok(mut reader) = decryptor.decrypt(&Secret::new(pass), None) {
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