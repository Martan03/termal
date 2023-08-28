///! Module with ansi escape codes. Most of them are taken from:
///! https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

// Sequences:

/// The escape character
pub const ESC: char = '\x1b';
/// Control Sequence Introducer: Start of CSI sequence
pub const CSI: &str = "\x1b[";
/// Device Control String: Start of DCS sequence
pub const DCS: &str = "\x1bP";
/// Operating System Command: Start of OCS sequence
pub const OCS: &str = "\x1b]";

/// Creates escape sequence, the first literal is the end of the sequence,
/// the other arguments are the values in the sequence
#[macro_export]
macro_rules! csi {
    ($i:literal, $($a:expr),+) => {
        $crate::seq!("\x1b[", $i, $($a),+)
    };
}

/// Creates the given sequence, this is used internally, you should use
/// the macro [`csi!`]
#[macro_export]
macro_rules! seq {
    ($sq:literal, $i:literal, $f:literal, $($a:literal),*) => {
        concat!($sq, $f $(, ';', $a)*, $i)
    };
    ($sq:literal, $i:literal, $f:expr, $($a:expr),*) => {
        $crate::seq!($sq, $i, $f, $(";{}"; $a),*)
    };
    ($sq:literal, $i:literal, $f:expr, $($l:literal; $e:expr),*) => {
        format!(concat!($sq, "{}", $($l),*, $i), $f $(,$e)*)
    }
}

// General ASCII codes

/// Produces terminal bell
pub const BELL: char = '\x07';
/// Moves the cursor left by one positoin
pub const BACKSPACE: char = '\x08';
/// Horizontal tabulator, moves cursor to the next stop
pub const HTAB: char = '\t';
/// Moves the cursor to the start of the next line
pub const NEWLINE: char = '\n';
/// Vertical tabulator, moves the cursor to the next vertical stop
pub const VTAB: char = '\x0b';
/// Indicates new page, usualy has no use in terminal
pub const FORMFEED: char = '\x0c';
/// Moves cursor to the start of the line
pub const CARRIAGE_RETURN: char = '\r';
/// Does nothing
pub const DELETE: char = '\x7f';

// Cursor controls

// For the macros is true that:
// If you use literals it returns `&str`,
// if you use expressions, it returns [`String`]. You can use the
// `.get_string()` method from the trait [`GetString`] to get [`String`] in
// both cases

/// Moves cursor to the given position.
#[macro_export]
macro_rules! move_to {
    ($x:expr, $y:expr) => {
        $crate::csi!('H', $y, $x)
    };
}

/// Moves cursor up by N positions
#[macro_export]
macro_rules! move_up {
    ($n:expr) => {
        $crate::csi!('A', $n)
    };
}

/// Moves cursor down by N positions
#[macro_export]
macro_rules! move_down {
    ($n:expr) => {
        $crate::csi!('B', $n)
    };
}

/// Moves cursor right by N positions
#[macro_export]
macro_rules! move_right {
    ($n:expr) => {
        $crate::csi!('C', $n)
    };
}

/// Moves cursor left by N positions
#[macro_export]
macro_rules! move_left {
    ($n:expr) => {
        $crate::csi!('D', $n)
    };
}

/// Moves cursor to the start of line N lines down
#[macro_export]
macro_rules! set_down {
    ($n:expr) => {
        $crate::csi!('E', $n)
    };
}

/// Moves cursor to the start of line N lines up
#[macro_export]
macro_rules! set_up {
    ($n:expr) => {
        $crate::csi!('F', $n)
    };
}

/// Moves cursor to the given column
#[macro_export]
macro_rules! column {
    ($n:expr) => {
        $crate::csi!('G', $n)
    };
}

/// Moves cursor one line up, scrolling if needed
pub const UP_SCRL: &str = "\x1bM";
/// Saves the cursor position (this is single save slot, not stack)
pub const CUR_SAVE: &str = "\x1b7";
/// Restores the cursor position to the last saved position (this is single
/// save slot, not stack)
pub const CUR_LOAD: &str = "\x1b8";

// Erase codes

/// Erases from the cursor to the end of the screen
pub const ERASE_TO_END: &str = "\x1b[J";
/// Erases from the start of the screen to the cursor
pub const ERASE_FROM_START: &str = "\x1b[1J";
/// Erases the entire screen
pub const ERASE_SCREEN: &str = "\x1b[2J";
/// Erases the whole screen and the scrollback buffer
pub const ERASE_ALL: &str = "\x1b[3J";
/// Erases from cursor to the end of the line
pub const ERASE_TO_LN_END: &str = "\x1b[K";
/// Erases from the start of the line to the cursor
pub const ERASE_FROM_LN_START: &str = "\x1b[1K";
/// Erases the entire line
pub const ERASE_LINE: &str = "\x1b[2K";

// Text modes

/// Resets all the text modes (colors and styles)
pub const RESET: &str = "\x1b[0m";

/// Set bold text mode (on some terminals may be just brighter color)
pub const BOLD: &str = "\x1b[1m";
/// Set dim/faint text mode
pub const FAINT: &str = "\x1b[2m";
/// Set italic mode
pub const ITALIC: &str = "\x1b[3m";
/// Set underline mode
pub const UNDERLINE: &str = "\x1b[4m";
/// Set blinking mode
pub const BLINKING: &str = "\x1b[5m";
/// Set inverse mode (inverse foreground and background)
pub const INVERSE: &str = "\x1b[7m";
/// Set invisible mode (foreground is same as background)
pub const INVISIBLE: &str = "\x1b[8m";
/// Set striketrough mode
pub const STRIKETROUGH: &str = "\x1b[9m";
/// Set double underline mode
pub const DOUBLE_UNDERLINE: &str = "\x1b[21";

/// Reset [`BOLD`] and [`FAINT`] mode
pub const RESET_BOLD: &str = "\x1b[22m";
/// Reset [`ITALIC`] mode
pub const RESET_ITALIC: &str = "\x1b[23m";
/// Reset [`UNDERLINE`] and [`DOUBLE_UNDERLINE`] mode
pub const RESET_UNDERLINE: &str = "\x1b[24m";
/// Reset [`BLINKING`] mode
pub const RESET_BLINKING: &str = "\x1b[25m";
/// Reset [`INVERSE`] mode
pub const RESET_INVERSE: &str = "\x1b[27m";
/// Reset [`INVISIBLE`] mode
pub const RESET_INVISIBLE: &str = "\x1b[28m";
/// Reset [`STRIKETROUGH`] mode
pub const RESET_STRIKETROUGH: &str = "\x1b[29m";

/// Set the foreground color to black (dark black)
pub const BLACK_FG: &str = "\x1b[30m";
/// Set the foreground color to white (bright white)
pub const WHITE_FG: &str = "\x1b[97m";
/// Set the foreground color to gray (bright black)
pub const GRAY_FG: &str = "\x1b[90m";
/// Set to foreground color to bright gray (dark white)
pub const GRAY_BRIGHT_FG: &str = "\x1b[37m";

/// Set the foreground color to red (bright red)
pub const RED_FG: &str = "\x1b[91m";
/// Set the foreground color to green (bright green)
pub const GREEN_FG: &str = "\x1b[92m";
/// Set the foreground color to yellow (bright yellow)
pub const YELLOW_FG: &str = "\x1b[93m";
/// Set the foreground color to blue (bright blue)
pub const BLUE_FG: &str = "\x1b[94m";
/// Set the foreground color to magenta (bright magenta)
pub const MAGENTA_FG: &str = "\x1b[95m";
/// Set the foreground color to cyan (bright cyan)
pub const CYAN_FG: &str = "\x1b[96m";

/// Set the foreground color to dark red
pub const RED_DARK_FG: &str = "\x1b[31m";
/// Set the foreground color to dark green
pub const GREEN_DARK_FG: &str = "\x1b[32m";
/// Set the foreground color to dark yellow
pub const YELLOW_DARK_FG: &str = "\x1b[33m";
/// Set the foreground color to dark blue
pub const BLUE_DARK_FG: &str = "\x1b[34m";
/// Set the foreground color to dark magenta
pub const MAGENTA_DARK_FG: &str = "\x1b[35m";
/// Set the foreground color to dark cyan
pub const CYAN_DARK_FG: &str = "\x1b[36m";

/// Reset the foreground color
pub const RESET_FG: &str = "\x1b[39m";

/// Set the background color to black (dark black)
pub const BLACK_BG: &str = "\x1b[40m";
/// Set the background color to white (bright white)
pub const WHITE_BG: &str = "\x1b[107m";
/// Set the background color to gray (bright black)
pub const GRAY_BG: &str = "\x1b[100m";
/// Set to background color to bright gray (dark white)
pub const GRAY_BRIGHT_BG: &str = "\x1b[47m";

/// Set the background color to red (bright red)
pub const RED_BG: &str = "\x1b[101m";
/// Set the background color to green (bright green)
pub const GREEN_BG: &str = "\x1b[102m";
/// Set the background color to yellow (bright yellow)
pub const YELLOW_BG: &str = "\x1b[103m";
/// Set the background color to blue (bright blue)
pub const BLUE_BG: &str = "\x1b[104m";
/// Set the background color to magenta (bright magenta)
pub const MAGENTA_BG: &str = "\x1b[105m";
/// Set the background color to cyan (bright cyan)
pub const CYAN_BG: &str = "\x1b[106m";

/// Set the background color to dark red
pub const RED_DARK_BG: &str = "\x1b[41m";
/// Set the background color to dark green
pub const GREEN_DARK_BG: &str = "\x1b[42m";
/// Set the background color to dark yellow
pub const YELLOW_DARK_BG: &str = "\x1b[43m";
/// Set the background color to dark blue
pub const BLUE_DARK_BG: &str = "\x1b[44m";
/// Set the background color to dark magenta
pub const MAGENTA_DARK_BG: &str = "\x1b[45m";
/// Set the background color to dark cyan
pub const CYAN_DARK_BG: &str = "\x1b[46m";

/// Reset the background color
pub const RESET_BG: &str = "\x1b[49m";

/// creates a foreground color, color is value in range 0..256
#[macro_export]
macro_rules! fg256 {
    ($c:expr) => {
        $crate::csi!('m', 38, 5, $c)
    };
}

/// creates a background color, color is value in range 0..256
#[macro_export]
macro_rules! bg256 {
    ($c:expr) => {
        $crate::csi!('m', 48, 5, $c)
    };
}

/// creates a true rgb foreground color. R, G and B must be values in range
/// 0..256
#[macro_export]
macro_rules! fg {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::csi!('m', 38, 2, $r, $g, $b)
    };
}

/// creates a true rgb foreground color. R, G and B must be values in range
/// 0..256
#[macro_export]
macro_rules! bg {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::csi!('m', 48, 2, $r, $g, $b)
    };
}

// Screen modes

/// Enables line wrapping
pub const ENABLE_LINE_WRAP: &str = "\x1b[=7h";
/// Disables line wrapping
pub const DISABLE_LINE_WRAP: &str = "\x1b[=7l";

// Private modes

/// Makes the cursor invisible
pub const HIDE_CURSOR: &str = "\x1b[?25l";
/// Makes the cursor visible
pub const SHOW_CURSOR: &str = "\x1b[?25h";
/// Saves the visible part of the screen buffer
pub const SAVE_SCREEN: &str = "\x1b[?47l";
/// Loads the last saved screen
pub const LOAD_SCREEN: &str = "\x1b[?47h";
/// Enables alternative buffer
pub const ENABLE_ALTERNATIVE_BUFFER: &str = "\x1b[?1049h";
/// Disables the laternative buffer
pub const DISABLE_ALTERNATIVE_BUFFER: &str = "\x1b[?1049l";

// Other

/*#[macro_export]
macro_rules! resize_window {
    ($x:expr, $y:expr) => {
        $crate::csi!('t', 8, $y, $x)
    };
}*/

/// Trait for getting string from &str and String
pub trait GetString {
    /// If [`self`] is `&str` uses `.to_owned()`, if [`self`] is [`String`] returns
    /// [`self`]
    fn get_string(self) -> String;
}

impl GetString for &str {
    fn get_string(self) -> String {
        self.to_owned()
    }
}

impl GetString for String {
    fn get_string(self) -> String {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    fn type_id_of<T: 'static>(_: T) -> TypeId {
        TypeId::of::<T>()
    }

    use super::*;

    #[test]
    fn test_macros() {
        assert_eq!(csi!('a', 1, 2, 3, 4, 5), "\x1b[1;2;3;4;5a");
        assert_eq!(csi!('a', 1 + 0, 2, 3, 4, 5), "\x1b[1;2;3;4;5a");
        assert_eq!(type_id_of(csi!('a', 1, 2, 3, 4, 5)), TypeId::of::<&str>());
        assert_eq!(
            type_id_of(csi!('a', 1 + 0, 2, 3, 4, 5)),
            TypeId::of::<String>()
        );
    }
}
