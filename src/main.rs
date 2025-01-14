mod regex_patterns;
mod utils;
mod color_updates;

use std::error::Error;
use crate::color_updates::{update_start_menu, update_search_bar};
use crate::utils::{test_color_values, get_valid_path, get_valid_hex_color, get_valid_opacity};

fn main() -> Result<(), Box<dyn Error>> {
    
    let path = get_valid_path();

    let start_menu_color = get_valid_hex_color("Enter start menu color (hex format, e.g., #FFFFFF): ");
    let start_menu_opacity = get_valid_opacity("Enter start menu opacity (0-1): ");

    let search_bar_color = get_valid_hex_color("Enter search bar color (hex format, e.g., #FFFFFF): ");
    let search_bar_opacity = get_valid_opacity("Enter search bar opacity (0-1): ");

    update_start_menu(&path, &start_menu_color, start_menu_opacity)?;
    update_search_bar(&path, &search_bar_color, search_bar_opacity)?;

    test_color_values(&path)?;

    Ok(())
}