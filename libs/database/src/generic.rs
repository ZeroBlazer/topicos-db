extern crate quick_csv;
extern crate rustc_serialize;

use self::quick_csv::Csv;

#[derive(Debug, RustcDecodable)]
pub struct MpgRecord {
    class: u32,
    values: [f32; 5],
}

#[derive(Debug, RustcDecodable)]
pub struct IrisRecord {
    class: String,
    values: [f32; 4],
}

#[derive(Debug)]
pub struct Database<T> {
    data: Vec<T>,
}

impl<T> Database<T>
    where T: rustc_serialize::Decodable + ::std::fmt::Debug
{
    pub fn new() -> Database<T> {
        Database { data: Vec::new() }
    }
    
    pub fn from_file(path: &str) -> Database<T> {
        let mut rdr = Csv::from_file(path).unwrap().has_header(true);
        let mut data: Vec<T> = Vec::new();
        for row in rdr.into_iter() {
            match row.unwrap().decode::<T>() {
                Ok(cols) => data.push(cols),
                Err(error) => println!("{}", error),
            }
        }

        Database { data: data }
    }

    pub fn add_file(&mut self, path: &str) {
        let mut rdr = Csv::from_file(path).unwrap();
        for row in rdr.into_iter() {
            match row.unwrap().decode::<T>() {
                Ok(cols) => data.push(cols),
                Err(error) => println!("{}", error),
            }
        }
    }

    // pub fn standarize(&mut self) {
    //     println!("Standarizing DB...");
    //     for i in 0..T::len() {

    //     }
    // }

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
