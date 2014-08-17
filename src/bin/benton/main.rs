#![crate_name = "benton"]
#![feature(globs, macro_rules)]

extern crate opentype;

use std::{io, os};
use result::*;

mod result;

fn main() {
    match start(&os::args()) {
        Ok(()) => println!("Done."),
        Err(error) => {
            println!("{}", error);
            os::set_exit_status(1);
        }
    }
}

fn start(arguments: &Vec<String>) -> Result<()> {
    if arguments.len() != 2 {
        raise!(ArgumentError, "Usage: {} <file>", arguments[0]);
    }

    let filename: &str = arguments[1].as_slice();

    println!("Filename: {}", filename);

    let mut stream = try!(io::File::open(&Path::new(filename)), ArgumentError);
    let font = try!(opentype::parse(&mut stream), ParseError);

    println!("{}", font);

    Ok(())
}
