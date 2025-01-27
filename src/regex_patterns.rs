/// Regular expression pattern to match the `background-color` value in the Linux theme `gtk-dark.css` file.
/// This `background-color` affects the main color of the Whisker menu.
///
/// This pattern is used to extract the `background-color` value from a CSS rule like:
/// ```css
/// .background {
///     color: #ffffff;
///     background-color: #000000;
/// }
/// ```
pub const PATTERN_BASE_MENU: &str =
    r"\.background\s*\{\s*(?:[^{}]*?\s+)?color:\s*[^;]+;\s*background-color:\s*([^;]+);";

/// Regular expression pattern to match the `background-color` value in the Linux theme `gtk-dark.css` file.
/// This `background-color` affects the main color of the Whisker menu.
///
/// This pattern is used to extract the `background-color` value from a CSS rule like:
/// ```css
/// .view, iconview,
/// .view text,
/// iconview text,
/// textview text {
///     color: #ffffff;
///     background-color: #000000;
/// }
/// ```
pub const PATTERN_MENU_OPACITY: &str = r"\.view,\s*iconview,\s*\.view text,\s*iconview text,\s*textview text\s*\{(?:[^{}]*?)color:\s*[^;]+;\s*background-color:\s*([^;]+);";

/// Regular expression pattern to match the `menu-opacity` property in Whisker menu configuration files.
///
/// This pattern is used to extract the opacity value from a line like:
/// ```plaintext
/// menu-opacity=0.8
/// ```
pub const PATTERN_MENU_BASE_OPACITY: &str = r"(menu-opacity=)(\d*\.?\d*)";

/// Regular expression pattern to match the `background-color` value in the Linux theme `gtk-dark.css` file.
/// This `background-color` affects the color of the Whisker menu search bar when it is not in focus.
///
/// This pattern is used to extract the `background-color` value from a CSS rule like:
/// ```css
/// entry {
///     border: 1px solid;
///     padding: 5px;
///     caret-color: #ffffff;
///     border-radius: 3px;
///     transition: all 0.3s;
///     color: #ffffff;
///     border-color: #000000;
///     background-color: #333333;
/// }
/// ```
pub const PATTERN_SEARCH_UNFOCUSED: &str = r"entry\s*\{\s*border:\s*[^;]+;\s*padding:\s*[^;]+;\s*caret-color:\s*[^;]+;\s*border-radius:\s*[^;]+;\s*transition:\s*[^;]+;\s*color:\s*[^;]+;\s*border-color:\s*[^;]+;\s*background-color:\s*([^;]+);";

/// Regular expression pattern to match the `background-color` value in the Linux theme `gtk-dark.css` file.
/// This `background-color` affects the color of the Whisker menu search bar when it is in focus.
///
/// This pattern is used to extract the `background-color` value from a CSS rule like:
/// ```css
/// entry:focus {
///     background-clip: padding-box;
///     color: #ffffff;
///     border-color: #000000;
///     background-color: #444444;
/// }
/// ```
pub const PATTERN_SEARCH_FOCUS: &str = r"entry:focus\s*\{\s*background-clip:\s*[^;]+;\s*color:\s*[^;]+;\s*border-color:\s*[^;]+;\s*background-color:\s*([^;]+);";

/// Regular expression pattern to match the `background-rgba` property in the XFCE4 panel XML configuration file.
/// This affects the color and transparency of the XFCE4 panel.
///
/// This pattern is used to extract RGBA values from an XML block like:
/// ```xml
/// <property name="background-rgba" type="array">
///     <value type="double" value="0.0"/>
///     <value type="double" value="0.0"/>
///     <value type="double" value="0.0"/>
///     <value type="double" value="0.8"/>
/// </property>
/// ```
pub const PATTERN_PANEL_BACKGROUND_RGBA: &str = r#"<property name="background-rgba" type="array">\s*
        <value type="double" value="([^"]+)"/>\s*
        <value type="double" value="([^"]+)"/>\s*
        <value type="double" value="([^"]+)"/>\s*
        <value type="double" value="([^"]+)"/>\s*
      </property>"#;

/// Regular expression pattern to match the `border-color` value in the Linux theme `gtk-dark.css` file.
/// This affects the color of all borders in the Whisker menu.
///
/// This pattern is used to extract the `border-color` value from a CSS rule like:
/// ```css
/// entry {
///     border: 1px solid;
///     padding: 5px;
///     caret-color: #ffffff;
///     border-radius: 3px;
///     transition: all 0.3s;
///     color: #ffffff;
///     border-color: #000000;
///     background-color: #333333;
/// }
/// ```
pub const PATTERN_BORDER_COLOR: &str = r"entry\s*\{\s*border:\s*[^;]+;\s*padding:\s*[^;]+;\s*caret-color:\s*[^;]+;\s*border-radius:\s*[^;]+;\s*transition:\s*[^;]+;\s*color:\s*[^;]+;\s*border-color:\s*([^;]+);\s*background-color:\s*[^;]+;\s*\}";
