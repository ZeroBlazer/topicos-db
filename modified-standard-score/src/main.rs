extern crate csv;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

//////////////////////////////DISTANCES////////////////////////////////
fn manhattan_dist(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    if x.len() != y.len() {
        panic!("Should compare vectors of same size");
    }
    let mut distance = 0.0;

    for i in 0..x.len() {
        if x[i] > 0.0 && y[i] > 0.0 {
            distance += (x[i] - y[i]).abs();
        }
    }

    distance
}
///////////////////////////////////////////////////////////////////////

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

fn median(vec: &Vec<f32>) -> f32 {
    let mut vec_cpy = vec.clone();
    vec_cpy.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let length = vec_cpy.len();
    let mid = length / 2;
    let mut ret = vec_cpy[mid];
    if length % 2 == 0 {
        ret += vec_cpy[mid - 1];
        ret /= 2.0;
    }
    println!("{} /2 -> {}", length, length / 2);
    ret
}

fn abs_standard_deviation(vec: &Vec<f32>) -> (f32, f32) {
    let median = median(vec);
    let mut asd = 0.0;

    for x in vec.iter() {
        asd += (x - median).abs();
    }

    (asd / vec.len() as f32, median)
}

fn mod_standard_score(val: f32, vec: &Vec<f32>) -> f32 {
    let (asd, median) = abs_standard_deviation(vec);

    (val - median) / asd
}

// fn user_rating_vector(db: &IndexedDB, id: &str) -> Vec<f32> {
//     let mut ret_vec = Vec::new();
//     if let Some(ref ratings) = db.0.get(&String::from(id)) {
//         for (_, &rating) in ratings.iter() {
//             ret_vec.push(rating);
//         }
//     }

//     ret_vec
// }

fn users_rating_vectors(db: &IndexedDB, id1: &str, id2: &str) -> (Vec<f32>, Vec<f32>) {
    let mut usr1_vec: Vec<f32> = Vec::new();
    let mut usr2_vec: Vec<f32> = Vec::new();

    if let Some(ref ratings1) = db.0.get(&String::from(id1)) {
        for (feat1_id, &rating1) in ratings1.iter() {
            usr1_vec.push(rating1);
            if let Some(ref ratings2) = db.0.get(&String::from(id2)) {
                if let Some(rating2) = ratings2.get(feat1_id) {
                    usr2_vec.push(*rating2);
                } else {
                    usr2_vec.push(0.0);
                }
            } else {
                panic!("user2 is not found!");
            }
        }
    } else {
        panic!("user1 not found!");
    }

    if let Some(ref ratings2) = db.0.get(&String::from(id2)) {
        for (feat2_id, &rating2) in ratings2.iter() {
            if let Some(ref ratings1) = db.0.get(&String::from(id1)) {
                if let Some(_) = ratings1.get(feat2_id) {
                } else {
                    usr2_vec.push(rating2);
                    usr1_vec.push(0.0);
                }
            } else {
                panic!("user2 is not found!");
            }
        }
    } else {
        panic!("user1 not found!");
    }

    (usr1_vec, usr2_vec)
}

fn nearest_neighbors(db: &IndexedDB,
                     id: &str,
                     func: fn(&Vec<f32>, &Vec<f32>) -> f32)
                     -> Vec<(f32, String)> {
    // let obj_vec = user_rating_vector(db, id);
    let mut dist_vec: Vec<(f32, String)> = Vec::new();

    // println!("OBJ: {:?}", obj_vec);

    for (rec_id, _) in db.0.iter() {
        if id != rec_id.as_str() {
            let rec_str = rec_id.clone();
            let (obj_vec, rec_vec) = users_rating_vectors(db, id, rec_id);
            println!("{:?}", rec_vec);
            println!("{} -> {}", rec_str, func(&obj_vec, &rec_vec));
            dist_vec.push((func(&obj_vec, &rec_vec), rec_str));
        }
    }

    dist_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    dist_vec
}

fn main() {
    println!("Loading database, please wait...");
    let db = load_db("./data/music.csv");
    println!("Database ready!\n---------------------------------------------");

    println!("{:?}", nearest_neighbors(&db, "Dr Dog/Fate", manhattan_dist));

    // println!("Med: {:?}", abs_standard_deviation(&mut vec![43., 45., 55., 69., 70., 75., 105., 115.]));
}
