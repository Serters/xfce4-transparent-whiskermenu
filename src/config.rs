use std::error::Error;
use std::fs;
use toml::Value;

pub fn create_default_config() -> Result<(), Box<dyn Error>> {
    let config_content = r##"# default_paths
theme_path = 'usr/share/themes/Mint-L-Dark/gtk-3.0/gtk-dark.css'
whisker_menu_path ='~/.config/xfce4/panel/'
panel_path = '~/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-panel.xml'

# colors
base_color = "#000000"
opacity = 0"##;

    fs::write("./config.toml", config_content)?;
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
