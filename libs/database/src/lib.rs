extern crate rustc_serialize;
extern crate csv;
extern crate utilities;
extern crate distance;
extern crate rand;
extern crate quick_csv;

use std::collections::HashMap;
use utilities::abs_standard_deviation;
use distance::{manhattan_dist, euclidian_dist, pearson_coef};
// use rand::Rng;

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
    asd_data: Vec<(f32, f32)>,
}

impl AthlDatabase {
    pub fn from_file(path: &str) -> AthlDatabase {
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
            asd_data: Vec::new(),
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
                                  _ => {
                    panic!("Out of range, fn covers only two options");
                }
                              });
            }

            let (asd, median) = abs_standard_deviation(&feat_vec);
            println!("\t{}> asd: {}\tmedian: {}", i, asd, median);

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

            self.asd_data.push((asd, median));
        }
    }

    fn nearest_neighbors(&self,
                         rcrd: &AthlRecord,
                         func: fn(&Vec<f32>, &Vec<f32>) -> f32)
                         -> Vec<usize> {
        let feats = vec![rcrd.height, rcrd.weight];
        let mut distances: Vec<(f32, usize)> = Vec::new();
        let mut i = 0;
        for record in self.data.iter() {
            distances.push((func(&feats, &vec![record.height, record.weight]), i));
            i += 1;
        }
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut indexes = Vec::new();
        for record in distances.iter() {
            indexes.push(record.1);
        }

        indexes
    }

    pub fn predict(&self, height: f32, weight: f32) -> AthlRecord {
        let mut rcrd = AthlRecord {
            class: String::new(),
            height: height,
            weight: weight,
        };

        for i in 0..2 {
            match i {
                0 => {
                    rcrd.height = (rcrd.height - self.asd_data[i].1) / self.asd_data[i].0;
                }
                1 => {
                    rcrd.weight = (rcrd.weight - self.asd_data[i].1) / self.asd_data[i].0;
                }
                _ => {}
            }
        }

        rcrd.class = self.data[self.nearest_neighbors(&rcrd, manhattan_dist)[0]]
            .class
            .clone();
        // println!("{:?}", self.data[self.nearest_neighbors(&rcrd, manhattan_dist)[0]]);
        rcrd
    }

    pub fn test(training_path: &str, test_path: &str) -> f32 {
        println!("Loading database, please wait...");
        let mut db = AthlDatabase::from_file(training_path);
        db.standarize();
        println!("Database ready!\n---------------------------------------------");

        let mut rdr = csv::Reader::from_file(test_path)
            .unwrap()
            .delimiter(b'\t')
            .has_headers(true);

        let mut n_correct = 0;
        let mut n_incorrect = 0;
        let mut count = 0;
        for record in rdr.decode() {
            let rcrd: (String, AthlRecord) = record.unwrap();
            let pred = db.predict(rcrd.1.height, rcrd.1.weight);

            // println!("{} <> {}", rcrd.1.class, pred.class);
            if rcrd.1.class == pred.class {
                n_correct += 1;
            } else {
                n_incorrect += 1;
            }
            count += 1;
        }

        println!("Correct: {}%\nIncorrect: {}%\n",
                 n_correct as f32 * 100.0 / count as f32,
                 n_incorrect as f32 * 100.0 / count as f32);

        n_correct as f32 * 100.0 / count as f32
    }
}
/*******************************************************/

/******************** Miles per Gallon DB ********************/
#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct MpgRecord {
    mpg: f32,
    cylinders: f32,
    ci: f32,
    hp: f32,
    weight: f32,
    secs: f32,
}

#[derive(Debug)]
pub struct MpgDatabase {
    keys: HashMap<String, usize>,
    data: Vec<MpgRecord>,
    asd_data: Vec<(f32, f32)>,
}

impl MpgDatabase {
    pub fn new() -> MpgDatabase {
        MpgDatabase {
            keys: HashMap::new(),
            data: Vec::new(),
            asd_data: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> MpgDatabase {
        let mut rdr = csv::Reader::from_file(path)
            .unwrap()
            .delimiter(b'\t')
            .has_headers(true);

        let mut keys: HashMap<String, usize> = HashMap::new();
        let mut data: Vec<MpgRecord> = Vec::new();

        let mut indx = 0;
        for record in rdr.decode() {
            let rcrd: (MpgRecord, String) = record.unwrap();
            data.push(rcrd.0);
            keys.insert(rcrd.1, indx);

            indx += 1;
        }

        MpgDatabase {
            keys: keys,
            data: data,
            asd_data: Vec::new(),
        }
    }

    pub fn add_file(&mut self, path: &str) {
        let mut rdr = csv::Reader::from_file(path)
            .unwrap()
            .delimiter(b'\t')
            .has_headers(false);

        for record in rdr.decode() {
            let rcrd: (MpgRecord, String) = record.unwrap();
            self.keys.insert(rcrd.1, self.data.len());
            self.data.push(rcrd.0);
        }
    }

    pub fn standarize(&mut self) {
        println!("Standarizing DB...");
        for i in 0..6 {
            let mut feat_vec: Vec<f32> = Vec::new();
            for feat in self.data.iter() {
                feat_vec.push(match i {
                                  0 => feat.mpg,
                                  1 => feat.cylinders,
                                  2 => feat.ci,
                                  3 => feat.hp,
                                  4 => feat.weight,
                                  5 => feat.secs,
                                  _ => {
                    panic!("Out of range, fn covers only two options");
                }
                              });
            }

            let (asd, median) = abs_standard_deviation(&feat_vec);
            println!("\t{}> asd: {}\tmedian: {}", i, asd, median);

            for feat in self.data.iter_mut() {
                match i {
                    0 => {
                        feat.mpg = (feat.mpg - median) / asd;
                    }
                    1 => {
                        feat.cylinders = (feat.cylinders - median) / asd;
                    }
                    2 => {
                        feat.ci = (feat.ci - median) / asd;
                    }
                    3 => {
                        feat.hp = (feat.hp - median) / asd;
                    }
                    4 => {
                        feat.weight = (feat.weight - median) / asd;
                    }
                    5 => {
                        feat.secs = (feat.secs - median) / asd;
                    }
                    _ => {}
                }
            }

            self.asd_data.push((asd, median));
        }
    }

    fn nearest_neighbors(&self,
                         rcrd: &MpgRecord,
                         func: fn(&Vec<f32>, &Vec<f32>) -> f32)
                         -> Vec<usize> {
        let feats = vec![rcrd.cylinders, rcrd.ci, rcrd.hp, rcrd.weight, rcrd.secs];
        let mut distances: Vec<(f32, usize)> = Vec::new();
        let mut i = 0;
        for record in self.data.iter() {
            distances.push((func(&feats,
                                 &vec![record.cylinders,
                                       record.ci,
                                       record.hp,
                                       record.weight,
                                       record.secs]),
                            i));
            i += 1;
        }
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut indexes = Vec::new();
        for record in distances.iter() {
            indexes.push(record.1);
        }

        indexes
    }

    pub fn standarize_record(&self, record: &mut MpgRecord) {
        record.mpg = (record.mpg - self.asd_data[0].1) / self.asd_data[0].0;
        record.cylinders = (record.cylinders - self.asd_data[1].1) / self.asd_data[1].0;
        record.ci = (record.ci - self.asd_data[2].1) / self.asd_data[2].0;
        record.hp = (record.hp - self.asd_data[3].1) / self.asd_data[3].0;
        record.weight = (record.weight - self.asd_data[4].1) / self.asd_data[4].0;
        record.secs = (record.secs - self.asd_data[5].1) / self.asd_data[5].0;
    }

    pub fn predict(&self, cylinders: f32, ci: f32, hp: f32, weight: f32, secs: f32) -> MpgRecord {
        let mut rcrd = MpgRecord {
            mpg: 0.0,
            cylinders: cylinders,
            ci: ci,
            hp: hp,
            weight: weight,
            secs: secs,
        };

        for i in 0..6 {
            match i {
                // 0 => {
                //     rcrd.mpg = (rcrd.mpg - self.asd_data[i].1) / self.asd_data[i].0;
                // }
                1 => {
                    rcrd.cylinders = (rcrd.cylinders - self.asd_data[i].1) / self.asd_data[i].0;
                }
                2 => {
                    rcrd.ci = (rcrd.ci - self.asd_data[i].1) / self.asd_data[i].0;
                }
                3 => {
                    rcrd.hp = (rcrd.hp - self.asd_data[i].1) / self.asd_data[i].0;
                }
                4 => {
                    rcrd.weight = (rcrd.weight - self.asd_data[i].1) / self.asd_data[i].0;
                }
                5 => {
                    rcrd.secs = (rcrd.secs - self.asd_data[i].1) / self.asd_data[i].0;
                }
                _ => {}
            }
        }

        rcrd.mpg = self.data[self.nearest_neighbors(&rcrd, manhattan_dist)[0]].mpg;
        // rcrd.mpg = self.data[self.nearest_neighbors(&rcrd, euclidian_dist)[0]].mpg;
        // rcrd.mpg = self.data[self.nearest_neighbors(&rcrd, pearson_coef)[0]].mpg;
        rcrd
    }

    pub fn test(training_path: &str, test_path: &str, has_headers: bool) -> f32 {
        println!("Loading database, please wait...");
        let mut db = MpgDatabase::from_file(training_path);
        db.standarize();
        println!("Database ready!\n---------------------------------------------");

        let mut rdr = csv::Reader::from_file(test_path)
            .unwrap()
            .delimiter(b'\t')
            .has_headers(has_headers);

        let mut n_correct = 0;
        let mut n_incorrect = 0;
        let mut count = 0;
        for record in rdr.decode() {
            let mut rcrd: (MpgRecord, String) = record.unwrap();
            let pred = db.predict(rcrd.0.cylinders,
                                  rcrd.0.ci,
                                  rcrd.0.hp,
                                  rcrd.0.weight,
                                  rcrd.0.secs);

            db.standarize_record(&mut rcrd.0);
            // println!("{} <> {}", rcrd.0.mpg, pred.mpg);
            if rcrd.0.mpg == pred.mpg {
                n_correct += 1;
            } else {
                n_incorrect += 1;
            }
            count += 1;
        }

        println!("Correct: {}%\nIncorrect: {}%\n",
                 n_correct as f32 * 100.0 / count as f32,
                 n_incorrect as f32 * 100.0 / count as f32);
        // println!("Pred => {:?}", db.predict(8.0, 360.0, 215.0, 4615.0, 14.0));
        n_correct as f32 * 100.0 / count as f32
    }

    pub fn cross_validation(training_path: &str, n: usize, prefix: &str) {
        let mut presition = 0.0;

        for j in 1..n + 1 {
            let mut db = MpgDatabase::new();

            for i in 1..n + 1 {
                if i != j {
                    let path = format!("../../data/cross-validation/{}-{number:>0width$}",
                                       prefix,
                                       number = i,
                                       width = 2);
                    db.add_file(path.as_ref());
                }
            }

            db.standarize();
            let path = format!("../../data/cross-validation/{}-{number:>0width$}",
                               prefix,
                               number = j,
                               width = 2);
            let mut rdr = csv::Reader::from_file(path)
                .unwrap()
                .delimiter(b'\t')
                .has_headers(false);

            let mut n_correct = 0;
            let mut n_incorrect = 0;
            let mut count = 0;
            for record in rdr.decode() {
                let mut rcrd: (MpgRecord, String) = record.unwrap();
                let pred = db.predict(rcrd.0.cylinders,
                                      rcrd.0.ci,
                                      rcrd.0.hp,
                                      rcrd.0.weight,
                                      rcrd.0.secs);

                db.standarize_record(&mut rcrd.0);

                if rcrd.0.mpg == pred.mpg {
                    n_correct += 1;
                } else {
                    n_incorrect += 1;
                }
                count += 1;
            }

            println!("Correct: {}%\nIncorrect: {}%\n",
                     n_correct as f32 * 100.0 / count as f32,
                     n_incorrect as f32 * 100.0 / count as f32);

            presition += n_correct as f32 * 100.0 / count as f32;
        }

        presition /= n as f32;
        println!("Avg pres: {}%", presition);
    }
}
/*******************************************************/
pub mod generic;