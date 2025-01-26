use std::error::Error;
use std::fs;
use toml::Value;

use std::env;
use std::path::PathBuf;

pub fn create_default_config() -> Result<(), Box<dyn Error>> {
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/home".to_string());

    let config_content = format!(
        r##"# default_paths
theme_path = '/usr/share/themes/Mint-L-Dark/gtk-3.0/gtk-dark.css'
whisker_menu_path = '{home_dir}/.config/xfce4/panel/'
panel_path = '{home_dir}/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-panel.xml'

# colors
base_color = "#000000"
opacity = 0.0
search_color = "#000000"
search_opacity = 0.0"##
    );

    let config_path = PathBuf::from("./config.toml");
    fs::write(config_path, config_content)?;
    println!("Created default config.toml in the current directory");
    Ok(())
}

/// Loads the configuration from the `config.toml` file.
///
/// # Returns
/// - An `Ok(Value)` containing the parsed TOML configuration if successful.
/// - An `Err` if the file cannot be read or parsed, or if the file is missing.
fn load_config() -> Result<Value, Box<dyn Error>> {
    let config_content = fs::read_to_string("./config.toml")?;
    print!("Config was Loaded");
    let config: Value = toml::from_str(&config_content)?;
    Ok(config)
}

/// Retrieves your gtk-3.0 linux theme path from the configuration.
///
/// # Returns
/// - An `Ok(String)` containing the theme path if found.
/// - An `Err` if the `theme_path` key is missing or not a valid string.
pub fn get_theme_path() -> Result<String, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["theme_path"]
        .as_str()
        .ok_or("Theme path not found in config")?
        .to_string())
}

/// Retrieves the panel configuration file   path from the configuration.
///
/// # Returns
/// - An `Ok(String)` containing the panel path if found.
/// - An `Err` if the `panel_path` key is missing or not a valid string.
pub fn get_panel_path() -> Result<String, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["panel_path"]
        .as_str()
        .ok_or("Panel path not found in config")?
        .to_string())
}


/// Retrieves the whisker menu configuration file path from the configuration.
///
/// # Returns
/// - An `Ok(String)` containing the whisker menu path if found.
/// - An `Err` if the `whisker_menu_path` key is missing or not a valid string.
pub fn get_whisker_menu_path() -> Result<String, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["whisker_menu_path"]
        .as_str()
        .ok_or("Whisker menu path not found in config")?
        .to_string())
}

/// Retrieves the base color from the configuration.
/// This color is used for the whisker menu border and main color.
///
/// # Returns
/// - An `Ok(String)` containing the base color if found.
/// - An `Err` if the `base_color` key is missing or not a valid string
pub fn get_base_color() -> Result<String, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["base_color"]
        .as_str()
        .ok_or("Base color not found in config")?
        .to_string())
}

/// Retrieves the opacity value from the configuration.
/// This color is used for the whisker menu main color.
/// 
/// # Returns
/// - An `Ok(f32)` containing the opacity value if found.
/// - An `Err` if the `opacity` key is missing or not a valid float.
pub fn get_opacity() -> Result<f32, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["opacity"]
        .as_float()
        .ok_or("Opacity not found in config")? as f32)
}

/// Retrieves the search color from the configuration.
/// This color is used for the search bar inside of whisker menu.
///
/// # Returns
/// - An `Ok(String)` containing the search color if found.
/// - An `Err` if the `search_color` key is missing or not a valid string.
pub fn get_search_color() -> Result<String, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["search_color"]
        .as_str()
        .ok_or("Search color not found in config")?
        .to_string())
}

/// Retrieves the search opacity value from the configuration.
/// This opacity is applied to the search bar color.
///
/// # Returns
/// - An `Ok(f32)` containing the search opacity value if found.
/// - An `Err` if the `search_opacity` key is missing or not a valid float.
pub fn get_search_opacity() -> Result<f32, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["search_opacity"]
        .as_float()
        .ok_or("Search opacity not found in config")? as f32)
}
