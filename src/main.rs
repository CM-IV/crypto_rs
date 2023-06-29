use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use age::secrecy::Secret;
use anyhow::Result;
use camino::Utf8PathBuf;
use dialogs::MyDialog;
use fltk::{
    app,
    button::*,
    dialog,
    frame,
    group::{self, Group, Pack, Tabs},
    input,
    prelude::{DisplayExt, GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt},
    text,
    window::Window,
};
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};
use rand::{distributions::Uniform, prelude::Distribution};

mod dialogs;

const WORDLIST: &str = include_str!("./assets/wordlist.txt");
const PASSWORD_LEN: usize = 10;

pub struct CryptoRS {
    app: app::App,
}

impl CryptoRS {
    fn new() -> Self {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);

        let widget_theme = WidgetTheme::new(ThemeType::Classic);
        widget_theme.apply();

        let mut window = Window::default()
            .with_size(700, 450)
            .with_label("crypto_rs")
            .center_screen();

        Self::draw_gallery();

        window.make_resizable(true);
        window.end();
        window.show();

        Self { app }
    }

    fn draw_gallery() {
        // vv Draw the interface vv
    
        let tab = Tabs::new(10, 10, 700 - 20, 450 - 20, "");
    
        let grp1 = Group::new(10, 35, 700 - 20, 450 - 45, "Encrypt\t\t");
    
        let mut pack = Pack::new(275, 200, 150, 450 - 45, None);
        pack.set_spacing(10);
        let mut btn = Button::default()
            .with_size(80, 30)
            .with_label("Select file");
    
        btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    
        btn.set_callback(|_| {
            if let Some(file) = Self::get_file() {
                let pass = Self::gen_password();
                if let Err(_) = Self::encrypt_file(&file, pass) {
                    return;
                }
            }
        });
    
        pack.end();
        grp1.end();
    
        let grp2 = Group::new(10, 35, 700 - 20, 450 - 25, "Decrypt\t\t");
        let mut pack = Pack::new(215, 150, 250, 450 - 45, None);
        pack.set_spacing(10);
        let flex = group::Flex::default()
            .with_size(150, 100)
            .column()
            .center_of_parent();
        frame::Frame::default().with_label("Enter password");
        let mut input = input::SecretInput::default();
        flex.end();
        let mut picker = Button::default()
            .with_size(80, 30)
            .with_label("Select file");
    
        picker.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    
        picker.set_callback(move |_| {
            if let Some(file) = Self::get_age_file() {
                let pass = input.value();
                input.set_value("");
                if let Err(_) = Self::decrypt_file(&file, pass) {
                    return;
                }
            };
        });
    
        pack.end();
        grp2.end();
    
        let grp3 = Group::new(10, 35, 700 - 20, 450 - 0, "Hash\t\t");
        let mut pack = Pack::new(275, 200, 150, 450 - 45, None);
        pack.set_spacing(10);
        let mut hash_file = Button::default()
            .with_size(80, 30)
            .with_label("Select file");
    
        hash_file.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    
        hash_file.set_callback(|_| {
            if let Some(file) = Self::get_file() {
                if let Err(_) = Self::gen_file_hash(&file) {
                    return;
                };
            };
        });
    
        pack.end();
        grp3.end();
    
        let grp4 = Group::new(10, 35, 700 - 20, 450 - 0, "FAQ\t\t");
        let mut buf = text::TextBuffer::default();
        let mut txt = text::TextDisplay::default()
            .with_size(390, 275)
            .center_of_parent();
        txt.set_buffer(buf.clone());
        buf.append("Q: How do I encrypt a file?");
        buf.append("\nA: Use the file selector in the 'Encrypt' tab to select a file, once it is selected an encrypted file is written to /Downloads\n");
        buf.append("\nQ: How do I decrypt a file?");
        buf.append("\nA: Go to the 'Decrypt' tab and enter the generated password that was shown to you after encryption. Select the '.age' file that needs decrypted.\n");
        buf.append("\nQ: How can I use the file hash feature?");
        buf.append("\nA: Go to the 'Hash' tab and pick the file you'd like to get a SHA-512 hash of before encryption.  You can later decrypt that file and get the hash again to compare the two.");
        grp4.end();
        tab.end();
    
        // ^^ Draw the interface ^^
    }

    fn get_file() -> Option<Utf8PathBuf> {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        dialog.show();
        let binding = dialog.filename();
        let file = Utf8PathBuf::from_path_buf(binding).expect("couldn't get Utf8PathBuf of file");
        Some(file)
    }

    fn get_age_file() -> Option<Utf8PathBuf> {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        dialog.set_filter("*.{age}");
        dialog.show();
        let binding = dialog.filename();
        let file = Utf8PathBuf::from_path_buf(binding).expect("couldn't get Utf8PathBuf of file");
        Some(file)
    }

    fn gen_password() -> String {
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
            .fold(String::new(), |acc, p| {
                if acc.is_empty() {
                    acc + p
                } else {
                    acc + "-" + p
                }
            });

        password
    }

    fn encrypt_file(file: &Utf8PathBuf, pass: String) -> Result<()> {
        let cloned_pass = pass.clone();

        let encrypted = {
            let encryptor = age::Encryptor::with_user_passphrase(Secret::new(pass));

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
        let dest = format!("{}/{}.age", dir.display(), file.file_name().unwrap());

        let mut writer = File::create(dest)?;

        match writer.write_all(encrypted.as_slice()) {
            Ok(_) => {
                MyDialog::new(&cloned_pass, "Success!", "Save this password:");
            }
            Err(_) => {
                dialog::alert_default("There was an error!");
                return Ok(());
            }
        };

        Ok(())
    }

    fn decrypt_file(file: &Utf8PathBuf, pass: String) -> Result<()> {
        let f = File::open(file.as_path())?;

        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let decrypted = {
            let decryptor = match age::Decryptor::new_buffered(&buffer[..])? {
                age::Decryptor::Passphrase(d) => d,
                _ => unreachable!(),
            };

            let mut decrypted = vec![];

            if let Ok(mut reader) = decryptor.decrypt(&Secret::new(pass), None) {
                reader.read_to_end(&mut decrypted)?;
            } else {
                return Ok(());
            };

            decrypted
        };

        let dir = dirs::download_dir().expect("Couldn't get downloads dir!");
        let dest = format!(
            "{}/{}",
            dir.display(),
            file.file_name().unwrap().strip_suffix(".age").unwrap()
        );

        let mut writer = File::create(dest)?;

        match writer.write_all(decrypted.as_slice()) {
            Ok(_) => {
                dialog::message_title("Success!");
                dialog::message_default("Your file was decrypted!");
            }
            Err(_) => {
                dialog::message_title("Error!");
                dialog::alert_default("There was an error!");
                return Ok(());
            }
        };

        Ok(())
    }
    fn gen_file_hash(file: &Utf8PathBuf) -> Result<()> {
        let f = File::open(file.as_path())?;

        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let slice = buffer.as_slice();

        let hash_bytes = hmac_sha512::Hash::hash(slice);

        let hex_chars: Vec<String> = hash_bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();
        let hash_str = hex_chars.join("");

        MyDialog::new(&hash_str, "Success!", "Here's your file hash:");

        Ok(())
    }
    pub fn exec(self) -> Result<()> {
        self.app.run()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let app = CryptoRS::new();

    app.exec()?;

    Ok(())
}