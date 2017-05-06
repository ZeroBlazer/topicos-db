extern crate csv;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

struct IndexedDB(HashMap<String, HashMap<String, f32>>, HashMap<String, HashMap<String, f32>>);

fn load_db(path: &str) -> IndexedDB {
    let mut rdr = csv::Reader::from_file(path).unwrap().has_headers(false);
    let mut ratings: HashMap<String, HashMap<String, f32>> = HashMap::new();
    let mut features: HashMap<String, HashMap<String, f32>> = HashMap::new();

    for record in rdr.decode() {
        let (user_id, feat_id, rating): (String, String, f32) = record.unwrap();
        let (user_id2, feat_id2, rating2) = (user_id.clone(), feat_id.clone(), rating);

        match ratings.entry(user_id) {
            Vacant(entry) => {
                let mut user_ratings = HashMap::new();
                user_ratings.insert(feat_id, rating);
                entry.insert(user_ratings);
            }
            Occupied(entry) => {
                entry.into_mut().insert(feat_id, rating);
            }
        }

        match features.entry(feat_id2) {
            Vacant(entry) => {
                let mut movie_ratings = HashMap::new();
                movie_ratings.insert(user_id2, rating2);
                entry.insert(movie_ratings);
            }
            Occupied(entry) => {
                entry.into_mut().insert(user_id2, rating2);
            }
        }
    }

    IndexedDB(ratings, features)
}

fn main() {
    println!("Loading database, please wait...");
    let mut db = load_db("./data/music.csv");
    println!("Database ready!\n---------------------------------------------");
}
