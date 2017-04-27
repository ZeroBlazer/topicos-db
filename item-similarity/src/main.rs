extern crate csv;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

struct MovieDB(HashMap<u32, HashMap<u32, f32>>, HashMap<u32, HashMap<u32, f32>>);

fn adjusted_cosine(vec_feat_1: &Vec<f32>, vec_feat_2: &Vec<f32>, vec_avg: &Vec<f32>) -> f32 {
    if vec_feat_1.len() != vec_feat_2.len() || vec_feat_1.len() != vec_avg.len() {
        panic!("Should compare vectors of same size");
    }

    let n_users = vec_feat_1.len();

    let mut usr_pref = 0.0;
    let mut feat_1_sqr = 0.0;
    let mut feat_2_sqr = 0.0;

    for i in 0..n_users {
        if vec_feat_1[i] > 0.0 && vec_feat_2[i] > 0.0 {
            let feat_1_inf = vec_feat_1[i] - vec_avg[i];
            let feat_2_inf = vec_feat_2[i] - vec_avg[i];
            usr_pref += feat_1_inf * feat_2_inf;
            feat_1_sqr += feat_1_inf.powf(2.0);
            feat_2_sqr += feat_2_inf.powf(2.0);
        }
    }

    print!("[{} -> {}, {}]\n",
           usr_pref / (feat_1_sqr.sqrt() * feat_2_sqr.sqrt()),
           usr_pref,
           (feat_1_sqr.sqrt() * feat_2_sqr.sqrt()));
    usr_pref / (feat_1_sqr.sqrt() * feat_2_sqr.sqrt())
}

fn computer_user_avg(db: &MovieDB, user: &u32) -> f32 {
    let mut avg: f32 = 0.0;
    match db.0.get(&user) {
        Some(ref usr_ratings) => {
            for (_, &rating) in usr_ratings.iter() {
                avg += rating;
            }
            avg /= usr_ratings.len() as f32
        }
        None => {}
    }
    avg
}

fn adjusted_cosine_features(db: &MovieDB, feat_1: &u32, feat_2: &u32) -> f32 {
    let mut vec_a: Vec<f32> = Vec::new();
    let mut vec_b: Vec<f32> = Vec::new();
    let mut vec_avg: Vec<f32> = Vec::new();

    if let Some(ref ratings_f1) = db.1.get(&feat_1) {
        for (user_id, &rating_f1) in ratings_f1.iter() {
            vec_avg.push(computer_user_avg(&db, user_id));
            if let Some(ratings_f2) = db.1.get(&feat_2) {
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

    if let Some(ref ratings_f2) = db.1.get(&feat_2) {
        for (user_id, &rating_f2) in ratings_f2.iter() {
            if let Some(ratings_f1) = db.1.get(&feat_1) {
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
    }

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
        if *rating != 0.0 && *rating < min || min == 0.0 {
            min = *rating;
        }
    }

    for rating in vect.iter() {
        if *rating > 0.0 {
            let norm = (2.0 * (rating - min) - (max - min)) / (max - min);
            normd_vec.push(norm);
        }
    }

    (normd_vec, max, min)
}

fn unnormalize(norm_val: f32, max: f32, min: f32) -> f32 {
    0.5 * (norm_val + 1.0) * (max - min) + min
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

fn item_based_prediction(db: &MovieDB, name: u32, feature: u32) -> f32 {
    let mut sim_vec: Vec<f32> = Vec::new();
    let mut usr_vec: Vec<f32> = Vec::new();

    match db.0.get(&name) {
        Some(a_ratings) => {
            for (movie_id, &rating) in a_ratings.iter() {
                // print!("{}, ", movie_id);
                sim_vec.push(adjusted_cosine_features(db, &feature, movie_id));
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

fn main() {
    let db = load_db("./data/ratings.csv");
    println!("Prediction: {}", item_based_prediction(&db, 11, 5679));

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