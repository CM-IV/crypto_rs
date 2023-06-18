use std::path::PathBuf;

use controllers::encryption::{encrypt_file, decrypt_file};
use owo_colors::OwoColorize;
use anyhow::Result;
use clap::Parser;

pub mod controllers;

#[derive(Parser)]
#[clap(author = "CM-IV <chuck@civdev.xyz>", version, long_about = r#"
File encryption software
By CM-IV <chuck@civdev.xyz>
"#)]
struct Arguments {
    /// Path to the file
    file: PathBuf,
    /// Password to decrypt a file
    #[arg(short, long)]
    pass: Option<String>,
    /// Encrypt a file (a password is generated for you)
    #[arg(group = "flag")]
    #[arg(short, long)]
    encrypt: bool,
    /// Decrypt a file
    #[arg(group = "flag")]
    #[arg(short, long)]
    decrypt: bool
}


fn main() -> Result<()> {

    let args = Arguments::parse();

    let file = args.file;
    let pass = args.pass;

    if !file.is_file() {
        println!("{}", "\nThe path must lead to a file\n".red());
        return Ok(());
    }

    if args.encrypt {
        encrypt_file(&file)?;
    }

    if args.decrypt {
        if let Some(i) = pass {
            decrypt_file(&file, i)?;
        } else {
            println!("{}", "\nA password must be provided to decrypt the file!\n".red());
            return Ok(());
        }
    }

    Ok(())
}
