

//! # uutols crate
//! A simple and user-friendly underscore variable naming tool
/// Main function to demonstrate the usage of the ctsc_utils crate.
///
/// This function calls the `camel_to_snake_case` function from the `ctsc_utils` crate
/// to convert the variable name "myVariableName" from camelCase to snake_case format,
/// and then prints the result to the console.
//mod utils;
use untools::{camel_to_snake,batch_convert,snake_to_camel};
fn main() {
    println!("{}", camel_to_snake("121",true));
    let input_file = "input.txt";
    let output_file = "output.txt";
    //batch_convert(input_file, output_file,true);
    //println!("res :{}",snake_to_camel("_case_test",false));
}
