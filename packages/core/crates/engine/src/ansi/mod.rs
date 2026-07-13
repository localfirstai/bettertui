//! ANSI escape sequence encoding and parsing (CSI, OSC, SGR).

mod encoder;
pub mod parser;

pub use encoder::AnsiEncoder;
pub use parser::{AnsiParser, ParserEvent, ParserState};
mod palette;
pub use palette::{CommandPalette, PaletteCommand, SearchResult};
