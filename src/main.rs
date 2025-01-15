mod regex_patterns;
mod utils;
mod color_updates;
mod config;

use std::error::Error;

use crate::color_updates::{update_whiskar_menu, update_search_bar};
use crate::utils::read_color_values;

fn main() -> Result<(), Box<dyn Error>> {
    
    update_whiskar_menu()?;
    update_search_bar()?;

    read_color_values("resources/gtk-dark.css");

    Ok(())
}