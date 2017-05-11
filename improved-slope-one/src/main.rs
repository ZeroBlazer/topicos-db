extern crate csv;
extern crate rustc_serialize;
extern crate stopwatch;

#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use stopwatch::{Stopwatch};

struct IndexedDB(HashMap<String, HashMap<String, f32>>, HashMap<String, HashMap<String, f32>>);

fn load_db(path: &str) -> IndexedDB {
    // let mut rdr = csv::Reader::from_file(path).unwrap().has_headers(false);
    let mut rdr = csv::Reader::from_file(path).unwrap().has_headers(true);
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

fn deviation(db: &IndexedDB, feat_1: &str, feat_2: &str) -> (f32, usize) {
    let mut vec_a: Vec<f32> = Vec::new();
    let mut vec_b: Vec<f32> = Vec::new();

    if let Some(ref ratings_f1) = db.1.get(&String::from(feat_1)) {
        for (user_id, &rating_f1) in ratings_f1.iter() {
            if let Some(ratings_f2) = db.1.get(&String::from(feat_2)) {
                if let Some(&rating_f2) = ratings_f2.get(user_id) {
                    vec_a.push(rating_f1);
                    vec_b.push(rating_f2);
                }
            } else {
                panic!("feat_2 is not found!");
            }
        }
    } else {
        panic!("feat_1 is not found!");
    }

    let length = vec_a.len();
    let mut deviation = 0.0;
    for i in 0..length {
        deviation += (vec_a[i] - vec_b[i]) / length as f32;
    }

    (deviation, length)
}

fn weighted_slope_one(db: &IndexedDB, user: &str, feat: &str) -> f32 {
    let mut num = 0.0;
    let mut den = 0.0;
    
    if let Some(ref ratings) = db.0.get(&String::from(user)) {
        for (feat_id, &rating) in ratings.iter() {
            let (dev, card) = deviation(&db, feat, feat_id);
            num += (dev + rating) * card as f32;
            den += card as f32;
        }
    } else {
        panic!("user is not found!");
    }

    num / den
}

fn prediction_input(db: &mut IndexedDB, sim_fun: fn(&IndexedDB, &str, &str) -> f32) -> f32 {
    let user_id: String;
    let feat_id: String;
    println!("Enter {{user}} y {{feature}}:");
    scan!("{} {}", user_id, feat_id);

    if let Some(_) = db.0.get(&user_id) {
    } else {
        println!("\nUser not found, you'll be requested to enter some ratings.\nEnter number of ratings...");
        let n_ratings: u32;
        scan!("{}", n_ratings);

        let mut ratings: HashMap<String, f32> = HashMap::new();
        let mut feat: String;
        let mut rating: f32;

        for i in 0..n_ratings {
            println!("{}. Enter feature and rating:", i);
            scan!("{} {}", feat, rating);

            let ins_feat = feat.clone();

            match db.1.entry(feat) {
                Vacant(entry) => {
                    let mut feat_ratings = HashMap::new();
                    feat_ratings.insert(user_id.clone(), rating);
                    entry.insert(feat_ratings);
                }
                Occupied(entry) => {
                    entry.into_mut().insert(user_id.clone(), rating);
                }
            }

            ratings.insert(ins_feat, rating);
        }
        
        db.0.insert(user_id.clone(), ratings);
    }

    let sw = Stopwatch::start_new();
    let similarity = sim_fun(&db, user_id.as_str(), feat_id.as_str());
    println!("Execution time: {}ms", sw.elapsed_ms());

    similarity
}

fn main() {
    println!("Loading database, please wait...");
    // let mut db = load_db("../report01/data/BX-Dump/BX-Book-Ratings.csv");
    // let mut db = load_db("../../../Downloads/ml-20m/ratings.csv");
    let mut db = load_db("../../../Downloads/ml-latest-small/ratings.csv"); // 311 1479
    println!("Database ready!\n---------------------------------------------");

    let mut ender: u32;
    loop {
        println!("Press 1 to make a query with Slope One.\n\
                  Press 0 to exit: ");
        scan!("{}", ender);
        match ender {
            1 => {
                println!("S1p: {}", prediction_input(&mut db, weighted_slope_one));
            }
            0 => {
                break;
            }
            _ => {
                println!("Try again");
            }
        }
        println!("\nNum. of Records: {} items, {} records", db.0.len(), db.1.len());
        println!("\n-------------------------------------------------------");
    }
    println!("\nBye bytes...");
}
