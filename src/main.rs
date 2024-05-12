//! # uutols crate
//! A simple and user-friendly underscore variable naming tool
use std::io;

/// Main function to demonstrate the usage of the ctsc_utils crate.
///
/// This function calls the `camel_to_snake_case` function from the `ctsc_utils` crate
/// to convert the variable name "myVariableName" from camelCase to snake_case format,
/// and then prints the result to the console.
//mod utils;
use untools::{camel_to_snake, snake_to_camel, batch_convert};

use clap::{Parser, Args};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    input: String,
    
    #[arg(long, short = 'c')]
    is_constant: bool,
    
    #[command(flatten)]
    conversion_type: ConversionType,
    
    #[arg(long, short = 's')]
    silent: bool
}

#[derive(Args, Debug, Clone)]
#[group(required = true, multiple = false)]
struct ConversionType {
    #[arg(long)]
    camel_to_snake: bool,
    
    #[arg(long)]
    snake_to_camel: bool,
    
    #[arg(long, value_name = "OUTPUT_FILE")]
    batch: Option<String>,
}


fn main() -> io::Result<()> {
    let args = Cli::parse();
    
    let input = args.input;
    
    let is_constant = args.is_constant;
    let silent = args.silent;
    
    let conversion_type = args.conversion_type;
    let (cts, stc, b) = (conversion_type.camel_to_snake, conversion_type.snake_to_camel, conversion_type.batch);
    
    let mut result: Option<String> = None;
    match (cts, stc, b) {
        (true, _, _) => { result = Some(camel_to_snake(input.as_str(), is_constant)); }
        
        (_, true, _) => { result = Some(snake_to_camel(input.as_str(), is_constant))},
        
        (_, _, Some(output_file)) => { batch_convert(input.as_str(), output_file.as_str(), is_constant, silent) },
        
        _ => { unreachable!() }
    }
    
    if !silent {
        println!("Output: ");
    }
    
    if cts || stc {
        if result.is_some() {
            println!("{}", result.unwrap());
        } 
    }
    
    println!("\nCompleted!");
    
    Ok(())
}
