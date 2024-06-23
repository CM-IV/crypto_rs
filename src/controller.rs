use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use age::secrecy::Secret;
use camino::Utf8PathBuf;
use fltk::{
    app, dialog,
    enums::Color,
    prelude::{DisplayExt, GroupExt, InputExt, WidgetExt},
    text,
};
use fltk_theme::{widget_schemes::fluent::colors::ACCENT_COLOR, WidgetScheme};
use rand::{distributions::Uniform, prelude::Distribution};

use crate::{
    dialogs::MyDialog,
    errors::{AppError, AppResult},
};

const WORDLIST: &str = include_str!("./assets/wordlist.txt");
const PASSWORD_LEN: usize = 10;

mod fluid {
    fl2rust_macro::include_ui!("src/myui.fl");
}

pub struct CryptoRS;

impl CryptoRS {
    pub fn new() -> Self {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);

        let widget_scheme = WidgetScheme::new(fltk_theme::SchemeType::Fluent);
        widget_scheme.apply();

        Self::draw_gallery();

        app.run().unwrap();

        Self
    }

    fn draw_gallery() {
        // vv Draw the interface vv
        let mut ui = fluid::UserInterface::make_window();

        let mut btn = ui.btn1;

        btn.set_color(Color::from_rgba_tuple(ACCENT_COLOR));
        btn.set_label_color(Color::White);
        btn.set_selection_color(btn.color().darker());

        btn.set_callback(|_| Self::handle_btn_callback());

        let mut input = ui.input1;

        let mut picker = ui.btn2;

        picker.set_color(Color::from_rgba_tuple(ACCENT_COLOR));
        picker.set_label_color(Color::White);
        picker.set_selection_color(picker.color().darker());

        picker.set_callback(move |_| {
            let pass = input.value();
            Self::handle_picker_callback(pass);
            input.set_value("");
        });

        let mut hash_file = ui.btn3;

        hash_file.set_color(Color::from_rgba_tuple(ACCENT_COLOR));
        hash_file.set_label_color(Color::White);
        hash_file.set_selection_color(hash_file.color().darker());

        hash_file.set_callback(|_| Self::handle_hash_file_callback());

        let mut buf = text::TextBuffer::default();
        let mut txt = ui.txt1;

        txt.set_buffer(buf.clone());
        buf.append("Q: How do I encrypt a file?");
        buf.append("\nA: Use the file selector in the 'Encrypt' tab to select a file, once it is selected an encrypted file is written to /Downloads\n");
        buf.append("\nQ: How do I decrypt a file?");
        buf.append("\nA: Go to the 'Decrypt' tab and enter the generated password that was shown to you after encryption. Select the '.age' file that needs decrypted.\n");
        buf.append("\nQ: How can I use the file hash feature?");
        buf.append("\nA: Go to the 'Hash' tab and pick the file you'd like to get a SHA-512 hash of before encryption.  You can later decrypt that file and get the hash again to compare the two.");

        // ^^ Draw the interface ^^
    }

    fn handle_btn_callback() {
        if let Some(file) = Self::get_file() {
            let pass = Self::gen_password();
            match Self::encrypt_file(&file, pass.as_str()) {
                Ok(_) => {
                    MyDialog::new(&pass, "Success!", "Save this password:");
                }
                Err(error) => {
                    if let AppError::AlreadyEncrypted = error {
                        dialog::message_title("Error!");
                        dialog::alert_default("Cannot encrypt an already encrypted file");
                    }
                }
            }
        }
    }

    fn handle_picker_callback(pass: String) {
        if let Some(file) = Self::get_age_file() {
            match Self::decrypt_file(&file, pass.as_str()) {
                Ok(_) => {
                    dialog::message_title("Success!");
                    dialog::message_default("Your file was decrypted!");
                }
                Err(error) => {
                    if let AppError::DecryptError(age::DecryptError::DecryptionFailed) = error {
                        dialog::message_title("Error!");
                        dialog::alert_default("Invalid password, file failed to decrypt");
                    }
                }
            }
        }
    }

    fn handle_hash_file_callback() {
        if let Some(file) = Self::get_file() {
            if let Ok(hash_str) = Self::gen_file_hash(&file) {
                MyDialog::new(&hash_str, "Success!", "Here's your file hash:");
            };
        };
    }

    fn get_file() -> Option<Utf8PathBuf> {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        dialog.set_title("un-encrypted file");
        dialog.show();
        let binding = dialog.filename();
        let file = Utf8PathBuf::from_path_buf(binding).expect("couldn't get Utf8PathBuf of file");
        Some(file)
    }

    fn get_age_file() -> Option<Utf8PathBuf> {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        dialog.set_title("encrypted file");
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
            .collect::<Vec<_>>()
            .join("-");

        password
    }

    fn encrypt_file(file: &Utf8PathBuf, pass: &str) -> AppResult<()> {
        let encrypted = {
            let encryptor = age::Encryptor::with_user_passphrase(Secret::new(pass.to_owned()));

            let path = file.as_path();

            if let Some(ext) = path.extension() {
                if ext == "age" {
                    return Err(AppError::AlreadyEncrypted.into());
                }
            }

            let f = File::open(path)?;

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

        if let Some(out_file) = file.file_name() {
            let dest = format!("{}/{}.age", dir.display(), out_file);

            let mut writer = File::create(dest)?;

            writer.write_all(encrypted.as_slice())?;
        }

        Ok(())
    }

    fn decrypt_file(file: &Utf8PathBuf, pass: &str) -> AppResult<()> {
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

            let mut reader = decryptor.decrypt(&Secret::new(pass.to_owned()), None)?;

            reader.read_to_end(&mut decrypted)?;

            decrypted
        };

        let dir = dirs::download_dir().expect("Couldn't get downloads dir!");

        if let Some(file_name) = file.file_name() {
            if let Some(no_suffix) = file_name.strip_suffix(".age") {
                let dest = format!("{}/{}", dir.display(), no_suffix);

                let mut writer = File::create(dest)?;

                writer.write_all(decrypted.as_slice())?;
            }
        }

        Ok(())
    }
    fn gen_file_hash(file: &Utf8PathBuf) -> AppResult<String> {
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

        Ok(hash_str)
    }
}
