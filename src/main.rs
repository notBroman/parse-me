use std::{error::Error, io, process};

fn main (){
    if let Err(err) = parse() {
        println!("{}", err);
        process::exit(1);
    }
}

fn parse() -> Result<(), Box<dyn Error>>{
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, the error is returhed after checking for it.
        let record = result?;
        println!("{:?}", record);
        // Print a debug version of the record.
    }
    Ok(())
}
