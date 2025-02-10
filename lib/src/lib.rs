pub mod block;
pub mod schema;
pub mod util;
pub mod error;
pub mod section;
pub mod group;
pub mod comment;

pub use toml_comment_derive::TomlSchema;
pub use schema::TomlSchema;

const TAG: &str = ".";
const ROOT_KEY: &str = "";
const COMMENT: &str = "#";
const BANG_COMMENT: &str = "#!";
