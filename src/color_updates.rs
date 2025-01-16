use regex::Regex;
use std::error::Error;
use std::fs;

use crate::config;
use crate::regex_patterns;
use crate::utils::{hex_to_normalized_rgba, hex_to_rgba};

pub fn update_whiskar_menu() -> Result<(), Box<dyn Error>> {
    let theme_path = config::get_theme_path()?;
    let whisker_menu_dir = config::get_whisker_menu_path()?;
    let hex_code = config::get_base_color()?;
    let opacity = config::get_opacity()?;

    let menu_opacity = opacity;
    let inverted_opacity = (((1.0 - opacity) * 100.0).round()) / 100.0;

    let mut theme_content = fs::read_to_string(&theme_path)?;
    let new_color = hex_to_rgba(&hex_code, inverted_opacity)?;
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

        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if whisker_menu_pattern.is_match(file_name) {
                let mut content = fs::read_to_string(&path)?;
                content = menu_opacity_re
                    .replace_all(&content, |caps: &regex::Captures| {
                        // Use the captured prefix and append the new value
                        format!("{}{}", &caps[1], menu_opacity)
                    })
                    .to_string();
                fs::write(&path, content)?;
            }
        }
    }
    Ok(())
}

pub fn update_search_bar() -> Result<(), Box<dyn Error>> {
    let path = config::get_theme_path()?;
    let hex_code = config::get_base_color()?;
    let opacity = config::get_opacity()?;

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