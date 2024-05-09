# uutool

A Rust utility crate for converting variable names from camelCase to snake_case.

## Features

- Convert variable names from camelCase to snake_case format.
- Option to convert the result to uppercase.
- Simple and easy-to-use tool for maintaining consistent variable naming conventions.

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
ctsc_utils = "0.1.0"
```



## Usage

```rust
use ctsc_utils::ctsc;

fn main() {
    let camel_case_name = "myVariableName";
    let snake_case_name = ctsc(camel_case_name, true);
    println!("Converted name: {}", snake_case_name);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](https://chat.ilikexff.cn/LICENSE) file for details.

------------------

