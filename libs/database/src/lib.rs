extern crate rustc_serialize;
extern crate csv;
extern crate utilities;

use std::collections::HashMap;
use utilities::abs_standard_deviation;

/******************** Athletes DB ********************/
#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct AthlRecord {
    class: String,
    height: f32,
    weight: f32,
}

#[derive(Debug)]
pub struct AthlDatabase {
    keys: HashMap<String, usize>,
    data: Vec<AthlRecord>,
}

impl AthlDatabase {
    pub fn load(path: &str) -> AthlDatabase {
        let mut rdr = csv::Reader::from_file(path)
            .unwrap()
            .delimiter(b'\t')
            .has_headers(true);

        let mut keys: HashMap<String, usize> = HashMap::new();
        let mut data: Vec<AthlRecord> = Vec::new();

        let mut indx = 0;
        for record in rdr.decode() {
            let rcrd: (String, AthlRecord) = record.unwrap();
            keys.insert(rcrd.0, indx);
            data.push(rcrd.1);

            indx += 1;
        }

        AthlDatabase {
            keys: keys,
            data: data,
        }
    }

    pub fn standarize(&mut self) {
        println!("Standarizing DB...");
        for i in 0..2 {
            let mut feat_vec: Vec<f32> = Vec::new();
            for feat in self.data.iter() {
                feat_vec.push(match i {
                                  0 => feat.height,
                                  1 => feat.weight,
                                  _ => { panic!("Out of range, fn covers only two options"); }
                              });
            }

            let (asd, median) = abs_standard_deviation(&feat_vec);
            for feat in self.data.iter_mut() {
                match i {
                    0 => {
                        feat.height = (feat.height - median) / asd;
                    }
                    1 => {
                        feat.weight = (feat.weight - median) / asd;
                    }
                    _ => {}
                }
            }
        }
    }
}
/*******************************************************/
