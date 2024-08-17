use anyhow::Result;
use bon::builder;
use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet},
    Select,
};
use owo_colors::OwoColorize;

mod cli;
mod controllers;

fn get_render_cfg() -> RenderConfig<'static> {
    RenderConfig {
        answer: StyleSheet::new()
            .with_attr(Attributes::ITALIC)
            .with_fg(Color::LightCyan),
        help_message: StyleSheet::new().with_fg(Color::LightCyan),
        ..Default::default()
    }
}

enum MainMenuItem {
    FileOperations,
    Exit,
}

#[builder]
fn build_main_menu<'a>(
    items: &'a [&'a str],
    help_message: Option<&'a str>,
) -> Result<MainMenuItem> {
    let choice = Select::new("What would you like to do?", items.to_vec())
        .with_help_message(help_message.unwrap_or_default())
        .prompt()?;

    let selected_item = match choice {
        "File Operations" => MainMenuItem::FileOperations,
        "Exit" => MainMenuItem::Exit,
        _ => unreachable!(),
    };

    Ok(selected_item)
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
        match build_main_menu()
            .items(&["File Operations", "Exit"])
            .help_message("Main menu")
            .call()?
        {
            MainMenuItem::FileOperations => cli::file_menu::file_operations()?,
            MainMenuItem::Exit => {
                println!("{}", "\nGoodbye!\n".purple());
                break;
            }
        }
    }

    Ok(())
}
