use anyhow::Result;
use controller::CryptoRS;

mod dialogs;
mod controller;
mod errors;

fn main() -> Result<()> {
    CryptoRS::new();

    Ok(())
}
