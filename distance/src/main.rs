extern crate serde_json;

use std::fs::File;
use std::io::Read;
use serde_json::{Value, Error};

fn read_db(path: &str) -> Value{
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

    println!("{}, {}, {}, {}",
             prod_xy,
             (sum_x * sum_y) / n_dims as f32,
             sqr_diff_x,
             sqr_diff_y);

    (prod_xy - (sum_x * sum_y) / n_dims as f32) /
    ((sqr_diff_x - avg_sqr_x).sqrt() * (sqr_diff_y - avg_sqr_y).sqrt())
}

fn distance(db: &Value, a: &str, b: &str, func: fn(&Vec<f32>, &Vec<f32>) -> f32) -> f32 {
    // let vec_a: Vec<f32>;
    // let vec_b: Vec<f32>;
    // for cal in db["califications"] {
    //     if cal["name"] == a {
    //         vec_a = cal["scores"];
    //     } 
    // }
    // println!("vec: {}", vec_a);
    
    // println!("{}", manhattan_dist(&vec_a, &vec_b));
    2.0
}

// fn prediction(k: u32, fun: fn(&Vec<f32>, &Vec<f32>) -> f32, user: str, band: str) -> f32 {}

fn main() {
    let db = read_db("data/db.json");
    println!("Distance = {}", distance(&db, "Angelica", "Bill", manhattan_dist));
    // let users = vec!["Angelica", "Bill", "Chan", "Dan", "Harley", "Jordyn", "Sam", "Veronica"];
    // let bands = vec!["Bues", "Brown", "Dead", "Nora", "Phoenix", "Slightly", "Strokes", "Vampire"];

    // let angelica: Vec<f32> = vec![3.5, 2., 0., 4.5, 5., 1.5, 2.5, 2.];
    // let bill: Vec<f32> = vec![2., 3.5, 4., 0., 2., 3.5, 0., 3.];
    // let chan: Vec<f32> = vec![5., 1., 1., 3., 5., 1., 0., 0.];
    // let dan: Vec<f32> = vec![3., 4., 4.5, 0., 3., 4.5, 4., 2.];
    // let harley: Vec<f32> = vec![0., 4., 1., 4., 0., 0., 4., 1.];
    // let jordyn: Vec<f32> = vec![0., 4.5, 4., 5., 5., 4.5, 4., 4.];
    // let sam: Vec<f32> = vec![5., 2., 0., 3., 5., 4., 5., 0.];
    // let veronica: Vec<f32> = vec![3., 0., 0., 5., 4., 2.5, 3., 0.];

    // let clara: Vec<f32> = vec![4.75, 4.5, 5., 4.25, 4.];
    // let roberts: Vec<f32> = vec![4., 3., 5., 2., 1.];

    // println!("{}", manhattan_dist(&angelica, &bill));
    // println!("{}", euclidian_dist(&angelica, &bill));
    // println!("{}", cosine_dist(&angelica, &bill));
    // println!("{}", pearson_coef(&clara, &roberts));
    // println!("{}", pearson_coef(&angelica, &bill));
    // println!("{}", prediction(3));
}
