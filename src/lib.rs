use std::fs;

use console::style;
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
/// * `is_constant` - A boolean flag indicating if the output should be in all caps.
///
/// # Returns
///
/// A String containing the snake_case version of the input variable name.
/// 
/// # Usage
///
/// This converts the the variable "testVariable" from camelcase to snakecase in all caps.
/// 
/// ``` 
/// assert_eq!(camel_to_snake("testVariable", true), "TEST_VARIABLE")
/// ```
///
pub fn camel_to_snake(name: &str, is_constant: bool) -> String {
    if name.is_empty() {
        println!(
            "{}",
            style("Input string is empty. Please provide a valid variable name.").color256(208)
        );
        return String::new();
    }

    if !starts_with_digit(name) {
        println!("{}", style("Input string is a digit.").color256(208));
        return String::new();
    }

    if name.contains('_') {
        return name.to_string();
    }

    if !is_camel_or_pascal_case(name) {
        println!(
            "{}",
            style("is not in camelCase format. Please provide a valid camelCase variable name.")
                .color256(208)
        );
        return String::new();
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
/// - `silent`: Whether to supress output
/// # Example
///
/// ```rust
/// untools::batch_convert("input.txt", "output.txt", true);
/// ```
pub fn batch_convert(ifile: &str, ofile: &str, is_constant: bool, silent: bool) {
    let contents = fs::read_to_string(ifile).expect("Unable to read file.");

    let converted_names: Vec<String> = contents
        .lines()
        .map(|line| camel_to_snake(line.trim(), is_constant))
        .collect();

    let output_content = converted_names.join("\n");
    fs::write(ofile, output_content).expect("Unable to write file.");
    if !silent {
        println!("Batch conversion successful! Results written to {}", ofile);
    }
}

/// Converts a snake_case string to a camelCase string.
///
/// # Arguments
///
/// * `name` - A string slice that holds the snake_case variable name to be converted.
/// * `to_lower_camel` - A boolean indicating whether the output should be in lowerCamelCase (true) or UpperCamelCase (false).
///
/// # Returns
///
/// * A String containing the camelCase representation of the input string.
///
pub fn snake_to_camel(name: &str, upper_case_camel: bool) -> String {
    let mut camel_case = String::new();
    let mut capitalize_next = true;

    let mut to_lower_camel = !upper_case_camel;
    
    if name.is_empty() {
        return String::from("Input string is empty. Please provide a valid variable name.");
    }

    if !starts_with_digit(name) {
        return String::from("Input string is a digit.");
    }

    for c in name.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                if to_lower_camel {
                    camel_case.push(c.to_ascii_lowercase());
                    to_lower_camel = false; // Reset to_lower_camel for the next word
                } else {
                    camel_case.push(c.to_ascii_uppercase());
                }
                capitalize_next = false;
            } else {
                camel_case.push(c);
            }
        }
    }
    camel_case
}
