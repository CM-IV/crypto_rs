use std::{
    fs::File,
    io::{BufReader, Read},
};

use anyhow::Result;
use camino::Utf8PathBuf;
use constant_time_eq::constant_time_eq;
use inquire::required;
use owo_colors::OwoColorize;

pub fn hash_file() -> Result<()> {
    let file: Utf8PathBuf = inquire::Text::new("Enter the path to the file for hashing")
        .with_validator(required!())
        .with_help_message("Enter the file path")
        .prompt()?
        .into();

    let f = File::open(file.as_path())?;

    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let hash_bytes = hmac_sha512::Hash::hash(buffer.as_slice());

    let hex_chars: Vec<String> = hash_bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();
    let hash_str = hex_chars.join("");

    println!("\nSHA512: {}\n", hash_str.green());

    Ok(())
}

pub fn compare_hashes() -> Result<()> {
    
    let hash1 = inquire::Text::new("Enter the first hash string")
        .with_validator(required!())
        .with_validator(| s: &str | {
            let re = regex::Regex::new(r"^[0-9a-f]{128}$").expect("failed to compile regex.");

            if re.is_match(&s) {
                Ok(inquire::validator::Validation::Valid)
            } else {
                Ok(inquire::validator::Validation::Invalid("A valid SHA-512sum must be used".into()))
            }
        })
        .prompt()?;

    let hash2 = inquire::Text::new("Enter the second hash string")
        .with_validator(required!())
        .with_validator(| s: &str | {
            let re = regex::Regex::new(r"^[0-9a-f]{128}$").expect("failed to compile regex.");

            if re.is_match(&s) {
                Ok(inquire::validator::Validation::Valid)
            } else {
                Ok(inquire::validator::Validation::Invalid("A valid SHA-512sum must be used".into()))
            }
        })
        .prompt()?;

    let dest1 = hash1.as_bytes();

    let dest2 = hash2.as_bytes();

    if constant_time_eq(dest1, dest2) {
        println!("{}", "\nThe hashes match, everything looks good\n".green());
        return Ok(());
    }

    println!("{}", "\nThe hashes do not match!\n".red());
    
    Ok(())
}