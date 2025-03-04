use regex::Regex;
use std::error::Error;
use std::fs;

use crate::config;
use crate::regex_patterns;
use crate::utils::{hex_to_normalized_rgba, hex_to_rgba};

/// Updates the Whisker Menu color and transparency in the linux theme configuration.
///
/// This function:
/// 1. Reads the current theme configuration
/// 2. Updates base menu colors and opacity using regular expressions
/// 3. Updates all whiskermenu-*.rc files in the Whisker Menu directory
///
/// # Returns
/// - `Ok(())` on successful update of all configurations
/// - `Err(Box<dyn Error>)` if any file operations or regex operations fail
pub fn update_whiskar_menu() -> Result<(), Box<dyn Error>> {
    let theme_path = config::get_theme_path()?;
    let whisker_menu_dir = config::get_whisker_menu_path()?;
    let hex_code = config::get_base_color()?;
    let opacity = config::get_opacity()?;

    // let menu_opacity = opacity;
    // let inverted_opacity = (((1.0 - opacity) * 100.0).round()) / 100.0;

    let mut theme_content = fs::read_to_string(&theme_path)?;
    let new_color = hex_to_rgba(&hex_code, 0.0)?;
    let base_new_color = hex_to_rgba(&hex_code, 0.99)?;

    let base_menu_re = Regex::new(regex_patterns::PATTERN_BASE_MENU)?;
    let menu_opacity_re = Regex::new(regex_patterns::PATTERN_MENU_OPACITY)?;

    theme_content = base_menu_re
        .replace_all(&theme_content, |caps: &regex::Captures| {
            caps[0].replace(&caps[1], &base_new_color)
        })
        .to_string();

    theme_content = menu_opacity_re
        .replace_all(&theme_content, |caps: &regex::Captures| {
            caps[0].replace(&caps[1], &new_color)
        })
        .to_string();

    fs::write(&theme_path, theme_content)?;

    let whisker_menu_pattern = Regex::new(r"whiskermenu-\d+\.rc$")?;
    let menu_opacity_re = Regex::new(regex_patterns::PATTERN_MENU_BASE_OPACITY)?;

    for entry in fs::read_dir(whisker_menu_dir)? {
        let entry = entry?;
        let path = entry.path();
        let new_opacity = (opacity * 100.0) as u32;

        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if whisker_menu_pattern.is_match(file_name) {
                let mut content = fs::read_to_string(&path)?;
                content = menu_opacity_re
                    .replace_all(&content, |caps: &regex::Captures| {
                        format!("{}{}", &caps[1], new_opacity)
                    })
                    .to_string();
                fs::write(&path, content)?;
            }
        }
    }
    Ok(())
}

/// Updates the search bar colors and transparency in the linux theme configuration. 
///
/// Applies the specified search color and opacity to both focused and unfocused
/// search bar states in the theme file.
///
/// # Returns
/// - `Ok(())` on successful update
/// - `Err(Box<dyn Error>)` for file I/O or regex errors
pub fn update_search_bar() -> Result<(), Box<dyn Error>> {
    let path = config::get_theme_path()?;
    let hex_code = config::get_search_color()?;
    let opacity = config::get_search_opacity()?;

    let mut content = fs::read_to_string(&path)?;

    let new_color = hex_to_rgba(&hex_code, opacity)?;

    let search_focus_re = Regex::new(regex_patterns::PATTERN_SEARCH_FOCUS)?;
    let search_unfocused_re = Regex::new(regex_patterns::PATTERN_SEARCH_UNFOCUSED)?;

    content = search_focus_re
        .replace_all(&content, |caps: &regex::Captures| {
            caps[0].replace(&caps[1], &new_color)
        })
        .to_string();

    content = search_unfocused_re
        .replace_all(&content, |caps: &regex::Captures| {
            caps[0].replace(&caps[1], &new_color)
        })
        .to_string();

    fs::write(&path, content)?;
    Ok(())
}

/// Updates the panel background color and transparency.
///
/// Converts the base color and opacity to normalized RGBA values and updates
/// the panel configuration XML file.
///
/// # Returns
/// - `Ok(())` on successful write
/// - `Err(Box<dyn Error>)` for file operations or color conversion errors
pub fn update_panel() -> Result<(), Box<dyn Error>> {
    let panel_path = config::get_panel_path()?;
    let hex_code = config::get_base_color()?;
    let opacity = config::get_opacity()?;

    let mut content = fs::read_to_string(&panel_path)?;

    let rgba_values = hex_to_normalized_rgba(&hex_code, opacity)?;

    let panel_background_re = Regex::new(regex_patterns::PATTERN_PANEL_BACKGROUND_RGBA)?;

    content = panel_background_re
        .replace_all(&content, |_caps: &regex::Captures| {
            format!(
                r#"<property name="background-rgba" type="array">
        <value type="double" value="{}"/>
        <value type="double" value="{}"/>
        <value type="double" value="{}"/>
        <value type="double" value="{}"/>
      </property>"#,
                rgba_values[0], rgba_values[1], rgba_values[2], rgba_values[3]
            )
        })
        .to_string();

    fs::write(&panel_path, content)?;
    Ok(())
}

/// Updates whiskar menu border colors in the linux theme configuration.
///
/// Applies the base color to all borders in the whisker menu.
///
/// # Returns
/// - `Ok(())` on success
/// - `Err(Box<dyn Error>)` for I/O or regex errors
pub fn update_border() -> Result<(), Box<dyn Error>> {
    let theme_path = config::get_theme_path()?;
    let border_color = config::get_base_color()?;

    let mut content = fs::read_to_string(&theme_path)?;

    let border_re = Regex::new(regex_patterns::PATTERN_BORDER_COLOR)?;

    content = border_re
        .replace_all(&content, |caps: &regex::Captures| {
            caps[0].replace(&caps[1], &border_color)
        })
        .to_string();

    fs::write(&theme_path, content)?;
    Ok(())
}
