mod plain;
pub use plain::{wrap_plain, wrap_plain_no_hyphen, RefLine};

mod colored;
pub use colored::{
    parse_colored_line, parse_colored_lines, wrap_colored, wrap_colored_no_hyphen, ColoredLine,
    ColoredSpan,
};
