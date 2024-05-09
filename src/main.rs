

//! # uutols crate
//! A simple and user-friendly underscore variable naming tool
/// Main function to demonstrate the usage of the ctsc_utils crate.
///
/// This function calls the `camel_to_snake_case` function from the `ctsc_utils` crate
/// to convert the variable name "myVariableName" from camelCase to snake_case format,
/// and then prints the result to the console.
mod ctsc_utils;
fn main() {
    //println!("out: {}", ctsc_utils::ctsc("myVariableName",false));
    let input_file = "input.txt";
    let output_file = "output.txt";
    ctsc_utils::batch_convert(input_file, output_file,true);
}
