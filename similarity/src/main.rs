extern crate csv;
extern crate rustc_serialize;

use std::fs::File;
use csv::Reader;

fn main() {
    let path = "./data/db1.csv";
    let rdr = csv::Reader::from_file(path).unwrap();

    for record in rdr.decode() {
        
    }
}
