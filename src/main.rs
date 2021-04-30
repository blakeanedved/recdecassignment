mod recdec;

use std::io::{self, Read};
use recdec::RecursiveDescentParser;

fn main() -> Result<(), &'static str> {
    
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut rdp = RecursiveDescentParser::new(&buffer);
    rdp.expect_program()?;

    println!("{} assignments", rdp.get_assignments());

    Ok(())
}
