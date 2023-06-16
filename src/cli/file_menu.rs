use anyhow::Result;
use inquire::Select;


use crate::controllers::encryption;

// Define an enum for file items
enum FileMenuItem {
    Encrypt,
    Decrypt,
    GoBack
}


struct FileMenuBuilder<'a> {
    items: &'a [&'a str],
    help_message: Option<&'a str>,
}

impl<'a> FileMenuBuilder<'a> {
    fn new(items: &'a [&'a str]) -> Self {
        Self {
            items,
            help_message: None,
        }
    }

    fn with_help_message(mut self, message: &'a str) -> Self {
        self.help_message = Some(message);
        self
    }

    fn build(self) -> Result<FileMenuItem> {
        let choice = Select::new(
            "Which file operation would you like to perform?",
            self.items.to_vec(),
        )
            .with_help_message(self.help_message.unwrap_or_default())
            .prompt()?;

        let selected_item = match choice {
            "Encrypt a file" => FileMenuItem::Encrypt,
            "Decrypt a file" => FileMenuItem::Decrypt,
            "Go back" => FileMenuItem::GoBack,
            _ => unreachable!(),
        };

        Ok(selected_item)
    }
}


pub fn file_operations() -> Result<()> {
    loop {
        match FileMenuBuilder::new(&[
            "Encrypt a file",
            "Decrypt a file",
            "Go back",
        ])
        .with_help_message("File menu")
        .build()?
        {
            FileMenuItem::Encrypt => encryption::encrypt_file()?,
            FileMenuItem::Decrypt => encryption::decrypt_file()?,
            FileMenuItem::GoBack => {
                break;
            },
        }
    }
    
    Ok(())
}