pub mod block;
pub mod schema;
pub mod util;
pub mod error;
pub mod section;
pub mod root;
pub mod comment;

pub use toml_input_derive::TomlInput;
pub use schema::TomlInput;

const TAG: &str = ".";
const ROOT_KEY: &str = "";
const COMMENT: &str = "#";
const BANG_COMMENT: &str = "#!";
