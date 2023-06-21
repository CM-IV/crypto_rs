use clap::Parser;
use anyhow::Result;
use crypto_rs::CryptoRS;

fn main() -> Result<()> {
    let crypto_rs = CryptoRS::parse();

    crypto_rs.exec()?;

    Ok(())
}