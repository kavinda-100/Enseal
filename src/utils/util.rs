use colored::*;

pub fn print_banner() {
    let banner = r#"
    _______  __    __  ________  _______  ________  __      
   /  ____/ /  |  / / /  _____/ /  ____/ /  ____  / /  |     
  /  /__   /   | / / /  /____  /  /__   /  /___/ / /  /      
 /  ___/  /  /| |/ / /____  / /  ___/  /  ____  / /  /____   
/_______//__/ |___/ /______/ /_______//__/   /_/ /_______/   
    "#;

    println!("{}", banner.green().bold());
    println!(
        "{} {}",
        "🛡".green(),
        "The High-Performance Environment Vault".italic().white()
    );
    println!(
        "{}\n",
        "-----------------------------------------------------------".bright_black()
    );
}
