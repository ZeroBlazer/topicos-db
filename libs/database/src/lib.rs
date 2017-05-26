extern crate rustc_serialize;
extern crate csv;

use std::collections::HashMap;

/******************** Athletes DB ********************/
#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct AthlRecord(String, u32, u32);

#[derive(Debug)]
pub struct AthlDatabase {
    keys: HashMap<String, usize>,
    data: Vec<AthlRecord>,
}

impl AthlDatabase {
    pub fn load(path: &str) -> AthlDatabase {
        let mut rdr = csv::Reader::from_file(path).unwrap().delimiter(b'\t').has_headers(true);

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
            data: data
        }
    }

    pub fn standarize(&mut self) {
        println!("Standarizing DB...");
        for (feat, ratings) in db.1.iter_mut() {
            let mut feat_vec: Vec<f32> = Vec::new();
            for rating in ratings.values() {
                feat_vec.push(*rating);
            }

            let (asd, median) = abs_standard_deviation(&feat_vec);
            for (usr, rating) in ratings.iter_mut() {
                *rating = (*rating - median) / asd;
                *db.0.get_mut(usr).unwrap().get_mut(feat).unwrap() = *rating;
            }
        }
    }
}
/*******************************************************/