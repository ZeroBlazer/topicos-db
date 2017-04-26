extern crate csv;
extern crate rustc_serialize;

use std::collections::HashMap;

fn adjusted_cosine(vec_feat_1: Vec<f32>, vec_feat_2: Vec<f32>, vec_avg: Vec<f32>) -> f32 {
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

    usr_pref / (feat_1_sqr.sqrt() * feat_2_sqr.sqrt())
}

fn get_user_vector(records: &(HashMap<String, u32>, HashMap<String, u32>, Vec<f32>),
                   name: &str)
                   -> Vec<f32> {
    let mut vec = Vec::new();
    let indx = records.1.get(name).unwrap();
    let i = records.0.len();

    for x in 0..i {
        vec.push(records.2[(*indx as usize * i + x) as usize]);
    }

    vec
}

fn get_feature_vector(records: &(HashMap<String, u32>, HashMap<String, u32>, Vec<f32>),
                      feat: &str)
                      -> Vec<f32> {
    let mut vec = Vec::new();
    let indx = records.0.get(feat).unwrap();
    let i = records.0.len();
    let j = records.1.len();

    for y in 0..j {
        vec.push(records.2[(y * i + *indx as usize) as usize]);
    }

    vec
}

fn get_user_avg(records: &(HashMap<String, u32>, HashMap<String, u32>, Vec<f32>)) -> Vec<f32> {
    let mut vec = Vec::new();
    let i = records.0.len();
    let j = records.1.len();

    for y in 0..j {
        let mut rated = 0;
        let mut avg = 0.0;
        for x in 0..i {
            let rating = records.2[(y * i + x) as usize];
            if rating != 0.0 {
                avg += rating;
                rated += 1;
            }
        }
        avg /= rated as f32;
        vec.push(avg);
    }

    vec
}

fn normalize(vect: &Vec<f32>) -> Vec<f32> {
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

    print!("max: {}, min: {}\n", max, min);

    for rating in vect.iter() {
        if *rating > 0.0 {
            let norm = (2.0 * (rating - min) - (max - min)) / (max - min);
            normd_vec.push(norm);
        }
    }

    normd_vec
}

fn load_database(path: &str) -> (HashMap<String, u32>, HashMap<String, u32>, Vec<f32>) {
    let mut rdr = csv::Reader::from_file(path).unwrap().has_headers(true);
    let mut headers: HashMap<String, u32> = HashMap::new();
    let mut names: HashMap<String, u32> = HashMap::new();
    let mut records: Vec<f32> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    loop {
        match rdr.next_bytes() {
            csv::NextField::Data(data) => {
                let d_string = String::from_utf8(data.to_vec()).unwrap();
                match j {
                    0 => {
                        /***********HEADERS***********/
                        if i > 0 {
                            headers.insert(d_string, i - 1);
                        }
                    }
                    _ => {
                        /***********RECORDS***********/
                        match i {
                            0 => {
                                /***********NAMES***********/
                                names.insert(d_string, j - 1);
                            }
                            _ => {
                                /***********VALUES***********/
                                records.push(d_string.parse::<f32>().unwrap());
                            }
                        }
                    }
                }
                i += 1;
            }
            csv::NextField::EndOfRecord => {
                j += 1;
                i = 0;
            }
            csv::NextField::EndOfCsv => break,
            csv::NextField::Error(err) => panic!(err),
        }
    }

    (headers, names, records)
}

fn item_similarity(records: &(HashMap<String, u32>, HashMap<String, u32>, Vec<f32>),
                   feat_1: &str,
                   feat_2: &str)
                   -> f32 {
    let v_feat_1 = get_feature_vector(&records, feat_1);
    let v_feat_2 = get_feature_vector(&records, feat_2);
    let v_avg = get_user_avg(&records);

    println!("AVG: {:?}", v_avg);

    adjusted_cosine(v_feat_1, v_feat_2, v_avg)
}

fn prediction(name: &str, feature: &str) -> f32 {
    // let records = load_database("./data/db1.csv");
    let records = load_database("./data/db1.1.csv");

    let user_vec = get_user_vector(&records, name);
    let normd_vec = normalize(&user_vec);
    let mut sim_vec = Vec::new();

    let mut i = 0;
    for (_feat, &_indx) in records.0.iter() {
        if _feat != feature {
            print!("{} - ", _feat);
            sim_vec.push(item_similarity(&records, _feat, feature));
        }
        i += 1;
    }

    println!("{:?}", normd_vec);
    println!("{:?}", sim_vec);

    3.1416
}

fn main() {
    println!("Prediction: {}", prediction("David", "Kacey Musgraves"));
}
