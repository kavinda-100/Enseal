use anyhow::{Context, Result};
use colored::*;
use inquire::{Password, PasswordDisplayMode, Select};
use std::fs;
use std::path::Path;

pub const ENCRYPTED_FILE_NAME: &str = ".envs.enc";
pub const DECRYPTED_FILE_NAME: &str = ".env";
pub const MIN_PASSWORD_LEN: usize = 4;

#[derive(Debug, Clone)]
pub enum Action {
    Encrypt,
    Decrypt,
    Exit,
}

/// Prompts the user to select an action: Encrypt or Decrypt.
pub fn prompt_action() -> Result<Action> {
    let options = vec![
        "Encrypt .env".to_string(),
        "Decrypt .envs.enc".to_string(),
        "Exit".to_string(),
    ];
    let ans = Select::new("What would you like to do?", options).prompt()?;
    // Map the user's selection to the corresponding Action enum variant.
    match ans.as_str() {
        "Encrypt .env" => Ok(Action::Encrypt),
        "Decrypt .envs.enc" => Ok(Action::Decrypt),
        "Exit" => Ok(Action::Exit),
        _ => Err(anyhow::anyhow!("Invalid action selected")),
    }
}

/// Safely prompts for a password using inquire's masked display.
pub fn get_password(prompt: &str) -> Result<String> {
    let password = Password::new(prompt)
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_help_message("This password will be used to derive your encryption key.")
        .prompt()?;

    if password.is_empty() {
        return Err(anyhow::anyhow!("Password cannot be empty"));
    }

    if password.len() < MIN_PASSWORD_LEN {
        return Err(anyhow::anyhow!(
            "Password must be at least {} characters long",
            MIN_PASSWORD_LEN
        ));
    }

    Ok(password)
}

/// Reads the raw .env file.
pub fn read_env_file() -> Result<Vec<u8>> {
    if !Path::new(DECRYPTED_FILE_NAME).exists() {
        return Err(anyhow::anyhow!(
            "{} not found in the current directory.",
            DECRYPTED_FILE_NAME.yellow()
        ));
    }
    fs::read(DECRYPTED_FILE_NAME).context("Failed to read .env file")
}

/// Reads the encrypted .envs.enc file.
pub fn read_encrypted_file() -> Result<Vec<u8>> {
    if !Path::new(ENCRYPTED_FILE_NAME).exists() {
        return Err(anyhow::anyhow!(
            "{} not found. Nothing to decrypt.",
            ENCRYPTED_FILE_NAME.yellow()
        ));
    }
    fs::read(ENCRYPTED_FILE_NAME).context("Failed to read encrypted file")
}

/// Writes the final processed bytes to disk.
pub fn save_file(file_name: &str, data: &[u8]) -> Result<()> {
    fs::write(file_name, data).with_context(|| format!("Failed to write to {}", file_name))
}
