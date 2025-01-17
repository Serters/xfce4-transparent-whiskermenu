use std::error::Error;
use std::fs;
use toml::Value;

use std::env;
use std::path::PathBuf;

pub fn create_default_config() -> Result<(), Box<dyn Error>> {
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/home".to_string());

    let config_content = format!(r##"# default_paths
theme_path = '/usr/share/themes/Mint-L-Dark/gtk-3.0/gtk-dark.css'
whisker_menu_path = '{home_dir}/.config/xfce4/panel/'
panel_path = '{home_dir}/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-panel.xml'

# colors
base_color = "#000000"
opacity = 0.0
search_color = "#000000"
search_opacity = 0.0"##);

    let config_path = PathBuf::from("./config.toml");
    fs::write(config_path, config_content)?;
    println!("Created default config.toml in the current directory");
    Ok(())
}

fn load_config() -> Result<Value, Box<dyn Error>> {
    let config_content = fs::read_to_string("./config.toml")?;
    print!("Config was Loaded");
    let config: Value = toml::from_str(&config_content)?;
    Ok(config)
}

pub fn get_theme_path() -> Result<String, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["theme_path"]
        .as_str()
        .ok_or("Theme path not found in config")?
        .to_string())
}

pub fn get_panel_path() -> Result<String, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["panel_path"]
        .as_str()
        .ok_or("Panel path not found in config")?
        .to_string())
}

pub fn get_whisker_menu_path() -> Result<String, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["whisker_menu_path"]
        .as_str()
        .ok_or("Whisker menu path not found in config")?
        .to_string())
}

pub fn get_base_color() -> Result<String, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["base_color"]
        .as_str()
        .ok_or("Base color not found in config")?
        .to_string())
}

pub fn get_opacity() -> Result<f32, Box<dyn Error>> {
    let config: Value = load_config()?;

    Ok(config["opacity"]
        .as_float()
        .ok_or("Opacity not found in config")? as f32)
}

pub fn get_search_color() -> Result<String, Box<dyn Error>> {
    let config: Value = load_config()?;
    
    Ok(config["search_color"]
        .as_str()
        .ok_or("Search color not found in config")?
        .to_string())
}

pub fn get_search_opacity() -> Result<f32, Box<dyn Error>> {
    let config: Value = load_config()?;
    
    Ok(config["search_opacity"]
        .as_float()
        .ok_or("Search opacity not found in config")? as f32)
}
