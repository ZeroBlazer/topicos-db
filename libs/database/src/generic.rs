use quick_csv::Csv;
use rustc_serialize;
use utilities::abs_standard_deviation;

pub trait Record {
    fn record_len() -> usize;
    fn data_at(&self, index: usize) -> f32;
    fn standarize_field(&mut self, index: usize, asd_median: &(f32, f32));
    fn values(&self) -> Vec<f32>;
}

#[derive(Debug, RustcDecodable)]
pub struct MpgRecord {
    class: u32,
    values: [f32; 5],
}

impl Record for MpgRecord {
    fn record_len() -> usize {
        5
    }

    fn data_at(&self, index: usize) -> f32 {
        self.values[index]
    }

    fn standarize_field(&mut self, index: usize, asd_median: &(f32, f32)) {
        self.values[index] = (self.values[index] - asd_median.1) / asd_median.0;
    }

    fn values(&self) -> Vec<f32> {
        self.values.to_vec()
    }
}

#[derive(Debug, RustcDecodable)]
pub struct IrisRecord {
    class: String,
    values: [f32; 4],
}

impl Record for IrisRecord {
    fn record_len() -> usize {
        4
    }

    fn data_at(&self, index: usize) -> f32 {
        self.values[index]
    }

    fn standarize_field(&mut self, index: usize, asd_median: &(f32, f32)) {
        self.values[index] = (self.values[index] - asd_median.1) / asd_median.0;
    }

    fn values(&self) -> Vec<f32> {
        self.values.to_vec()
    }
}

#[derive(Debug)]
pub struct Database<T> {
    data: Vec<T>,
    abs_sd: Vec<(f32, f32)>,
}

impl<T> Database<T>
    where T: rustc_serialize::Decodable + ::std::fmt::Debug + Record
{
    pub fn new() -> Database<T> {
        Database {
            data: Vec::new(),
            abs_sd: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> Database<T> {
        let rdr = Csv::from_file(path).unwrap().has_header(true);
        let mut data: Vec<T> = Vec::new();
        for row in rdr.into_iter() {
            match row.unwrap().decode::<T>() {
                Ok(cols) => data.push(cols),
                Err(error) => println!("{}", error),
            }
        }

        Database {
            data: data,
            abs_sd: Vec::new(),
        }
    }

    pub fn add_file(&mut self, path: &str) {
        let rdr = Csv::from_file(path).unwrap();
        for row in rdr.into_iter() {
            match row.unwrap().decode::<T>() {
                Ok(cols) => self.data.push(cols),
                Err(error) => println!("{}", error),
            }
        }
    }

    pub fn standarize(&mut self) {
        println!("Standarizing DB...");
        let record_len = T::record_len();
        let mut mult_feat_vec = vec![Vec::<f32>::new(); record_len];

        for rcrd in self.data.iter() {
            for i in 0..record_len {
                mult_feat_vec[i].push(rcrd.data_at(i));
            }
        }

        let mut i = 0;
        for feat_vec in mult_feat_vec.iter() {
            let asd_median_tup = abs_standard_deviation(&feat_vec);
            println!("\t{}> asd: {}\tmedian: {}", i, asd_median_tup.0, asd_median_tup.1);

            for rcrd in self.data.iter_mut() {
                rcrd.standarize_field(i, &asd_median_tup);
            }

            self.abs_sd.push(asd_median_tup);
            i += 1;
        }
    }

    fn nearest_neighbors(&self, rcrd: &T, func: fn(&Vec<f32>, &Vec<f32>) -> f32) -> Vec<usize> {
        let mut distances: Vec<(f32, usize)> = Vec::new();
        let mut i = 0;
        for record in self.data.iter() {
            distances.push((func(&rcrd.values(), &record.values()), i));
            i += 1;
        }
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let indexes = distances.into_iter().map(|x| x.1).collect();
        indexes
    }

    pub fn predict(&self, vals: Vec<f32>) -> T {
        let record_len = T::record_len();
        if val 
        let mut record = T::new();
        record.set_values(vals.as_ref());
        for i in 0..record_len {
            rcrd.standarize_field(i, self.abs_sd[i]);
        }
        record.set_class()
    }

    // pub fn cross_validation(training_path: &str, n: usize, prefix: &str) {
    //     let mut precision = 0.0;

    //     for j in 1..n + 1 {
    //         let mut db = MpgDatabase::new();

    //         for i in 1..n + 1 {
    //             if i != j {
    //                 let path = format!("../../data/cross-validation/{}-{number:>0width$}",
    //                                    prefix,
    //                                    number = i,
    //                                    width = 2);
    //                 db.add_file(path.as_ref());
    //             }
    //         }

    //         db.standarize();
    //         let path = format!("../../data/cross-validation/{}-{number:>0width$}",
    //                            prefix,
    //                            number = j,
    //                            width = 2);
    //         let mut rdr = csv::Reader::from_file(path)
    //             .unwrap()
    //             .delimiter(b'\t')
    //             .has_headers(false);

    //         let mut n_correct = 0;
    //         let mut n_incorrect = 0;
    //         let mut count = 0;
    //         for record in rdr.decode() {
    //             let mut rcrd: (MpgRecord, String) = record.unwrap();
    //             let pred = db.predict(rcrd.0.cylinders,
    //                                   rcrd.0.ci,
    //                                   rcrd.0.hp,
    //                                   rcrd.0.weight,
    //                                   rcrd.0.secs);

    //             db.standarize_record(&mut rcrd.0);

    //             if rcrd.0.mpg == pred.mpg {
    //                 n_correct += 1;
    //             } else {
    //                 n_incorrect += 1;
    //             }
    //             count += 1;
    //         }

    //         println!("Correct: {}%\nIncorrect: {}%\n",
    //                  n_correct as f32 * 100.0 / count as f32,
    //                  n_incorrect as f32 * 100.0 / count as f32);

    //         presition += n_correct as f32 * 100.0 / count as f32;
    //     }

    //     presition /= n as f32;
    //     println!("Avg pres: {}%", presition);
    // }
}
