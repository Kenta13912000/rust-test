use rust_test::parse_file;
use rust_test::parse_schema;
use rust_test::validate;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_file("config.txt")?;
    println!("{:#?}", config);

    let schema = parse_schema("schema.txt")?;

    if let Err(e) = validate(&config, &schema) {
        println!("Validation error: {}", e);
        std::process::exit(1);
    }

    println!("Validation OK!");

    Ok(())
}
