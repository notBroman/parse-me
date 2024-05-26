use core::fmt;
use std::borrow::{Borrow, BorrowMut};
use std::str::FromStr;
use std::{error::Error, io, process};
use std::collections::HashMap;
use time::{PrimitiveDateTime, parsing, formatting, macros};
use csv::{ReaderBuilder, StringRecord};

struct Entry {
    what: String,
    product: String,
    started: PrimitiveDateTime,
    description: String,
    amount: f32,
    fee: f32,
    currency: String,
    state: String,
    balance: f32,
}

struct Summary {
    month: time::Month,
    food_total: f32,
    misc_total: f32,
    housing_total: f32,
    incoming: f32,
    outgoing: f32,
}


impl From<StringRecord> for Entry {

    fn from(record: StringRecord) -> Self {
        let format = time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        Entry {
            what: record.get(0).unwrap().to_string().clone(),
            product: record.get(1).unwrap().to_string().clone(),
            started: PrimitiveDateTime::parse(record.get(2)
                .expect("Slice should not be empty"), &format).unwrap(),
            description: record.get(4).unwrap().to_string().clone(),
            amount: record.get(5).unwrap().parse().unwrap(),
            fee: record.get(6).unwrap().parse().unwrap(),
            currency: record.get(7).unwrap().to_string().clone(),
            state: record.get(8).unwrap().to_string().clone(),
            balance: record.get(9).unwrap().parse().expect(""),
        }
    }
}

impl fmt::Display for Entry {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} spent at {:?}", self.amount, self.description)?;
        Ok(())
    }
}


fn data_from_csv(data_vector: &mut Vec<Entry>) -> Result<(), Box<dyn Error>> {
    // allocate the vector to hold the data

    // read all the data into the vactor from the csv file
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for res in rdr.records() {
        let record = res?;
        // clone the fields of the record into the entry
        if record.get(9).unwrap() != "" {
            data_vector.push(Entry::from(record));
        }
    }
    data_vector.shrink_to_fit();
    // println!("{:?}", data_vector.len());
    return Ok(());
}

fn summarise(entry_data: &Vec<Entry>) -> Result<HashMap<time::Month, Summary> , Box<dyn Error>> {
    let mut overview: HashMap<time::Month, Summary> = HashMap::new();
    let food_stores: Vec<&str> = Vec::from(["Morrisons", "Tesco"
                                                ,"Co-op", "Many Mart"
                                                , "Lidl"]);
    
    for e in entry_data {
        if !overview.contains_key(&e.started.month()) {
            overview.insert(e.started.month().clone(), Summary {month: e.started.month().clone(),
                                                        food_total: 0.0,
                                                        misc_total: 0.0,
                                                        housing_total: 0.0,
                                                        incoming: 0.0,
                                                        outgoing: 0.0});
        }
    
        // reference the summary of the given month to update
        let overview_entry: &mut Summary = overview.get_mut(&e.started.month()).unwrap();
        // logic for adding to summary
        if e.amount < 0.0 {
            overview_entry.outgoing += e.amount;
        }
        else {
            overview_entry.incoming += e.amount;
        }
        if food_stores.contains(&e.description.as_str()){
            overview_entry.food_total += e.amount;
            println!("{e}");

        }

    }

    return Ok(overview);
}

fn main (){
    let mut data_vector: Vec<Entry> = Vec::new();
    if let Err(err) = data_from_csv(data_vector.borrow_mut()) {
        println!("{}", err);
        process::exit(1);
    }

    println!("{:?}", data_vector.len());
    let _ = summarise(data_vector.borrow());


    
}


