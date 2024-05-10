
[English Doc](https://github.com/08820048/untools/blob/master/README.md)
# untools
[![Crates.io](https://img.shields.io/crates/d/untools.svg)](https://crates.io/crates/untools)
[![License](https://img.shields.io/github/license/08820048/untools)](https://github.com/08820048/untools/blob/master/LICENSE)
[![rustc 1.77.0](https://img.shields.io/badge/rust-1.77.0-orange.svg)](https://img.shields.io/badge/rust-1.77.0-orange.svg)
[![Documentation](https://docs.rs/console/badge.svg)](https://docs.rs/untools)
[![GitHub stars](https://img.shields.io/github/stars/08820048/untools)](https://github.com/08820048/untools/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/08820048/untools)](https://github.com/08820048/untools/network/members)
[![GitHub issues](https://img.shields.io/github/issues/08820048/untools)](https://github.com/08820048/untools/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/08820048/untools)](https://github.com/08820048/untools/pulls)

一个用于将变量名从驼峰式命名转换为蛇形命名的 Rust 实用工具包。

## 安装

将以下依赖项添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
uutools = "1.0.0" # 这只是个示例，建议使用最新的版本号
```

## 使用

```rust
use untools::camel_to_snake;

fn main() {
    let camel_case_name = "myVariableName";
    let snake_case_name = camel_to_snake(camel_case_name, true);
    println!("转换后的名称: {}", snake_case_name);
}
```

```rust
use untools::batch_convert;
fn main() {
    // Specify the input file, output file, and naming convention
    batch_convert("input.txt", "output.txt", true);
}
```

- "`input.txt`" 是包含待转换变量名的输入文件路径。
- "`output.txt`" 是转换后的变量名将被写入的输出文件路径。
- `true `表示变量名将被转换为 `SCREAMING_SNAKE_CASE`。将其设置为 `false `可进行 `camelCase `转换。

------------------

## 未来计划

> 以下是我们计划在未来为工具添加的功能和改进。如果您有任何建议或想法，请随时分享！

1. **支持多种命名约定:**
   - 添加对其他命名约定（如 SCREAMING_SNAKE_CASE）的支持。
2. **批量转换功能:**
   - :white_check_mark: 允许用户一次转换多个变量名。
3. **交互模式:**
   - 创建交互式命令行界面，提供更直观的用户体验。
4. **文件处理功能:**
   - :white_check_mark: 支持对文件中变量名的批量转换。
5. **自定义规则:**
   - 允许用户定义自定义转换规则。
6. **与编辑器插件集成:**
   - 开发编辑器插件，使用户可以直接在其编辑器中使用转换工具。
7. **GUI界面:**
   - 开发图形用户界面，提供更友好的体验。
8. **支持逆向转换**
   - :white_check_mark: 将下划线转为大小驼峰风格

如果您对以上功能有任何想法或建议，请在`Issues`部分提出，或直接提交`Pull Request`。

----

## 许可证

本项目根据 MIT 许可证授权 - 有关详细信息，请参阅 [LICENSE](https://opensource.org/license/MIT) 文件。

