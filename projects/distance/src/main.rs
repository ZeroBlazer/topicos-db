extern crate serde_json;

use std::fs::File;
use std::io::Read;
use serde_json::Value;
// use serde_json::{Value, Error};

fn read_db(path: &str) -> Value {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let v: Value = serde_json::from_str(&data).unwrap();
    v
}

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

fn euclidian_dist(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    if x.len() != y.len() {
        panic!("Should compare vectors of same size");
    }
    let mut distance = 0.0;

    for i in 0..x.len() {
        distance += (x[i] - y[i]).powf(2.0);
    }

    distance.sqrt()
}

fn cosine_dist(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    if x.len() != y.len() {
        panic!("Should compare vectors of same size");
    }

    let mut dot = 0.0;
    let mut den_a = 0.0;
    let mut den_b = 0.0;

    for i in 0..x.len() {
        dot += x[i] * y[i];
        den_a += x[i] * x[i];
        den_b += y[i] * y[i];
    }
    dot / (den_a.sqrt() * den_b.sqrt())
}

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

fn get_vector(db: &Value, name: &str) -> Vec<f32> {
    let mut vec: Vec<f32> = vec![];
    for elem in db["scores"][name].as_array().unwrap().iter() {
        vec.push(elem.as_f64().unwrap() as f32);
    }

    vec
}

fn distance(db: &Value, a: &str, b: &str, func: fn(&Vec<f32>, &Vec<f32>) -> f32) -> f32 {
    let vec_a = get_vector(&db, a);
    let vec_b = get_vector(&db, b);

    func(&vec_a, &vec_b)
}

fn prediction(db: &Value,
              user: &str,
              //   band: &str,
              band: u32,
              k: u32,
              fun: fn(&Vec<f32>, &Vec<f32>) -> f32)
              -> f32 {
    let mut distances: Vec<(f32, &str)> = Vec::new();
    for it in db["names"].as_array().unwrap().iter() {
        let name = it.as_str().unwrap();
        if name != user {
            distances.push((distance(&db, user, name, fun), name));
        }
    }

    // distances.sort_by_key(|k| k.0);
    distances.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let mut sum_dist = 0.0;
    for i in 0..k {
        sum_dist += distances[i as usize].0;
    }

    let mut projection = 0.0;
    for i in 0..k {
        projection += distances[i as usize].0 *
                      (db["scores"][(distances[i as usize]).1]
                           .as_array()
                           .unwrap()
                           [band as usize]
                               .as_f64()
                               .unwrap() as f32) / sum_dist;
    }

    projection
}

fn get_feature_vector(db: &Value, feat: u32) -> Vec<f32> {
    let mut vec: Vec<f32> = vec![];
    for it in db["names"].as_array().unwrap().iter() {
        let name = it.as_str().unwrap();
        vec.push(db["scores"][name].as_array().unwrap()[feat as usize]
                     .as_f64()
                     .unwrap() as f32);
    }

    vec
}

fn adjusted_cosine(db: &Value, feat_1: u32, feat_2: u32) -> f32 {
    let vec_feat_1 = get_feature_vector(db, feat_1);
    let vec_feat_2 = get_feature_vector(db, feat_2);

    if vec_feat_1.len() != vec_feat_2.len() {
        panic!("Should compare vectors of same size");
    }

    let n_users = vec_feat_1.len();

    let mut usr_pref = 0.0;
    let mut feat_1_sqr = 0.0;
    let mut feat_2_sqr = 0.0;

    for i in 0..n_users {
        if vec_feat_1[i] > 0.0 && vec_feat_2[i] > 0.0 {
            let name = db["names"].as_array().unwrap()[i as usize]
                .as_str()
                .unwrap();
            let avg = db["scores"][name].as_array().unwrap()[0]
                .as_f64()
                .unwrap() as f32;
            let feat_1_inf = vec_feat_1[i] - avg;
            let feat_2_inf = vec_feat_2[i] - avg;
            usr_pref += feat_1_inf * feat_2_inf;
            feat_1_sqr += feat_1_inf.powf(2.0);
            feat_2_sqr += feat_2_inf.powf(2.0);
        }
    }

    usr_pref / (feat_1_sqr.sqrt() * feat_2_sqr.sqrt())
}

fn main() {
    // let db = read_db("data/db.json");
    // println!("Distance = {}",
    //          distance(&db, "Angelica", "Bill", cosine_dist));
    // println!("Prediction = {}",
    //          prediction(&db, "Dan", 3, 5, manhattan_dist));
    // println!("Prediction = {}",
    //          prediction(&db, "Dan", 3, 5, euclidian_dist));
    // println!("Prediction = {}", prediction(&db, "Dan", 3, 5, cosine_dist));
    // println!("Prediction = {}",
    //          prediction(&db, "Dan", 3, 5, pearson_coef));

    let db = read_db("data/db2.json");
    println!("S = {}", adjusted_cosine(&db, 1, 4));
}
