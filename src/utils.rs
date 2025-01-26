use regex::Regex;
use std::error::Error;

/// Converts a hexadecimal color code to an RGBA string representation.
///
/// # Arguments
/// - `hex_code`: A string slice representing the hexadecimal color code.
///    Must be in the format `#RRGGBB` or `#RGB`.
/// - `opacity`: A `f32` value between 0.0 and 1.0 representing the alpha (opacity) value.
///
/// # Returns
/// - An `Ok(String)` containing the `rgba(r, g, b, a)` string representation of the color if the conversion is successful.
/// - An `Err` if the input is invalid.
///
/// # Errors
/// - The hex code is not in the correct format.
/// - The opacity value is not between 0.0 and 1.0.
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
            u8::from_str_radix(&hex_code[4..6], 16)?,
        )
    } else if hex_code.len() == 3 {
        (
            u8::from_str_radix(&format!("{}{}", &hex_code[0..1], &hex_code[0..1]), 16)?,
            u8::from_str_radix(&format!("{}{}", &hex_code[1..2], &hex_code[1..2]), 16)?,
            u8::from_str_radix(&format!("{}{}", &hex_code[2..3], &hex_code[2..3]), 16)?,
        )
    } else {
        return Err("Invalid hex code length".into());
    };

    Ok(format!("rgba({}, {}, {}, {})", r, g, b, opacity))
}


/// Converts a hexadecimal color code to a normalized RGBA array.
///
/// # Arguments
/// - `hex_code`: A string slice representing the hexadecimal color code. 
///    Must be in the format `#RRGGBB` or `#RGB`.
/// - `opacity`: A `f32` value between 0.0 and 1.0 representing the alpha (opacity) value.
///
/// # Returns
/// - An `Ok([f32; 4])` containing:
/// - Red, green, blue and alpha components normalized to the range `[0.0, 1.0]`.
/// - An `Err` if the input is invalid.
///
/// # Errors
/// - The hex code is not in the correct format.
/// - The opacity value is not between 0.0 and 1.0.
/// - The hex code cannot be parsed into numeric values.
pub fn hex_to_normalized_rgba(hex_code: &str, opacity: f32) -> Result<[f32; 4], Box<dyn Error>> {
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
            u8::from_str_radix(&hex_code[4..6], 16)?,
        )
    } else if hex_code.len() == 3 {
        (
            u8::from_str_radix(&format!("{}{}", &hex_code[0..1], &hex_code[0..1]), 16)?,
            u8::from_str_radix(&format!("{}{}", &hex_code[1..2], &hex_code[1..2]), 16)?,
            u8::from_str_radix(&format!("{}{}", &hex_code[2..3], &hex_code[2..3]), 16)?,
        )
    } else {
        return Err("Invalid hex code length".into());
    };

    Ok([
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        opacity,
    ])
}
