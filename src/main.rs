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

struct Summary {
    month: time::Month,
    food_total: i32,
    misc_total: i32,
    housing_total: i32,
    incoming: i32,
    outgoing: i32,
}

impl From<StringRecord> for Entry {

    fn from(record: StringRecord) -> Self {
        let format = time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        Entry {
            what: record.get(0).unwrap().to_string().clone(),
            product: record.get(1).unwrap().to_string().clone(),
            started: PrimitiveDateTime::parse(record.get(2)
                .expect("Slice should not be empty"), &format).unwrap(),
            completed: PrimitiveDateTime::parse(record.get(3).
                expect("Slice should not be enpty"), &format).unwrap(),
            description: record.get(4).unwrap().to_string().clone(),
            amount: i32::from_str_radix(record.get(5).unwrap(), 10).unwrap(),
            fee: i32::from_str_radix(record.get(6).unwrap(), 10).unwrap(),
            currency: record.get(7).unwrap().to_string().clone(),
            state: record.get(8).unwrap().to_string().clone(),
            balance: i32::from_str_radix(record.get(9).unwrap(), 10).unwrap(),
        }
    }

}


fn data_from_csv() -> Result<Vec<Entry>, Box<dyn Error>> {
    let mut data_vector: Vec<Entry>= Vec::new();

    let mut rdr = csv::Reader::from_reader(io::stdin());
    for res in rdr.records() {
        let record = res?;
        // clone the fields of the record into the entry
        data_vector.push(Entry::from(record));
    }

    Ok(data_vector)
}

fn summarise(data: Vec<Entry>) -> Result<HashMap<time::Month, Summary> , Box<dyn Error>> {

}

fn main (){
    if let Err(err) = data_from_csv() {
        println!("{}", err);
        process::exit(1);
    }

    
}


