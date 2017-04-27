extern crate csv;
extern crate rustc_serialize;

use std::collections::HashMap;

fn init_db_reader(path: &str) -> Reader<File> {
    let rdr = csv::Reader::from_file(path).unwrap();
    rdr
}

fn load_db(path: &str) {
    let rdr = csv::Reader::from_file(path).has_headers(true).unwrap();
    let mut ratings: HashMap<u32, HashMap<String, u32>> = HashMap::new();

    // let mut i = 0;
    for record in rdr.decode() {
        // if i > 54000 {
        //     break;
        // }
        // i += 1;
        let rating: Rating = record.unwrap();

        match ratings.entry(rating.user_id) {
            Vacant(entry) => {
                let mut user_ratings = HashMap::new();
                user_ratings.insert(rating.book_isbn, rating.rating);
                entry.insert(user_ratings);
            }
            Occupied(entry) => {
                entry.into_mut().insert(rating.book_isbn, rating.rating);
            }
        }
    }

    
}

fn main() {
    let db = load_db("./data/ratings.csv");
    println!("Hello, world!");
}

// #[derive(Debug)]
// struct Database {
//     headers: HashMap<String, u32>,
//     names: HashMap<String, u32>,
//     records: Vec<f32>,
// }

// impl Database {
//     fn new(path: &str) {
//         let mut rdr = csv::Reader::from_file(path).unwrap().has_headers(true);
//         let mut i = 0;
//         for record in rdr.decode() {
//             i+=1;
//         }
//         println!("{}", i);
//     }
// }