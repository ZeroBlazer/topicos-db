extern crate csv;
extern crate rustc_serialize;

// use csv;
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

    // for y in 0..5 {
    //     for x in 0..j {
    //         print!("{}, ", records[(y * j + x) as usize]);
    //     }
    //     println!("");
    // }
    (headers, names, records)
}

fn item_similarity(feat_1: &str, feat_2: &str) -> f32 {
    let records = load_database("./data/db1.csv");

    let v_feat_1 = get_feature_vector(&records, feat_1);
    let v_feat_2 = get_feature_vector(&records, feat_2);

    println!("{:?}", v_feat_1);
    println!("{:?}", v_feat_2);

    3.1416
}

fn main() {
    println!("Sim: {}", item_similarity("Kacey Musgraves", "Daft Punk"));
    println!("Hello, world!");
}
