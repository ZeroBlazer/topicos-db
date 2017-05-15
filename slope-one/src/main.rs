extern crate csv;
extern crate rustc_serialize;
extern crate time;

#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use time::PreciseTime;

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

fn adjusted_cosine(vec_feat_1: &Vec<f32>, vec_feat_2: &Vec<f32>, vec_avg: &Vec<f32>) -> f32 {
    if vec_feat_1.len() != vec_feat_2.len() || vec_feat_1.len() != vec_avg.len() {
        panic!("Should compare vectors of same size");
    }

    let n_users = vec_feat_1.len();

    let mut usr_pref = 0.0;
    let mut feat_1_sqr = 0.0;
    let mut feat_2_sqr = 0.0;

    for i in 0..n_users {
        /*  THIS CODE APPLIES WHEN NO EXPLICIT 0 SCORES WERE GIVEN   */
        // if vec_feat_1[i] > 0.0 && vec_feat_2[i] > 0.0 {
        //     let feat_1_inf = vec_feat_1[i] - vec_avg[i];
        //     let feat_2_inf = vec_feat_2[i] - vec_avg[i];
        //     usr_pref += feat_1_inf * feat_2_inf;
        //     feat_1_sqr += feat_1_inf.powf(2.0);
        //     feat_2_sqr += feat_2_inf.powf(2.0);
        // }
            let feat_1_inf = vec_feat_1[i] - vec_avg[i];
            let feat_2_inf = vec_feat_2[i] - vec_avg[i];
            usr_pref += feat_1_inf * feat_2_inf;
            feat_1_sqr += feat_1_inf.powf(2.0);
            feat_2_sqr += feat_2_inf.powf(2.0);
    }

    // print!(">> {} / {} ", usr_pref, (feat_1_sqr.sqrt() * feat_2_sqr.sqrt()));

    usr_pref / (feat_1_sqr.sqrt() * feat_2_sqr.sqrt())
}

fn computer_user_avg(db: &IndexedDB, user: &str) -> f32 {
    let mut avg: f32 = 0.0;
    if let Some(ref ratings) = db.0.get(&String::from(user)) {
        for (_, &rating) in ratings.iter() {
            avg += rating;
        }
        avg /= ratings.len() as f32
    }
    avg
}

fn adjusted_cosine_features(db: &IndexedDB, feat_1: &str, feat_2: &str) -> f32 {
    let mut vec_a: Vec<f32> = Vec::new();
    let mut vec_b: Vec<f32> = Vec::new();
    let mut vec_avg: Vec<f32> = Vec::new();

    if let Some(ref ratings_f1) = db.1.get(&String::from(feat_1)) {
        for (user_id, &rating_f1) in ratings_f1.iter() {
            vec_avg.push(computer_user_avg(&db, user_id));
            if let Some(ratings_f2) = db.1.get(&String::from(feat_2)) {
                if let Some(&rating_f2) = ratings_f2.get(user_id) {
                    vec_a.push(rating_f1);
                    vec_b.push(rating_f2);
                } else {
                    vec_a.push(rating_f1);
                    vec_b.push(0.0);
                }
            } else {
                panic!("1: feat_2 is not found!");
            }
        }
    } else {
        panic!("1: feat_1 is not found!");
    }

    if let Some(ref ratings_f2) = db.1.get(&String::from(feat_2)) {
        for (user_id, &rating_f2) in ratings_f2.iter() {
            if let Some(ratings_f1) = db.1.get(&String::from(feat_1)) {
                if let Some(_) = ratings_f1.get(user_id) {
                } else {
                    vec_a.push(0.0);
                    vec_b.push(rating_f2);
                    vec_avg.push(computer_user_avg(&db, user_id));
                }
            } else {
                panic!("2: feat_1 is not found!");
            }
        }
    } else {
        panic!("2: feat_2 is not found!");
    }

    adjusted_cosine(&vec_a, &vec_b, &vec_avg)
}

fn prediction(sim_vec: Vec<f32>, normd_vec: Vec<f32>) -> f32 {
    let mut num = 0.0;
    let mut den = 0.0;
    let inv_i = sim_vec.len();
    for i in 0..inv_i {
        num += sim_vec[i] * normd_vec[i];
        den += sim_vec[i].abs();
        // print!(">>> {} <> {} ", sim_vec[i], normd_vec[i]);   /////////debug
    }
    // println!(">>");   /////////debug

    num / den
}

fn normalize(vect: &Vec<f32>) -> (Vec<f32>, f32, f32) {
    let mut normd_vec = Vec::with_capacity(vect.len());

    let mut max = vect[0];
    let mut min = vect[0];

    for rating in vect.iter() {
        if *rating > max {
            max = *rating;
        }
        /*  THIS CODE APPLIES WHEN NO EXPLICIT 0 SCORES WERE GIVEN   */
        // if *rating != 0.0 && *rating < min || min == 0.0 {
        //     min = *rating;
        // }
        if *rating < min {
            min = *rating;
        }
    }

    for rating in vect.iter() {
        /*  THIS CODE APPLIES WHEN NO EXPLICIT 0 SCORES WERE GIVEN   */
        // if *rating > 0.0 {
        //     let norm = (2.0 * (rating - min) - (max - min)) / (max - min);
        //     normd_vec.push(norm);
        // }
        let norm = (2.0 * (rating - min) - (max - min)) / (max - min);
        normd_vec.push(norm);
    }

    (normd_vec, max, min)
}

fn unnormalize(norm_val: f32, max: f32, min: f32) -> f32 {
    0.5 * (norm_val + 1.0) * (max - min) + min
}

fn adjusted_cosine_prediction(db: &IndexedDB, user: &str, feature: &str) -> f32 {
    let mut sim_vec: Vec<f32> = Vec::new();
    let mut usr_vec: Vec<f32> = Vec::new();

    match db.0.get(&String::from(user)) {
        Some(a_ratings) => {
            for (feat_id, &rating) in a_ratings.iter() {
                sim_vec.push(adjusted_cosine_features(db, &feature, feat_id));
                usr_vec.push(rating);
            }
        }
        None => {}
    }

    let normd_usr_vec = normalize(&usr_vec);
    let normd_pred = prediction(sim_vec, normd_usr_vec.0);
    println!("Normalized prediction: {}", normd_pred);
    unnormalize(normd_pred, normd_usr_vec.1, normd_usr_vec.2)
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

    let start = PreciseTime::now();
    let similarity = sim_fun(&db, user_id.as_str(), feat_id.as_str());
    let end = PreciseTime::now();
    println!("Execution time: {} seconds", start.to(end));
    similarity
}

fn main() {
    println!("Loading database, please wait...");
    // let mut db = load_db("./data/db2.csv");
    // let mut db = load_db("../report01/data/BX-Dump/BX-Book-Ratings.csv");
    // let mut db = load_db("../../../Downloads/ml-20m/ratings.csv");
    let mut db = load_db("../../../Downloads/ml-latest-small/ratings2.csv"); // 311 1479
    println!("Database ready!\n---------------------------------------------");

    let mut ender: u32;
    loop {
        println!("Press 1 to make a query with Slope One.\n\
                  Press 2 to make a query with Adjusted Cosine.\n\
                  Press 0 to exit: ");
        scan!("{}", ender);
        match ender {
            1 => {
                println!("S1p: {}", prediction_input(&mut db, weighted_slope_one));
            }
            2 => {
                println!("ACp: {}",
                         prediction_input(&mut db, adjusted_cosine_prediction));
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
