use regex::Regex;
use std::error::Error;
use std::fs;
use crate::regex_patterns;

pub fn read_color_values(path: &str) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(path)?;

    let base_menu_re = Regex::new(regex_patterns::PATTERN_BASE_MENU)?;
    let menu_opacity_re = Regex::new(regex_patterns::PATTERN_MENU_OPACITY)?;
    let search_focus_re = Regex::new(regex_patterns::PATTERN_SEARCH_FOCUS)?;
    let search_unfocused_re = Regex::new(regex_patterns::PATTERN_SEARCH_UNFOCUSED)?;

    println!("Base Menu Colors:");
    for cap in base_menu_re.captures_iter(&content) {
        println!("{}", &cap[1]);
    }

    println!("\nMenu Opacity Colors:");
    for cap in menu_opacity_re.captures_iter(&content) {
        println!("{}", &cap[1]);
    }

    println!("\nSearch Focus Colors:");
    for cap in search_focus_re.captures_iter(&content) {
        println!("{}", &cap[1]);
    }

    println!("\nSearch Unfocused Colors:");
    for cap in search_unfocused_re.captures_iter(&content) {
        println!("{}", &cap[1]);
    }

    Ok(())
}

pub fn hex_to_rgba(hex_code: &str, opacity: f32) -> Result<String, Box<dyn Error>> {
    let hex_regex = Regex::new(r"^#([0-9A-Fa-f]{3}){1,2}$")?;
    if !hex_regex.is_match(hex_code) {
        return Err("Invalid hex code format. Must be in the format #RRGGBB or #RGB.".into());
    }

    if opacity < 0.0 || opacity > 1.0 {
        return Err("Opacity must be between 0 and 1.".into());
    }

    let hex_code = hex_code.trim_start_matches('#');
    
    let (r, g, b) = if hex_code.len() == 6 {
        (
            u8::from_str_radix(&hex_code[0..2], 16)?,
            u8::from_str_radix(&hex_code[2..4], 16)?,
            u8::from_str_radix(&hex_code[4..6], 16)?
        )
    } else if hex_code.len() == 3 {
        (
            u8::from_str_radix(&format!("{}{}", &hex_code[0..1], &hex_code[0..1]), 16)?,
            u8::from_str_radix(&format!("{}{}", &hex_code[1..2], &hex_code[1..2]), 16)?,
            u8::from_str_radix(&format!("{}{}", &hex_code[2..3], &hex_code[2..3]), 16)?
        )
    } else {
        return Err("Invalid hex code length".into());
    };

    Ok(format!("rgba({}, {}, {}, {})", r, g, b, opacity))
}