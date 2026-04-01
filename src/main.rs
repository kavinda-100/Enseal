use anyhow::Result;
use colored::*;
use indicatif::ProgressBar;
use io::{Action, DECRYPTED_FILE_NAME, ENCRYPTED_FILE_NAME};
use std::time::Duration;

use crate::utils::util::print_banner;
mod crypto;
mod io;
mod utils;

fn main() -> Result<()> {
    // Display the banner
    print_banner();

    loop {
        match io::prompt_action()? {
            Action::Encrypt => handle_encryption()?,
            Action::Decrypt => handle_decryption()?,
            Action::Exit => {
                println!("{}", "Stay secure! Goodbye.".green());
                break;
            }
        }
    }

    Ok(())
}

// Handles the encryption flow: reading .env, getting password, encrypting, and saving .envs.enc
fn handle_encryption() -> Result<()> {
    // 1. Read .env
    let data = io::read_env_file()?;

    // 2. Get password
    let password = io::get_password("Enter a master password to encrypt:")?;

    // 3. Encrypt with a progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_message("Deriving key and encrypting...");
    pb.enable_steady_tick(Duration::from_millis(120));

    let encrypted_data = crypto::encrypt(&data, password)?;

    // 4. Save to .envs.enc
    io::save_file(ENCRYPTED_FILE_NAME, &encrypted_data)?;

    pb.finish_with_message(format!(
        "{} Successfully encrypted to {}",
        "✔".green(),
        ENCRYPTED_FILE_NAME.bold()
    ));
    println!("{}", "Remember: Keep your master password safe. If you lose it, you lose access to these secrets.".yellow());
    Ok(())
}

// Handles the decryption flow: reading .envs.enc, getting password, decrypting, and saving .env
fn handle_decryption() -> Result<()> {
    // 1. Read .envs.enc
    let data = io::read_encrypted_file()?;

    // 2. Get password
    let password = io::get_password("Enter your master password to decrypt:")?;

    // 3. Decrypt with a progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_message("Decrypting and verifying...");
    pb.enable_steady_tick(Duration::from_millis(120));

    match crypto::decrypt(&data, password) {
        Ok(decrypted_data) => {
            // 4. Save to .env
            io::save_file(DECRYPTED_FILE_NAME, &decrypted_data)?;
            pb.finish_with_message(format!(
                "{} Successfully decrypted to {}",
                "✔".green(),
                DECRYPTED_FILE_NAME.bold()
            ));
        }
        Err(e) => {
            pb.abandon_with_message(format!("{} {}", "✘".red(), e));
        }
    }
    Ok(())
}
