extern crate csv;
extern crate rustc_serialize;

use std::fs::File;
use csv::Reader;

fn init_db_reader(path: &str) -> Reader<File> {
    let rdr = csv::Reader::from_file(path).unwrap();
    rdr
}

fn load_database() {
    let mut rdr = init_db_reader("./data/db1.csv").has_headers(true);
    let mut ratings: HashMap<String, HashMap<String, u32>> = HashMap::new();

    while !rdr.done() {
        
    }

}

fn similarity(user: &str, band: &str, k: u32, fun: fn(&Vec<f32>, &Vec<f32>) -> f32 {

}

fn main() {
    println!("Sim: {}", similarity("Dan", ));
    println!("Hello, world!");
}
