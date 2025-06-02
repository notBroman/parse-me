use core::fmt;
use csv::StringRecord;
use std::{error::Error, io};
use time::{Duration, PrimitiveDateTime};

pub struct Entry {
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

pub struct Summary {
    pub month: time::Month,
    food_total: f32,
    misc_total: f32,
    housing_total: f32,
    incoming: f32,
    outgoing: f32,
}

impl From<StringRecord> for Entry {
    fn from(record: StringRecord) -> Self {
        let format =
            time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        Entry {
            what: record.get(0).unwrap().to_string().clone(),
            product: record.get(1).unwrap().to_string().clone(),
            started: PrimitiveDateTime::parse(
                record.get(2).expect("Slice should not be empty"),
                &format,
            )
            .unwrap(),
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
        return Ok(());
    }
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "In: {:?}\n Out: {:?}\n", self.incoming, self.outgoing)?;
        return Ok(());
    }
}

fn summarise() {
    todo!("reimplement summarising");
    //     entry_data: &Vec<Entry>,
    // ) -> Result<HashMap<(time::Month, i32), Summary>, Box<dyn Error>> {
    //     let mut overview: HashMap<(time::Month, i32), Summary> = HashMap::new();
    //     let food_stores: Vec<&str> = Vec::from(["Morrisons", "Tesco", "Co-op", "Many Mart", "Lidl"]);
    //
    //     for e in entry_data {
    //         let date_month: (time::Month, i32) = (e.started.month(), e.started.year());
    //         if !overview.contains_key(date_month.borrow()) {
    //             overview.insert(
    //                 date_month.clone(),
    //                 Summary {
    //                     month: e.started.month().clone(),
    //                     food_total: 0.0,
    //                     misc_total: 0.0,
    //                     housing_total: 0.0,
    //                     incoming: 0.0,
    //                     outgoing: 0.0,
    //                 },
    //             );
    //         }
    //
    //         // reference the summary of the given month to update
    //         let overview_entry: &mut Summary = overview.get_mut(date_month.borrow()).unwrap();
    //         // logic for adding to summary
    //         if e.amount < 0.0 {
    //             overview_entry.outgoing += e.amount;
    //         } else {
    //             overview_entry.incoming += e.amount;
    //         }
    //         if food_stores.contains(&e.description.as_str()) {
    //             overview_entry.food_total += e.amount;
    //         }
    //     }
    //
    //     return Ok(overview);
}
