use std::{
    fs::{File, self},
    io::{BufReader, Read, Write, self}, path::PathBuf,
};

use age::{secrecy::Secret, DecryptError};
use anyhow::Result;
use camino::Utf8PathBuf;
use flate2::{write::GzEncoder, Compression};
use fltk::{
    app,
    button::*,
    dialog, frame,
    group::{self, Group, Pack, Tabs},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt, WindowExt, InputExt},
    text,
    window::Window, input,
};
use fltk_theme::{widget_themes, ThemeType, WidgetScheme, WidgetTheme};
use rand::{distributions::Uniform, prelude::Distribution};

use crate::{dialogs::MyDialog, errors::EncryptionError};

const WORDLIST: &str = include_str!("./assets/wordlist.txt");
const PASSWORD_LEN: usize = 10;

pub struct CryptoRS;

impl CryptoRS {
    pub fn new() -> Self {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);

        let widget_theme = WidgetTheme::new(ThemeType::Dark);
        widget_theme.apply();

        let widget_scheme = WidgetScheme::new(fltk_theme::SchemeType::SvgBased);
        widget_scheme.apply();

        let mut window = Window::default()
            .with_size(700, 450)
            .with_label("crypto_rs")
            .center_screen();

        Self::draw_gallery();

        window.make_resizable(true);
        window.end();
        window.show();

        app.run().unwrap();

        Self
    }

    fn draw_gallery() {
        // vv Draw the interface vv

        let mut tab = Tabs::new(10, 10, 700 - 20, 450 - 20, "");
        tab.set_frame(widget_themes::OS_TABS_BOX);

        let grp1 = Group::new(10, 35, 700 - 20, 450 - 45, "Encrypt\t\t");

        let mut pack = Pack::new(275, 200, 150, 450 - 45, None);
        pack.set_spacing(10);
        let mut btn = Button::default()
            .with_size(80, 30)
            .with_label("Select file");

        btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);

        btn.set_callback(|_| Self::handle_btn_callback());

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
        input.set_frame(widget_themes::OS_INPUT_THIN_DOWN_BOX);
        flex.end();
        let mut picker = Button::default()
            .with_size(80, 30)
            .with_label("Select file");

        picker.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);

        picker.set_callback(move |_| {
            let pass = input.value();
            Self::handle_picker_callback(pass);
            input.set_value("");
        });

        pack.end();
        grp2.end();

        let grp3 = Group::new(10, 35, 700 - 20, 450, "Hash\t\t");
        let mut pack = Pack::new(275, 200, 150, 450 - 45, None);
        pack.set_spacing(10);
        let mut hash_file = Button::default()
            .with_size(80, 30)
            .with_label("Select file");

        hash_file.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);

        hash_file.set_callback(|_| Self::handle_hash_file_callback());

        pack.end();
        grp3.end();

        let grp4 = Group::new(10, 35, 700 - 20, 450, "FAQ\t\t");

        let mut buf = text::TextBuffer::default();
        let mut txt = text::TextDisplay::default()
            .with_size(500, 275)
            .center_of_parent();
        txt.set_buffer(buf.clone());
        buf.append("Q: How do I encrypt a file?");
        buf.append("\nA: Use the file selector in the 'Encrypt' tab to select a file, once it is selected an encrypted file is written to /Downloads\n");
        buf.append("\nQ: How do I decrypt a file?");
        buf.append("\nA: Go to the 'Decrypt' tab and enter the generated password that was shown to you after encryption. Select the '.age' file that needs decrypted.\n");
        buf.append("\nQ: How can I use the file hash feature?");
        buf.append("\nA: Go to the 'Hash' tab and pick the file you'd like to get a SHA-512 hash of before encryption.  You can later decrypt that file and get the hash again to compare the two.");

        let mut text = frame::Frame::default()
            .with_size(200, 20)
            .above_of(&txt, 30)
            .with_label("crypto_rs\nCM-IV\n<chuck[at]civdev[dot]xyz>");

        text.set_pos(250, 70);
        text.set_label_size(12);

        grp4.end();
        tab.end();

        // ^^ Draw the interface ^^
    }

    fn handle_btn_callback() {
        if let Some(files) = Self::get_files() {

            match Self::compress_files(files) {
                Ok(file_buf) => {
                    let pass = Self::gen_password();
                    if let Err(error) = Self::encrypt_file(file_buf, pass.as_str()) {
                        dialog::message_title("Error!");
                        dialog::alert_default(&error.to_string());

                        return;
                    };
                    MyDialog::new(&pass, "Success!", "Save this password:");
                },
                Err(error) => {
                    // Check for custom error enum here, not using age::EncryptError
                    if let Some(custom_error) = error.downcast_ref::<EncryptionError>() {
                        dialog::message_title("Error!");
                        dialog::alert_default(&custom_error.to_string());
                    }
                }
            };
        }
    }

    fn handle_picker_callback(pass: String) {
        if let Some(file) = Self::get_age_file() {
            if let Err(error) = Self::decrypt_file(&file, pass.as_str()) {
                // Check for age::DecryptError here
                if let DecryptError::DecryptionFailed = error {
                    dialog::message_title("Error!");
                    dialog::alert_default("Invalid password, file failed to decrypt");
                }
            } else {
                dialog::message_title("Success!");
                dialog::message_default("Your file was decrypted!");
            }
        };
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

    fn get_files() -> Option<Vec<PathBuf>> {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseMultiFile);
        dialog.set_title("multiple un-encrypted files");
        dialog.show();
        let binding = dialog.filenames();

        if binding.len() == 0 {
            return None;
        }
        
        Some(binding)
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
            .fold(String::new(), |acc, p| {
                if acc.is_empty() {
                    acc + p
                } else {
                    acc + "-" + p
                }
            });

        password
    }

    fn compress_files(files: Vec<PathBuf>) -> Result<Vec<u8>> {

        let mut ar = tar::Builder::new(Vec::with_capacity(files.len()));

        let mut file_name = String::new();

        for path in files {

            if let Some(ext) = path.extension() {
                if ext == "age" {
                    return Err(EncryptionError::AlreadyEncrypted.into());
                }
            }

            let utf8_pathbuf = Utf8PathBuf::from_path_buf(path).unwrap();

            let mut input_file = File::open(utf8_pathbuf.as_path()).unwrap();

            file_name.push_str(utf8_pathbuf.file_name().unwrap());

            ar.append_file(utf8_pathbuf.file_name().unwrap(), &mut input_file).unwrap();
        }

        let tar_data = ar.get_ref().to_owned();

        let mut gz_encoder = GzEncoder::new(Vec::with_capacity(tar_data.len()), Compression::default());
        gz_encoder.write_all(&tar_data).unwrap();
        
        let gz_data = gz_encoder.finish().unwrap();

        Ok(gz_data)
    }

    fn encrypt_file(buffer: Vec<u8>, pass: &str) -> Result<()> {
        let encrypted = {
            let encryptor = age::Encryptor::with_user_passphrase(Secret::new(pass.to_owned()));

            let mut encrypted = vec![];
            let mut writer = encryptor.wrap_output(&mut encrypted)?;
            writer.write_all(&buffer)?;
            writer.finish()?;

            encrypted
        };

        let dir = dirs::download_dir().expect("Couldn't get downloads dir!");

        let dest = format!("{}/data.tar.gz.age", dir.display());

        // Check if the file already exists
        if let Ok(metadata) = fs::metadata(&dest) {
            if metadata.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!("File {} already exists!", dest),
                )
                .into());
            }
        }

        let mut writer = File::create(dest)?;

        writer.write_all(encrypted.as_slice())?;

        Ok(())
    }

    fn decrypt_file(file: &Utf8PathBuf, pass: &str) -> Result<(), DecryptError> {
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
    fn gen_file_hash(file: &Utf8PathBuf) -> Result<String> {
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