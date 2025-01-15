pub const PATTERN_BASE_MENU: &str = r"\.background\s*\{\s*(?:[^{}]*?\s+)?color:\s*[^;]+;\s*background-color:\s*([^;]+);";
pub const PATTERN_MENU_OPACITY: &str = r"\.view,\s*iconview,\s*\.view text,\s*iconview text,\s*textview text\s*\{(?:[^{}]*?)color:\s*[^;]+;\s*background-color:\s*([^;]+);";

// PATTERN_MENU_OPACITY_VALUE
pub const PATTERN_MENU_BASE_OPACITY: &str = r"menu-opacity=(\d+)";

pub const PATTERN_SEARCH_UNFOCUSED: &str = r"entry\s*\{\s*border:\s*[^;]+;\s*padding:\s*[^;]+;\s*caret-color:\s*[^;]+;\s*border-radius:\s*[^;]+;\s*transition:\s*[^;]+;\s*color:\s*[^;]+;\s*border-color:\s*[^;]+;\s*background-color:\s*([^;]+);";
pub const PATTERN_SEARCH_FOCUS: &str = r"entry:focus\s*\{\s*background-clip:\s*[^;]+;\s*color:\s*[^;]+;\s*border-color:\s*[^;]+;\s*background-color:\s*([^;]+);";

// PATTERN_BACKGROUND_RGBA
pub const PATTERN_PANEL_BACKGROUND_RGBA: &str = 
    r#"<property name="background-rgba" type="array">\s*
        <value type="double" value="([0-9.]+)"/>\s*
        <value type="double" value="([0-9.]+)"/>\s*
        <value type="double" value="([0-9.]+)"/>\s*
        <value type="double" value="([0-9.]+)"/>\s*
      </property>"#;