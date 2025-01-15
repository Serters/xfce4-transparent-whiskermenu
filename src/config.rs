use std::error::Error;
use toml::Value;
use std::fs;

pub fn get_theme_path() -> Result<String, Box<dyn Error>> {
    let config_content = fs::read_to_string("resources/default_config.toml")?;
    let config: Value = toml::from_str(&config_content)?;
    
    Ok(config["theme_path"]
        .as_str()
        .ok_or("Theme path not found in config")?
        .to_string())
}

pub fn get_panel_path() -> Result<String, Box<dyn Error>> {
    let config_content = fs::read_to_string("resources/default_config.toml")?;
    let config: Value = toml::from_str(&config_content)?;
    
    Ok(config["panel_path"]
        .as_str()
        .ok_or("Panel path not found in config")?
        .to_string())
}

pub fn get_whisker_menu_path() -> Result<String, Box<dyn Error>> {
    let config_content = fs::read_to_string("resources/default_config.toml")?;
    let config: Value = toml::from_str(&config_content)?;
    
    Ok(config["whisker_menu_path"]
        .as_str()
        .ok_or("Whisker menu path not found in config")?
        .to_string())
}

pub fn get_base_color() -> Result<String, Box<dyn Error>> {
    let config_content = fs::read_to_string("resources/default_config.toml")?;
    let config: Value = toml::from_str(&config_content)?;
    
    Ok(config["base_color"]
        .as_str()
        .ok_or("Base color not found in config")?
        .to_string())
}

pub fn get_opacity() -> Result<f32, Box<dyn Error>> {
    let config_content = fs::read_to_string("resources/default_config.toml")?;
    let config: Value = toml::from_str(&config_content)?;
    
    Ok(config["opacity"]
        .as_float()
        .ok_or("Opacity not found in config")? as f32)
}