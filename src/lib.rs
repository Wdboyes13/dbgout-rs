//! A crate for simple debug output, with optional coloured output
#![warn(missing_docs)]

use std::io::{self, Write};
use std::sync::{LazyLock, Mutex};

/// A writer which we can send debug logs to
pub type Writer = Box<dyn Write + Send>;

static DEBUG_WRITER: LazyLock<Mutex<Writer>> = LazyLock::new(|| Mutex::new(Box::new(io::stderr())));

/// Set the writer for debug output
///
/// # Arguments
/// * `writer` - the Writer which we should switch to for debugs
pub fn set_debug_writer(writer: Writer) {
    *DEBUG_WRITER.lock().unwrap() = writer;
}

#[cfg(feature = "color")]
mod dbgout_colorized {
    use crate::{DEBUG_WRITER, DebugInfo};
    use colored::{Color, Colorize};
    use std::sync::{LazyLock, Mutex};

    static DEBUG_COLOR: LazyLock<Mutex<Color>> = LazyLock::new(|| Mutex::new(Color::Yellow));

    fn get_debug_color() -> Color {
        *DEBUG_COLOR.lock().unwrap()
    }

    pub(crate) fn debug_color_print(info: DebugInfo, text: std::fmt::Arguments) {
        let formatted_text = format!(
            "[debug {} @ {}:{} ({})] {}",
            info.file, info.line, info.col, info.mod_path, text
        );

        let _ = writeln!(
            DEBUG_WRITER.lock().unwrap(),
            "{}",
            formatted_text.color(get_debug_color())
        );
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

/// A function to tell if we're in debug mode
///
/// # Returns
/// * If the `--debug` program argument is passed => true
/// * If `should_check_build` is `true` and its a Debug build => true
pub fn has_debug_flag(should_check_build: bool) -> bool {
    std::env::args().any(|a| a == "--debug") || (should_check_build && cfg!(debug_assertions))
}

/// A macro which retrieves gets a `DebugInfo` struct for the current location
///
/// # Usage
/// Force enable/disable the `debug_mode` struct member
/// * `get_dbginfo!(true)`
/// * `get_dbginfo!(false)`
///
/// Set the `debug_mode` member based on if the `--debug` program argument was passed
/// * `get_dbginfo!()`
///
/// Set the `debug_mode` member if we are on a debug build or the `--debug` argument was passed
/// * `get_dbginfo!(auto)`
///
/// # Returns
/// A `DebugInfo` struct
#[macro_export]
macro_rules! get_dbginfo {
    () => {
        $crate::DebugInfo {
            line: line!(),
            col: column!(),
            file: file!(),
            mod_path: module_path!(),
            debug_mode: $crate::has_debug_flag(false),
        }
    };
    (auto) => {
        $crate::DebugInfo {
            line: line!(),
            col: column!(),
            file: file!(),
            mod_path: module_path!(),
            debug_mode: $crate::has_debug_flag(true),
        }
    };
    ($debug_mode:literal) => {
        $crate::DebugInfo {
            line: line!(),
            col: column!(),
            file: file!(),
            mod_path: module_path!(),
            debug_mode: $debug_mode,
        }
    };
}

#[doc(hidden)]
pub fn debug_impl(info: DebugInfo, text: std::fmt::Arguments) {
    if info.debug_mode {
        #[cfg(not(feature = "color"))]
        let _ = writeln!(
            DEBUG_WRITER.lock().unwrap(),
            "[debug {} @ {}:{} ({})] {}",
            info.file,
            info.line,
            info.col,
            info.mod_path,
            text
        );
        #[cfg(feature = "color")]
        crate::dbgout_colorized::debug_color_print(info, text);
    }
}

/// A macro to write debug info
///
/// It will print info in the format
/// `[debug file @ line:column (module_name)] debug_text`
/// It will print to the current writer (modified with `set_debug_writer`) by default `std::io::stderr()`
///
/// # Usage
/// Force enable/disable printing
/// * `debug!(true, "debug_text")`
/// * `debug!(false, "debug_text")`
/// * `debug!(true)`
/// * `debug!(false)`
///
/// Print based on if the `--debug` program argument was passed
/// * `debug!("debug_text")`
/// * `debug!()`
///
/// Print if we are on a debug build or the `--debug` argument was passed
/// * `debug!(auto, "debug_text")`
/// * `debug!(auto)`
///
/// # Examples
///
/// Force enable debug output
/// ```rust,ignore
/// use dbgout::debug;
/// debug!(true, "read data from file \"{}\": {:#?}", file_name, data)
/// ```
/// Print some debug output based on if the `--debug` program argument was passed
/// ```rust
/// use dbgout::debug;
/// debug!("Test Print: {}", 123);
/// ```
/// Just print the debug info
/// ```rust
/// use dbgout::debug;
/// debug!();
/// ```
#[macro_export]
macro_rules! debug {
    (auto, $($arg:tt)*) => {
        $crate::debug_impl($crate::get_dbginfo!(auto), format_args!($($arg)*))
    };
    (true, $($arg:tt)*) => {
        $crate::debug_impl($crate::get_dbginfo!(true), format_args!($($arg)*))
    };
    (false, $($arg:tt)*) => {
        $crate::debug_impl($crate::get_dbginfo!(false), format_args!($($arg)*))
    };
    (auto) => {
        $crate::debug_impl($crate::get_dbginfo!(auto), format_args!(""))
    };
    (true) => {
        $crate::debug_impl($crate::get_dbginfo!(true), format_args!(""))
    };
    (false) => {
        $crate::debug_impl($crate::get_dbginfo!(false), format_args!(""))
    };
    () => {
        $crate::debug_impl($crate::get_dbginfo!(), format_args!(""))
    };
    ($($arg:tt)*) => {
        $crate::debug_impl($crate::get_dbginfo!(), format_args!($($arg)*))
    };
}
