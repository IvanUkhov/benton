//! A parser for OpenType fonts.
//!
//! ## Example
//!
//! ```
//! extern crate opentype;
//!
//! use opentype::File;
//! use opentype::truetype::NamingTable;
//!
//! # fn main() {
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let file = File::open(path).unwrap();
//!
//! match file.font_header {
//!     Some(ref table) => assert_eq!(table.unitsPerEm, 1000),
//!     _ => unreachable!(),
//! }
//! match file.horizontal_header {
//!     Some(ref table) => assert_eq!(table.Ascender, 918),
//!     _ => unreachable!(),
//! }
//! match file.naming_table {
//!     Some(NamingTable::Format0(ref table)) => {
//!         let strings = table.strings().unwrap();
//!         assert_eq!(&strings[1], "Source Serif Pro");
//!         assert_eq!(&strings[9], "Frank Grießhammer");
//!     },
//!     _ => unreachable!(),
//! }
//! # }
//! ```

extern crate postscript as postscript_crate;
extern crate truetype as truetype_crate;

pub use postscript_crate as postscript;
pub use truetype_crate as truetype;

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

mod file;
mod tape;

pub use file::File;
