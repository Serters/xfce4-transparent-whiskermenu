use regex::Regex;
use std::error::Error;
use std::fs;
use crate::regex_patterns;
use crate::utils::hex_to_rgba;

pub fn update_start_menu(path: &str, hex_code: &str, opacity: f32) -> Result<(), Box<dyn Error>> {
    // Invert opacity
    let opacity = (((1.0 - opacity) * 100.0).round()) / 100.0;

    let mut content = fs::read_to_string(path)?;

    let new_color = hex_to_rgba(hex_code, opacity)?;

    let base_menu_re = Regex::new(regex_patterns::PATTERN_BASE_MENU)?;
    let menu_opacity_re = Regex::new(regex_patterns::PATTERN_MENU_OPACITY)?;

    content = base_menu_re.replace_all(&content, |caps: &regex::Captures| {
        caps[0].replace(&caps[1], &new_color)
    }).to_string();

    content = menu_opacity_re.replace_all(&content, |caps: &regex::Captures| {
        caps[0].replace(&caps[1], &new_color)
    }).to_string();

    fs::write(path, content)?;
    Ok(())
}

pub fn update_search_bar(path: &str, hex_code: &str, opacity: f32) -> Result<(), Box<dyn Error>> {

    let mut content = fs::read_to_string(path)?;

    let new_color = hex_to_rgba(hex_code, opacity)?;

    let search_focus_re = Regex::new(regex_patterns::PATTERN_SEARCH_FOCUS)?;
    let search_unfocused_re = Regex::new(regex_patterns::PATTERN_SEARCH_UNFOCUSED)?;

    content = search_focus_re.replace_all(&content, |caps: &regex::Captures| {
        caps[0].replace(&caps[1], &new_color)
    }).to_string();

    content = search_unfocused_re.replace_all(&content, |caps: &regex::Captures| {
        caps[0].replace(&caps[1], &new_color)
    }).to_string();

    fs::write(path, content)?;
    Ok(())
}