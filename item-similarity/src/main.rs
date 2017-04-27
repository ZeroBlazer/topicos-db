extern crate csv;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

struct MovieDB(HashMap<u32, HashMap<u32, f32>>, HashMap<u32, HashMap<u32, f32>>);

fn item_based_prediction(name: u32, feature: u32) -> f32 {
    
}

fn load_db(path: &str) -> MovieDB {
    let mut rdr = csv::Reader::from_file(path).unwrap().has_headers(true);
    let mut ratings: HashMap<u32, HashMap<u32, f32>> = HashMap::new();
    let mut features: HashMap<u32, HashMap<u32, f32>> = HashMap::new();

    for record in rdr.decode() {
        let (user_id, movie_id, rating): (u32, u32, f32) = record.unwrap();

        match ratings.entry(user_id) {
            Vacant(entry) => {
                let mut user_ratings = HashMap::new();
                user_ratings.insert(movie_id, rating);
                entry.insert(user_ratings);
            }
            Occupied(entry) => {
                entry.into_mut().insert(movie_id, rating);
            }
        }

        match features.entry(movie_id) {
            Vacant(entry) => {
                let mut movie_ratings = HashMap::new();
                movie_ratings.insert(user_id, rating);
                entry.insert(movie_ratings);
            }
            Occupied(entry) => {
                entry.into_mut().insert(user_id, rating);
            }
        }
    }

    MovieDB(ratings, features)
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