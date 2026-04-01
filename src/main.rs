use anyhow::Result;

use crate::utils::util::print_banner;
mod io;
mod utils;

fn main() -> Result<()> {
    // Display the banner
    print_banner();

    Ok(())
}
