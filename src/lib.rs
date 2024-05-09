use std::fs;

use utils::{is_camel_or_pascal_case, starts_with_digit};


mod utils;
/// Converts a camelCase or PascalCase string to snake_case.
///
/// If the input string is empty, returns an error message.
/// If the input string is a digit, returns an error message.
/// If the input string already contains underscores, returns the input string as-is.
/// If the input string is not in camelCase format, returns an error message.
///
/// # Arguments
///
/// * `name` - A string slice that holds the variable name to be converted.
/// * `is_constant` - A boolean flag indicating if the output should be in uppercase.
///
/// # Returns
///
/// A String containing the snake_case version of the input variable name.
///
/// # Panics 
/// When a panic occurs, it may be due to **your variable** not conforming to the rules of **camelCase or PascalCase**, or being an **empty string**.
pub fn ctsc(name: &str, is_constant: bool) -> String {
    if name.is_empty() {
        return String::from("Input string is empty. Please provide a valid variable name.");
    }

    if !starts_with_digit(name) {
        return String::from("Input string is a digit.");
    }

    if name.contains('_') {
        return name.to_string();
    }

    if !is_camel_or_pascal_case(name) {
        return format!("Input '{}' is not in camelCase format. Please provide a valid camelCase variable name.", name);
    }

    let mut result = String::new();

    for (i, c) in name.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }

    if is_constant {
        return result.to_uppercase();
    }

    result
}


/// Batch convert variable names
/// 
/// This function reads variable name data from the specified input file, converts them to the specified naming convention (camelCase or SCREAMING_SNAKE_CASE),
/// and then writes the converted results to the specified output file.
/// 
/// # Arguments
/// - `ifile`: The path to the input file
/// - `ofile`: The path to the output file
/// - `is_constant`: Whether to convert to SCREAMING_SNAKE_CASE, `true` to convert to SCREAMING_SNAKE_CASE, `false` to convert to camelCase
/// 
/// # Example
/// 
/// ```rust
/// untools::batch_convert("input.txt", "output.txt", true);
/// ```
pub fn batch_convert(ifile: &str, ofile: &str, is_constant: bool) {
    let contents = fs::read_to_string(ifile).expect("Unable to read file.");

    let converted_names: Vec<String> = contents.lines().map(|line| ctsc(line.trim(), is_constant)).collect();

    let output_content = converted_names.join("\n");
    fs::write(ofile, output_content).expect("Unable to write file.");

    println!("Batch conversion successful! Results written to {}", ofile);
}