//! # uutols crate
//! A simple and user-friendly underscore variable naming tool
use std::io;
use untools::{camel_to_snake, snake_to_camel, batch_convert};
use clap::{Parser, Args};

/// Command line arguments structure.
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

/// Structure defining the conversion type options.
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

/// Main function to parse command line arguments and execute the appropriate conversion.
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