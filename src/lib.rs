pub mod command;
pub mod constant;

use std::{error::Error, io};

pub fn check_args() -> Result<(String, Vec<String>), Box<dyn Error>> {
    // if there is an iterator, use that iterator instead of cloning
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut input = input.split_ascii_whitespace();
    let command = input.next().ok_or("")?.to_string();
    let option = input.map(|x| x.to_string()).collect::<Vec<String>>();
    Ok((command, option))
}
