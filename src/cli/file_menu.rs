use anyhow::{Result, anyhow};
use inquire::Select;


use crate::controllers::encryption;


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

    fn build(self) -> Result<&'a str> {
        let choice = Select::new(
            "Which file operation would you like to perform?",
            self.items.to_vec(),
        )
        .with_help_message(self.help_message.unwrap_or_default())
        .prompt()?;

        Ok(choice)
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
            "Encrypt a file" => encryption::encrypt_file()?,
            "Decrypt a file" => encryption::decrypt_file()?,
            "Go back" => {
                break;
            }
            err => return Err(anyhow!("{}", err)),
        }
    }
    
    Ok(())
}