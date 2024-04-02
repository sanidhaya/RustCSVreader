use csv;
use std::{env::args, error::Error, fs::File, io::{BufReader, Read}};

fn read_from_file(path: &str) -> Result<(),Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records(){
        let record = result?;

        println!( "{:?}",record);
    }
    Ok(())
}

fn main() {
    if args().len() != 2{
        eprintln!("usage : `source`");
        return;
    }

    let mut input = args().nth(1).unwrap();

    if let Err(e) = read_from_file(&input){
        eprintln!("Error reading {}",input)
    }
}

