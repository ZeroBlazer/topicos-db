extern crate csv;
extern crate rustc_serialize;
extern crate time;
// extern crate stopwatch;

#[macro_use]
extern crate text_io;

use time::PreciseTime;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
// use stopwatch::Stopwatch;

struct IndexedDB(HashMap<String, HashMap<String, f32>>, HashMap<String, HashMap<String, f32>>);

fn pearson_coef(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    if x.len() != y.len() {
        panic!("Should compare vectors of same size");
    }

    let n_dims = x.len();

    let mut prod_xy = 0.0;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;

    for i in 0..n_dims {
        sum_x += x[i];
        sum_y += y[i];
        prod_xy += x[i] * y[i];
    }

    let avg_sqr_x = sum_x.powf(2.0) / n_dims as f32;
    let avg_sqr_y = sum_y.powf(2.0) / n_dims as f32;
    let mut sqr_diff_x = 0.0;
    let mut sqr_diff_y = 0.0;

    for i in 0..x.len() {
        sqr_diff_x += x[i].powf(2.0);
        sqr_diff_y += y[i].powf(2.0);
    }

    (prod_xy - (sum_x * sum_y) / n_dims as f32) /
    ((sqr_diff_x - avg_sqr_x).sqrt() * (sqr_diff_y - avg_sqr_y).sqrt())
}

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

fn knn_pearson_coef(db: &IndexedDB, user: &str, k: u32) -> IndexedDB {
    let ref ratings_u = db.0.get(&String::from(user)).unwrap();
    let mut pearson_coefs: Vec<(f32, String)> = Vec::new();

    for (user_2, ref ratings_2) in db.0.iter() {
        let mut vec_a: Vec<f32> = Vec::new();
        let mut vec_b: Vec<f32> = Vec::new();

        // if user_2 != user {
            for (feat_u, &rating_u) in ratings_u.iter() {
                if let Some(&rating) = ratings_2.get(feat_u) {
                    vec_a.push(rating_u);
                    vec_b.push(rating);
                } else {
                    vec_a.push(rating_u);
                    vec_b.push(0.0);
                }
            }

            for (feat_2, &rating_2) in ratings_2.iter() {
                if let Some(_) = ratings_u.get(feat_2) {
                } else {
                    vec_a.push(0.0);
                    vec_b.push(rating_2);
                }
            }
            pearson_coefs.push((pearson_coef(&vec_a, &vec_b), user_2.clone()));
        // }
    }

    pearson_coefs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut db_ratings: HashMap<String, HashMap<String, f32>> = HashMap::new();
    let mut db_features: HashMap<String, HashMap<String, f32>> = HashMap::new();

    for _ in 0..(k+1) {
        let (_, user_id) = pearson_coefs.pop().unwrap();

        if let Some(ratings) = db.0.get(&user_id) {
            for (feat_id, &rating) in ratings.iter() {
                let user_id1 = user_id.clone();
                let (user_id2, feat_id2, rating2) = (user_id.clone(), feat_id.clone(), rating);

                match db_ratings.entry(user_id1) {
                    Vacant(entry) => {
                        let mut usr_ratings = HashMap::new();
                        usr_ratings.insert(feat_id.clone(), rating);
                        entry.insert(usr_ratings);
                    }
                    Occupied(entry) => {
                        entry.into_mut().insert(feat_id.clone(), rating);
                    }
                }

                match db_features.entry(feat_id2) {
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
        } else {
            println!("Couldn't find ratings for user");
        }
    }

    IndexedDB(db_ratings, db_features)
}

fn improved_slope_one(db: &IndexedDB, user: &str, feat: &str) -> f32 {
    let mut num = 0.0;
    let mut den = 0.0;

    let s_k = knn_pearson_coef(db, user, 10);
    // let s_k = knn_pearson_coef(db, user, 2);

    if let Some(ref ratings) = s_k.0.get(&String::from(user)) {
        for (feat_id, &rating) in ratings.iter() {
            let (dev, card) = deviation(&s_k, feat, feat_id);
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
        println!("\nUser not found, you'll be requested to enter some ratings.\
                  \nEnter number of ratings...");
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

    // let sw = Stopwatch::start_new();
    let start = PreciseTime::now();
    let similarity = sim_fun(&db, user_id.as_str(), feat_id.as_str());
    let end = PreciseTime::now();
    println!("Execution time: {} seconds", start.to(end));
    // println!("Execution time: {}ms", sw.elapsed_ms());

    similarity
}

fn main() {
    println!("Loading database, please wait...");
    // let mut db = load_db("../slope-one/data/db2.csv");
    // let mut db = load_db("../report01/data/BX-Dump/BX-Book-Ratings.csv");
    // let mut db = load_db("../../../Downloads/ml-20m/ratings.csv");
    let mut db = load_db("../../../Downloads/ml-latest-small/ratings2.csv"); // 311 1479
    println!("Database ready!\n---------------------------------------------");

    let mut ender: u32;
    loop {
        println!("Press 1 to make a query with Slope One.\n\
                  Press 0 to exit: ");
        scan!("{}", ender);
        match ender {
            1 => {
                println!("I-S1p: {}", prediction_input(&mut db, improved_slope_one));
            }
            0 => {
                break;
            }
            _ => {
                println!("Try again");
            }
        }
        println!("\nNum. of Records: {} items, {} records",
                 db.0.len(),
                 db.1.len());
        println!("\n-------------------------------------------------------");
    }
    println!("\nBye bytes...");
}
