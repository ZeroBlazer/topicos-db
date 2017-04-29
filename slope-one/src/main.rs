extern crate csv;
extern crate rustc_serialize;

#[macro_use]
extern crate text_io;

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
                panic!("1: feat_2 is not found!");
            }
        }
    } else {
        panic!("1: feat_1 is not found!");
    }

    let length = vec_a.len();
    let mut deviation = 0.0;
    for i in 0..length {
        deviation += (vec_a[i] - vec_b[i]) / length as f32;
    }

    (deviation, length)
}

// fn item_based_prediction_input(db: &mut IndexedDB,
//                                sim_fun: fn(&MovieDB, String, String) -> f32)
//                                -> f32 {
//     let user_id: String;
//     let movie_id: String;
//     println!("Enter user_id y movie_id:");
//     scan!("{} {}", user_id, movie_id);

//     match db.0.entry(user_id) {
//         Vacant(entry) => {
//             let mut ratings: HashMap<u32, f32> = HashMap::new();
//             let mut movie: u32;
//             let mut rating: f32;
//             for i in 0..10 {
//                 println!("Enter movie_id:");
//                 scan!("{}", movie);
//                 println!("Enter rating:");
//                 scan!("{}", rating);
//                 ratings.insert(movie, rating);
//                 match db.1.entry(user_id) {    
//                     Vacant(usr_map) => {
//                         let mut user_rtngs: HashMap<u32, f32> = HashMap::new();
//                         user_rtngs.insert(user_id, rating);
//                         usr_map.insert(user_rtngs);
//                     }
//                     Occupied(usr_map) => {
//                         usr_map.into_mut().insert(user_id, rating);
//                     }
//                 }
//             }
//             entry.insert(ratings);
//         }
//         Occupied(_) => {}
//     }

//     sim_fun(&db, user_id, movie_id)
// }

fn main() {
    println!("Loading database, please wait...");
    let mut db = load_db("./data/db2.csv");
    // let mut db = load_db("../report01/data/BX-Dump/BX-Book-Ratings.csv");
    println!("Database ready!\n________________________________________________");

    println!("{}", deviation(&db, "Taylor Swift", "PSY"));
    println!("{}", deviation(&db, "PSY", "Whitney Houston"));
    println!("{}", deviation(&db, "Taylor Swift", "Whitney Houston"));

    // let mut ender: u32;
    // loop {
    //     println!("Press 1 to make a query with Slope One.\n\
    //               Press 2 to make a query with Adjusted Cosine.\n\
    //               Press 0 to exit: ");
    //     scan!("{}", ender);
    //     match ender {
    //         1 => {
    //             println!("Slope One");
    //         }
    //         2 => {
    //             println!("Item similarity");
    //         }
    //         0 => {
    //             break;
    //         }
    //         _ => {
    //             println!("Try again");
    //         }
    //     }
    //     println!("\n______________________________________________________");
    // }
    // println!("\nBye bytes...");
}
