pub mod block;
pub mod comment;
pub mod config;
pub mod error;
pub mod schema;
pub mod section;
mod toml_input;
pub use toml_input::*;
pub mod util;
mod value;
pub use value::*;
mod content;
pub use content::*;

pub use error::Error;
pub use schema::Schema;
pub use toml_input_derive::TomlInput;

const TAG: &str = ".";
const ROOT_KEY: &str = "";
const COMMENT: &str = "#";
const BANG_COMMENT: &str = "#!";

use toml::Value as TomlValue;
