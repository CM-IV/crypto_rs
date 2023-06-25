use std::{fs::File, io::{Write, BufReader, Read}};

use age::secrecy::Secret;
use camino::Utf8PathBuf;
use fltk::{
    app,
    button::*,
    group::{Group, Pack, Tabs, self},
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt, InputExt},
    window::Window, dialog, frame, input, output, enums::{FrameType, Color}
};
use anyhow::Result;
use rand::{distributions::Uniform, prelude::Distribution};

const WORDLIST: &str = include_str!("./assets/wordlist.txt");
const PASSWORD_LEN: usize = 10;

pub struct MyDialog {
    out: output::Output,
}

impl MyDialog {
    pub fn default(val: String) -> Self {
        let mut win = Window::default()
            .with_size(500, 100)
            .with_label("Success!");
        win.set_color(Color::from_rgb(240, 240, 240));
        frame::Frame::default().with_label("Write down your password!").with_pos(140, 20);
        let mut pack = group::Pack::default()
            .with_size(400, 30)
            .center_of_parent()
            .with_type(group::PackType::Horizontal);
        pack.set_spacing(20);
        let mut out = output::Output::default().with_size(300, 0);
        out.set_value(&val);
        out.set_frame(FrameType::FlatBox);
        let mut ok = Button::default().with_size(80, 0).with_label("Ok");
        pack.end();
        win.end();
        win.make_modal(true);
        win.show();
        ok.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });
        while win.shown() {
            app::wait();
        }
        Self { out }
    }
    pub fn hash_dialog(val: String) -> Self {
        let mut win = Window::default()
            .with_size(600, 100)
            .with_label("Success!");
        win.set_color(Color::from_rgb(240, 240, 240));
        frame::Frame::default().with_label("Here's your file hash").with_pos(170, 20);
        let mut pack = group::Pack::default()
            .with_size(400, 30)
            .center_of_parent()
            .with_type(group::PackType::Horizontal);
        pack.set_spacing(20);
        let mut out = output::Output::default().with_size(350, 0);
        out.set_value(&val);
        out.set_frame(FrameType::FlatBox);
        let mut ok = Button::default().with_size(80, 0).with_label("Ok");
        pack.end();
        win.end();
        win.make_modal(true);
        win.show();
        ok.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });
        while win.shown() {
            app::wait();
        }
        Self { out }
    }
    pub fn value(&self) -> String {
        self.out.value()
    }
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
    let dest = format!(
        "{}/{}.age",
        dir.display(),
        file.file_name().unwrap()
    );

    let mut writer = File::create(dest)?;

    match writer.write_all(encrypted.as_slice()) {
        Ok(_) => {
            MyDialog::default(cloned_pass);
        },
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
        file.file_name()
            .unwrap()
            .strip_suffix(".age")
            .unwrap()
    );

    let mut writer = File::create(dest)?;

    match writer.write_all(decrypted.as_slice()) {
        Ok(_) => {
            dialog::message_title("Success!");
            dialog::message_default("Your file was decrypted!");
        },
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

    let hex_chars: Vec<String> = hash_bytes.iter().map(|byte| format!("{:02x}", byte)).collect();
    let hash_str = hex_chars.join("");

    MyDialog::hash_dialog(hash_str);
    
    Ok(())
}
fn draw_gallery() -> Result<()> {
    let tab = Tabs::new(10, 10, 500 - 20, 450 - 20, "");

    let grp1 = Group::new(10, 35, 500 - 20, 450 - 45, "Encrypt\t\t");

    let mut pack = Pack::new(170, 200, 150, 450 - 45, None);
    pack.set_spacing(10);
    let mut btn = Button::default()
        .with_size(80, 30)
        .with_label("Select file");

    btn.set_callback(|_| {
        if let Some(file) = get_file() {
            let pass = gen_password();
            if let Err(_) = encrypt_file(&file, pass) {
                return;
            }
        }
    });

    pack.end();
    grp1.end();

    let grp2 = Group::new(10, 35, 500 - 30, 450 - 25, "Decrypt\t\t");
    let mut pack = Pack::new(120, 150, 250, 450 - 45, None);
    pack.set_spacing(10);
    let flex = group::Flex::default().with_size(150, 100).column().center_of_parent();
    frame::Frame::default().with_label("Enter password");
    let mut input = input::SecretInput::default();
    flex.end();
    let mut picker = Button::default()
        .with_size(80, 30)
        .with_label("Select file");

    picker.set_callback(move |_| {
        if let Some(file) = get_age_file() {
            let pass = input.value();
            input.set_value("");
            if let Err(_) = decrypt_file(&file, pass) {
                return;
            }
        };

    });

    pack.end();
    grp2.end();

    let grp3 = Group::new(10, 35, 500 - 40, 450 - 0, "Hash\t\t");
    let mut pack = Pack::new(170, 200, 150, 450 - 45, None);
    pack.set_spacing(10);
    let mut hash_file = Button::default()
        .with_size(80, 30)
        .with_label("Select file");

    hash_file.set_callback(|_| {
        if let Some(file) = get_file() {
            if let Err(_) = gen_file_hash(&file) {
                return;
            };
        };

    });

    pack.end();
    grp3.end();

    let grp4 = Group::new(10, 35, 500 - 40, 450 - 0, "FAQ\t\t");
    grp4.end();
    tab.end();

    Ok(())
}

fn main() -> Result<()> {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("crypto_rs")
        .center_screen();

    draw_gallery()?;

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run()?;

    Ok(())
}