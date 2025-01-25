use serde::Serialize;
use toml;
use toml_comment_derive::TomlSchema;

// #[test]
// fn test() {
//     #[derive(TomlSchema, Debug, Default, Serialize)]
//     /// this is comment of struct
//     #[serde()]
//     struct Test {
//         /// this is comment of field
//         a: i32,
//     }
// 
//     use toml_comment::TomlSchema;
//     dbg!(Test::schema());
// }

// #[test]
// fn test1() {
//     /// this is comment of Test1
//     #[derive(TomlSchema, Debug, Serialize)]
//     enum Test1 {
//         A,
//         B,
//         C2,
//     }
//     impl Default for Test1 {
//         fn default() -> Self {
//             Test1::B
//         }
//     }
//     use toml_comment::TomlSchema;
//     dbg!(Test1::schema());
// }
