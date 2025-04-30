# A library to generate toml text with clear options and comments

## Example 1: toml text from definition (all examples see tests)
rust code:
```rust
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter};
use toml_input::TomlInput;

/// comment `Test`
#[derive(Debug, TomlInput, Serialize, Deserialize, Default)]
struct Test {
    /// comment `a`
    a: i32,
    /// comment `b`
    b: TestEnum,
}
/// comment `TestEnum`
#[allow(dead_code)]
#[derive(Debug, EnumIter, AsRefStr, TomlInput, Serialize, Deserialize)]
#[derive(Default)]
enum TestEnum {
    /// comment `A`
    A,
    /// comment `B`
    #[default]
    B,
}

let text = Test::schema_to_string().unwrap();
```
toml text:
```toml
# comment `Test`

# comment `a`
a = 0
# comment `A`
#!b = "A"
# comment `B`
b = "B"
```

## Example 2: toml text from value
rust code:
```rust
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, Default, PartialEq)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        #[toml_input(enum_style = "fold")]
        b: TestEnum,
    }
    /// comment `TestEnum`
    #[derive(Debug, Clone, EnumIter, AsRefStr, TomlInput, Serialize, Deserialize, PartialEq)]
    #[allow(dead_code)]
    #[derive(Default)]
    enum TestEnum {
        /// comment `A`
        A,
        /// comment `B`
        #[default]
        B,
    }
    
    let test = Test {
        a: 0,
        b: TestEnum::B,
    };
    let text = test.into_string().unwrap();
```
toml text:
```toml
# comment `Test`

# comment `a`
a = 0
# b = "A" | "B"
# comment `B`
b = "B"
```

## Example 3: enum tuple
rust code:
```rust
    /// comment `Test`
    #[derive(Debug, Clone, TomlInput, Serialize, Deserialize, PartialEq, Default)]
    struct Test {
        /// comment `a`
        a: i32,
        /// comment `b`
        b: TestEnum,
    }
    /// comment `TestEnum`
    #[derive(Debug, Clone, EnumIter, AsRefStr, TomlInput, Serialize, Deserialize, PartialEq)]
    #[allow(dead_code)]
    enum TestEnum {
        /// comment `A`
        A,
        /// comment `B`
        B(String),
    }
    impl Default for TestEnum {
        fn default() -> Self {
            TestEnum::B(String::new())
        }
    }
    let test = Test {
        a: 0,
        b: TestEnum::B("test B".to_string()),
    };
    let text = test.into_string().unwrap();
```
toml text:
```toml
# comment `Test`

# comment `a`
a = 0
# comment `A`
#!b = "A"
# comment `B`
b = { B = "test B" }
```