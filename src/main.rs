mod error;
mod iching;

use std::env;

fn main() -> Result<(), error::Error> {
    let args: Vec<String> = env::args().collect();
    let mode = if args.len() > 0 {
        iching::Mode::from(&args[0])
    } else {
        iching::Mode::Random
    };

    let result = iching::create_reading(mode)?;
    println!("Hello, world!");
    Ok(())
}
