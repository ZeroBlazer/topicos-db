extern crate csv;
extern crate rustc_serialize;

use std::fs::File;
use csv::Reader;
use std::collections::HashMap;

// fn adjusted_cosine(vec_feat_1: Vec<f32>, vec_feat_2: Vec<f32>) -> f32 {
//     if vec_feat_1.len() != vec_feat_2.len() {
//         panic!("Should compare vectors of same size");
//     }

//     let n_users = vec_feat_1.len();

//     let mut usr_pref = 0.0;
//     let mut feat_1_sqr = 0.0;
//     let mut feat_2_sqr = 0.0;

//     for i in 0..n_users {
//         if vec_feat_1[i] > 0.0 && vec_feat_2[i] > 0.0 {
//             let name = db["names"].as_array().unwrap()[i as usize]
//                 .as_str()
//                 .unwrap();
//             let avg = db["scores"][name].as_array().unwrap()[0]
//                 .as_f64()
//                 .unwrap() as f32;
//             let feat_1_inf = vec_feat_1[i] - avg;
//             let feat_2_inf = vec_feat_2[i] - avg;
//             usr_pref += feat_1_inf * feat_2_inf;
//             feat_1_sqr += feat_1_inf.powf(2.0);
//             feat_2_sqr += feat_2_inf.powf(2.0);
//         }
//     }

//     usr_pref / (feat_1_sqr.sqrt() * feat_2_sqr.sqrt())
// }

fn load_database(path: &str) {
    let mut rdr = csv::Reader::from_file(path)
        .unwrap()
        .has_headers(true)
        .double_quote(true);
    let mut headers: HashMap<String, u32> = HashMap::new();
    let mut names: HashMap<String, u32> = HashMap::new();
    let mut records: Vec<Vec<i32>> = Vec::new();

    let mut i = 0;
    let mut j = 0;
    loop {
        match rdr.next_bytes() {
            csv::NextField::Data(data) => {
                let d_string = String::from_utf8(data.to_vec()).unwrap();
                match j {
                    0 => {
                        if i > 0 {
                            println!("{} -> {}", i - 1, d_string);
                            headers.insert(d_string, i - 1);
                        }
                    }
                    r => {
                        match i {
                            0 => {
                                names.insert(d_string, j - 1);
                                print!("{} -> ", j - 1);
                            }
                            c => {
                                print!("{} ", d_string.parse::<f32>().unwrap());
                            }
                        }
                    }
                }
                i += 1;
            }
            csv::NextField::EndOfRecord => {
                j += 1;
                i = 0;
                println!("\\");
            }
            csv::NextField::EndOfCsv => break,
            csv::NextField::Error(err) => panic!(err),
        }
    }
}

fn item_similarity(user: &str, band: &str) -> f32 {
    load_database("./data/db1.csv");
    3.1416
}

fn main() {
    println!("Sim: {}", item_similarity("Kacey Musgraves", "Daft Punk"));
    println!("Hello, world!");
}
