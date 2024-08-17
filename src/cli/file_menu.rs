use anyhow::Result;
use bon::builder;
use inquire::Select;

use crate::controllers::encryption;
use crate::controllers::hash;

enum FileMenuItem {
    Encrypt,
    Decrypt,
    Hash,
    Compare,
    GoBack,
}

#[builder]
fn build_file_menu<'a>(
    items: &'a [&'a str],
    help_message: Option<&'a str>,
) -> Result<FileMenuItem> {
    let choice = Select::new(
        "Which file operation would you like to perform?",
        items.to_vec(),
    )
    .with_help_message(help_message.unwrap_or_default())
    .prompt()?;

    let selected_item = match choice {
        "Encrypt a file" => FileMenuItem::Encrypt,
        "Decrypt a file" => FileMenuItem::Decrypt,
        "Get file hash" => FileMenuItem::Hash,
        "Compare hashes" => FileMenuItem::Compare,
        "Go back" => FileMenuItem::GoBack,
        _ => unreachable!(),
    };

    Ok(selected_item)
}

pub fn file_operations() -> Result<()> {
    loop {
        match build_file_menu()
            .items(&[
                "Encrypt a file",
                "Decrypt a file",
                "Get file hash",
                "Compare hashes",
                "Go back",
            ])
            .help_message("File menu")
            .call()?
        {
            FileMenuItem::Encrypt => encryption::encrypt_file()?,
            FileMenuItem::Decrypt => encryption::decrypt_file()?,
            FileMenuItem::Hash => hash::hash_file()?,
            FileMenuItem::Compare => hash::compare_hashes()?,
            FileMenuItem::GoBack => {
                break;
            }
        }
    }

    Ok(())
}
