use owo_colors::OwoColorize;
use anyhow::Result;
use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet},
    Select,
};

pub mod cli;
pub mod controllers;

fn get_render_cfg() -> RenderConfig<'static> {
    RenderConfig {
        answer: StyleSheet::new()
            .with_attr(Attributes::ITALIC)
            .with_fg(Color::LightCyan),
        help_message: StyleSheet::new().with_fg(Color::LightCyan),
        ..Default::default()
    }
}

// Define an enum for menu items
enum MainMenuItem {
    FileOperations,
    Exit,
}

struct MainMenuBuilder<'a> {
    items: &'a [&'a str],
    help_message: Option<&'a str>,
}

impl<'a> MainMenuBuilder<'a> {
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

    fn build(self) -> Result<MainMenuItem> {
        let choice = Select::new("What would you like to do?", self.items.to_vec())
            .with_help_message(self.help_message.unwrap_or_default())
            .prompt()?;

        let selected_item = match choice {
                "File Operations" => MainMenuItem::FileOperations,
                "Exit" => MainMenuItem::Exit,
                _ => unreachable!(),
            };

        Ok(selected_item)
    }
}

fn main() -> Result<()> {
    inquire::set_global_render_config(get_render_cfg());

    let greet = r#"
     .--------.
    / .------. \
   / /        \ \
   | |        | |
  _| |________| |_
.' |_|        |_| '.
'._____ ____ _____.'
|     .'____'.     |
'.__.'.'    '.'.__.'
'.__  |      |  __.'
|   '.'.____.'.'   |
'.____'.____.'____.'LGB
'.________________.'
                         __                       
  ____________  ______  / /_____        __________
 / ___/ ___/ / / / __ \/ __/ __ \______/ ___/ ___/
/ /__/ /  / /_/ / /_/ / /_/ /_/ /_____/ /  (__  ) 
\___/_/   \__, / .___/\__/\____/     /_/  /____/  
         /____/_/                                 
                                                                       
    "#;

    println!("{}", greet.red());
    println!("File Encryption Software");
    println!("By CM-IV <chuck@civdev.xyz>\n");


    loop {
        match MainMenuBuilder::new(&[
            "File Operations",
            "Exit",
        ])
        .with_help_message("Main menu")
        .build()?
        {
            MainMenuItem::FileOperations => cli::file_menu::file_operations()?,
            MainMenuItem::Exit => {
                println!("{}", "\nGoodbye!\n".purple());
                break;
            },
        }
    }

    Ok(())
}
