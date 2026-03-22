//! A crate for simple debug output, with optional coloured output

#[cfg(feature = "color")]
mod dbgout_colorized {
    use crate::DebugInfo;
    use colored::{Color, Colorize};
    use std::sync::{LazyLock, Mutex};

    static DEBUG_COLOR: LazyLock<Mutex<Color>> = LazyLock::new(|| Mutex::new(Color::Yellow));

    fn get_debug_color() -> Color {
        DEBUG_COLOR.lock().unwrap().clone()
    }

    pub(crate) fn debug_color_print(info: DebugInfo, text: std::fmt::Arguments) {
        let formatted_text = format!(
            "[debug {} @ {}:{} ({})] {}",
            info.file, info.line, info.col, info.mod_path, text
        );

        eprintln!("{}", formatted_text.color(get_debug_color()));
    }

    /// Sets the colour to use if the "color" feature is enabled
    /// # Arguments
    /// * `color` - The colour to use for debug printing
    pub fn set_debug_color(color: Color) {
        *DEBUG_COLOR.lock().unwrap() = color;
    }
}

#[cfg(feature = "color")]
pub use colored::Color;
#[cfg(feature = "color")]
pub use dbgout_colorized::set_debug_color;

/// A struct containing info for debugging output
#[derive(Clone, Copy, Debug)]
pub struct DebugInfo {
    /// The line which the debug happened
    pub line: u32,
    /// The column of the line which the debug happened
    pub col: u32,
    /// The file which the debug happened in
    pub file: &'static str,
    /// The module the debug happened in
    pub mod_path: &'static str,
    /// If we should print a debug message or not
    /// by default this is controlled by the `--debug` program argument
    pub debug_mode: bool,
}

/// A function which tells us if the `--debug` program argument was passed
pub fn has_debug_flag() -> bool {
    std::env::args().any(|a| a == "--debug")
}

/// A macro which retrieves gets a `DebugInfo` struct for the current location
///
/// # Usage
/// Force enable/disable the `debug_mode` struct member
/// * `get_dbginfo!(true)`
/// * `get_dbginfo!(false)`
/// Set the `debug_mode` member based on if the `--debug` program argument was passed
/// * `get_dbfinfo!()
///
/// # Returns
/// A `DebugInfo` struct
#[macro_export]
macro_rules! get_dbginfo {
    ($debug_mode:literal) => {
        $crate::DebugInfo {
            line: line!(),
            col: column!(),
            file: file!(),
            mod_path: module_path!(),
            debug_mode: $debug_mode,
        }
    };

    () => {
        $crate::DebugInfo {
            line: line!(),
            col: column!(),
            file: file!(),
            mod_path: module_path!(),
            debug_mode: has_debug_flag(),
        }
    };
}

#[doc(hidden)]
pub fn debug_impl(info: DebugInfo, text: std::fmt::Arguments) {
    if info.debug_mode {
        #[cfg(not(feature = "color"))]
        eprintln!(
            "[debug {} @ {}:{} ({})] {}",
            info.file, info.line, info.col, info.mod_path, text
        );
        #[cfg(feature = "color")]
        crate::dbgout_colorized::debug_color_print(info, text);
    }
}

/// A macro to print debug info
/// It will print info in the format
/// `[debug file @ line:column (module_name)] debug_text`
///
/// # Arguments
/// * `$debug_mode` (optional) - Force enable/disable debug printing
/// * `$($arg)*` - The debug text to print, uses the same format as `format!()`
///
/// # Examples
///
/// Force enable debug output
/// ```rust
/// debug!(true, "read data from file \"{}\": {:#?}", file_name, data)
/// ```
/// Print some debug output based on if the `--debug` program argument was passed
/// ```rust
/// debug!("Test Print: {}", 123);
/// ```
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::debug_impl($crate::get_dbginfo!(), format_args!($($arg)*))
    };
    ($debug_mode:literal, $($arg:tt)*) => {
        $crate::debug_impl($crate::get_dbginfo!($debug_mode), format_args!($($arg)*))
    };
}
