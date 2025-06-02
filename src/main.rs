#![allow(warnings)]
use core::fmt;
use csv::{ReaderBuilder, StringRecord};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::{error::Error, io};
use time::{Duration, PrimitiveDateTime};

mod entry;
use entry::Entry;

mod my_tui;

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

fn main() {
    // let mut data_vector: Vec<Entry> = Vec::new();
    // if let Err(err) = data_from_csv(data_vector.borrow_mut()) {
    //     println!("{}", err);
    //     process::exit(1);
    // }
    //
    // println!("{:?}", data_vector.len());
    // let digest = summarise(data_vector.borrow()).unwrap();
    //
    // for (key, value) in digest {
    //     println!("{:?}, {:?}: {:?}\n", key.0, key.1, value.food_total);
    // }
    my_tui::func();
}
