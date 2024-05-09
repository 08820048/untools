> 中文文档见[README-CN.md](https://github.com/08820048/uutools/blob/master/REAEME-CN.md)
# uutools

A Rust utility crate for converting variable names from camelCase to snake_case.

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
uutools = "1.0.0"
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

```rust
use ctsc_utils::batch_convert;
fn main() {
    // Specify the input file, output file, and naming convention
    batch_convert("input.txt", "output.txt", true);
}
```

In the example above:
- "input.txt" is the path to the input file containing variable names to be converted.
- "output.txt" is the path to the output file where the converted variable names will be written.
- true indicates that the variable names will be converted to SCREAMING_SNAKE_CASE. Set it to false for camelCase conversion.
------------------
## Futures

> Here are the features and improvements we plan to add to the tool in the future. If you have any suggestions or ideas, feel free to share!

1. **Support for Multiple Naming Conventions:**
   - Add support for other naming conventions such as SCREAMING_SNAKE_CASE.

2. **Batch Conversion Feature:** [x]
   - Allow users to convert multiple variable names at once.

3. **Interactive Mode:**
   - Create an interactive command-line interface for a more intuitive user experience.

4. **File Processing Feature:** [x]
   - Support batch conversion of variable names in files.

5. **Custom Rules:**
   - Enable users to define custom conversion rules.

6. **Integration with Editor Plugins:**
   - Develop editor plugins to allow users to use the conversion tool directly in their editors.

7. **GUI Interface:**
   - Develop a graphical user interface for a more user-friendly experience.

8. **History Tracking Feature:**
   - Keep a record of users' conversion history for easy reference and re-operation.

9. **Export Functionality:**
   - Allow users to export conversion results to files or the clipboard.

10. **Error Handling and Logging:**
    - Implement robust error handling mechanisms and logging capabilities.

If you have any ideas or suggestions regarding the above features, feel free to raise them in the Issues section or directly submit a Pull Request.

----

## License

This project is licensed under the MIT License - see the [LICENSE](https://opensource.org/license/MIT) file for details.