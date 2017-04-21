extern crate csv;

use std::fs::File;
use csv::Reader;

fn init_db(path: &str) -> Reader<File> {
    let mut rdr = csv::Reader::from_file(path).unwrap();
    rdr
}

fn main() {
    let mut rdr = init_db("./data/Movie_Ratings.csv");

    for record in rdr.decode() {
        let (s1, s2, dist): (String, String, usize) = record.unwrap();
        println!("({}, {}): {}", s1, s2, dist);
    }

    println!("Hello, world!");
}
