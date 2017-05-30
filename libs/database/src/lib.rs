extern crate rustc_serialize;
extern crate csv;
extern crate utilities;
extern crate distance;

use std::collections::HashMap;
use utilities::abs_standard_deviation;
use distance::manhattan_dist;

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
            asd_data: Vec::new()
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

    fn nearest_neighbors(&self, rcrd: &AthlRecord, func: fn(&Vec<f32>, &Vec<f32>) -> f32) -> Vec<usize> {
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

        rcrd.class = self.data[self.nearest_neighbors(&rcrd, manhattan_dist)[0]].class.clone();
        // println!("{:?}", self.data[self.nearest_neighbors(&rcrd, manhattan_dist)[0]]);
        rcrd
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
            asd_data: Vec::new()
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
}
/*******************************************************/



// def test(training_filename, test_filename):
//     """Test the classifier on a test set of data"""
//     classifier = Classifier(training_filename)
//     f = open(test_filename)
//     lines = f.readlines()
//     f.close()
//     numCorrect = 0.0
//     for line in lines:
//         data = line.strip().split('\t')
//         vector = []
//         classInColumn = -1
//         for i in range(len(classifier.format)):
//               if classifier.format[i] == 'num':
//                   vector.append(float(data[i]))
//               elif classifier.format[i] == 'class':
//                   classInColumn = i
//         theClass= classifier.classify(vector)
//         prefix = '-'
//         if theClass == data[classInColumn]:
//             # it is correct
//             numCorrect += 1
//             prefix = '+'
//         print("%s  %12s  %s" % (prefix, theClass, line))
//     print("%4.2f%% correct" % (numCorrect * 100/ len(lines)))
