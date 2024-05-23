use std::{error::Error, io, process};
use std::collections::HashMap;
use time::{PrimitiveDateTime, parsing, formatting, macros};
use csv::{ReaderBuilder, StringRecord};

struct Entry {
    what: String,
    product: String,
    started: PrimitiveDateTime,
    completed: PrimitiveDateTime,
    description: String,
    amount: i32,
    fee: i32,
    currency: String,
    state: String,
    balance: i32
}

fn main (){
    if let Err(err) = parse() {
        println!("{}", err);
        process::exit(1);
    }
}

fn parse() -> Result<(), Box<dyn Error>>{
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // Loop over each record.
    let format = time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let mut map: HashMap<(i32, time::Month), i32> = HashMap::new();
    let mut vendors: HashMap<String, i32> = HashMap::new();

    for result in rdr.records() {
        // An error may occur, the error is returhed after checking for it.
        let record = result?;

        let transaction_start = PrimitiveDateTime::parse(record.get(2)
            .expect("Slice should not be empty"), &format)?;
        let trans_month = (transaction_start.year(), transaction_start.month());
        let occ = map.get(&trans_month);
        if occ == None {
            map.insert(trans_month, 1);
        } else {
            map.insert(trans_month, occ.unwrap() + 1);
        }

        let vendor: String = record.get(4).unwrap().to_string();
        if !vendors.contains_key(&vendor) {
            vendors.insert(vendor, 1);
        } else {
            vendors.insert(vendor.clone(), vendors.get(&vendor).unwrap() + 1);
        }
    }

    for (key, val) in map.iter(){
        println!("{:?} {:?}, {val}", key.0, key.1);
    }
    for (key, val) in vendors.iter(){
        println!("{key}, {val}");
    }
    Ok(())
}
