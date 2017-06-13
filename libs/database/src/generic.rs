use rustc_serialize;
use quick_csv::Csv;
use csv;
use nn::{NN, HaltCondition};
use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fmt::Debug;
use rustc_serialize::{Decodable, Encodable};
use std::marker::PhantomData;
use std::hash::Hash;
use rand::{thread_rng, Rng};
use distance::*;
use utilities::abs_standard_deviation;

use rm::learning::svm::SVM;
use rm::learning::SupModel;
use rm::learning::toolkit::kernel::HyperTan;
use rm::linalg::Matrix;
use rm::linalg::Vector;

pub trait Record<U>
    where U: Clone + Eq + Debug + Hash
{
    fn record_len() -> usize;
    fn data_at(&self, index: usize) -> f32;
    fn standarize_field(&mut self, index: usize, asd_median: &(f32, f32));
    fn values(&self) -> Vec<f32>;
    fn values_f64(&self) -> Vec<f64>;
    fn set_values(&mut self, Vec<f32>);
    fn get_class(&self) -> U;
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct MpgRecord<U> {
    class: U,
    values: [f32; 5],
}

impl<U> Clone for MpgRecord<U>
    where U: Clone
{
    fn clone(&self) -> MpgRecord<U> {
        MpgRecord::<U> {
            class: self.class.clone(),
            values: self.values.clone(),
        }
    }
}

impl<U> Record<U> for MpgRecord<U>
    where U: Clone + Eq + Debug + Hash
{
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

    fn values_f64(&self) -> Vec<f64> {
        self.values
            .to_vec()
            .iter()
            .map(|x| *x as f64)
            .collect()
    }

    fn get_class(&self) -> U {
        self.class.clone()
    }

    fn set_values(&mut self, values: Vec<f32>) {
        for i in 0..self.values.len() {
            self.values[i] = values[i];
        }
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct IrisRecord<U> {
    class: U,
    values: [f32; 4],
}

impl<U> Clone for IrisRecord<U>
    where U: Clone
{
    fn clone(&self) -> IrisRecord<U> {
        IrisRecord::<U> {
            class: self.class.clone(),
            values: self.values.clone(),
        }
    }
}

impl<U> Record<U> for IrisRecord<U>
    where U: Clone + Eq + Debug + Hash
{
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

    fn values_f64(&self) -> Vec<f64> {
        self.values
            .to_vec()
            .iter()
            .map(|x| *x as f64)
            .collect()
    }

    fn get_class(&self) -> U {
        self.class.clone()
    }

    fn set_values(&mut self, values: Vec<f32>) {
        for i in 0..self.values.len() {
            self.values[i] = values[i];
        }
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct PirsonRecord<U> {
    class: U,
    values: Vec<f32>,
}

impl<U> Clone for PirsonRecord<U>
    where U: Clone
{
    fn clone(&self) -> PirsonRecord<U> {
        PirsonRecord::<U> {
            class: self.class.clone(),
            values: self.values.clone(),
        }
    }
}

impl<U> Record<U> for PirsonRecord<U>
    where U: Clone + Eq + Debug + Hash
{
    fn record_len() -> usize {
        1423
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

    fn values_f64(&self) -> Vec<f64> {
        self.values
            .to_vec()
            .iter()
            .map(|x| *x as f64)
            .collect()
    }

    fn get_class(&self) -> U {
        self.class.clone()
    }

    fn set_values(&mut self, values: Vec<f32>) {
        for i in 0..self.values.len() {
            self.values[i] = values[i];
        }
    }
}

#[derive(Debug)]
pub enum TrainingMethod {
    NearestNeighbors,
    NeuralNetwork,
    SVM,
}

use self::TrainingMethod::*;

#[derive(Debug)]
pub struct Database<T, U>
    where T: Record<U> + Clone + Encodable,
          U: Clone + Eq + Debug + Hash
{
    data: Vec<T>,
    abs_sd: Vec<(f32, f32)>,
    classifier: HashMap<U, usize>,
    net: NN,
    // svm_mod: SVM,
    phantom: PhantomData<U>,
}

impl<T, U> Database<T, U>
    where T: Decodable + Debug + Record<U> + Clone + Encodable,
          U: Clone + Eq + Debug + Hash
{
    pub fn new() -> Database<T, U> {
        Database {
            data: Vec::new(),
            abs_sd: Vec::new(),
            classifier: HashMap::new(),
            net: NN::new(&[T::record_len() as u32, 10, 9]),
            // svm_mod: SVM::default(),
            phantom: PhantomData,
        }
    }

    pub fn from_file(path: &str) -> Database<T, U> {
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
            classifier: HashMap::new(),
            net: NN::new(&[T::record_len() as u32, 10, 9]),
            // svm_mod: SVM::default(),
            phantom: PhantomData,
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
            println!("\t{}> asd: {}\tmedian: {}",
                     i,
                     asd_median_tup.0,
                     asd_median_tup.1);

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

    pub fn standarize_record(&self, record: &mut T) {
        for i in 0..T::record_len() {
            record.standarize_field(i, &self.abs_sd[i]);
        }
    }

    pub fn predict_knn(&self, record: &T, k: usize) -> U {
        let mut counts: HashMap<U, usize> = HashMap::new();
        for i in 0..k {
            let counter = counts
                .entry(self.data[self.nearest_neighbors(&record, pearson_coef)[i]].get_class())
                .or_insert(0);
            *counter += 1;
        }

        let mut p_class = self.data[self.nearest_neighbors(&record, pearson_coef)[0]].get_class();
        let mut gtr_count = 1;

        for (class, count) in &counts {
            if *count > gtr_count {
                p_class = class.clone();
            }
        }

        p_class
    }

    pub fn classify(&mut self) {
        let mut class = 0;
        for record in self.data.iter() {
            match self.classifier.entry(record.get_class()) {
                Vacant(entry) => {
                    entry.insert(class);
                    class += 1;
                }
                _ => {}
            }
        }
    }

    pub fn get_class_index(&self, record: &T) -> usize {
        *self.classifier.get(&record.get_class()).unwrap()
    }

    pub fn get_class_from_index(&self, class_val: usize) -> U {
        for (class, val) in &self.classifier {
            if class_val == *val {
                return class.clone();
            }
        }
        self.data[0].get_class().clone()
    }

    pub fn predict_nn(&mut self, record: &T) -> U {
        let pred = self.net.run(&record.values_f64());
        println!("{:?}", pred);
        let mut indx = 0;

        for i in 1..pred.len() {
            if pred[indx] < pred[i] {
                indx = i;
            }
        }
        self.get_class_from_index(indx)
    }

    pub fn predict_svm(&self, record: &T) -> U {
        unimplemented!()
    }

    pub fn count_classes(&self) -> HashMap<U, usize> {
        let mut counts: HashMap<U, usize> = HashMap::new();
        for record in self.data.iter() {
            let counter = counts.entry(record.get_class()).or_insert(0);
            *counter += 1;
        }
        counts
    }

    pub fn segment(&self, n: usize, prefix: &str) {
        let mut record_transfers: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut rng = thread_rng();
        let mut i = 0;
        for record in self.data.iter() {
            record_transfers[rng.gen_range(0, n)].push(i);
            i += 1;
        }

        i = 1;
        for item in record_transfers.iter() {
            let path = format!("../../data/cross-validation/{}-{number:>0width$}",
                               prefix,
                               number = i,
                               width = 2);
            let str_path: &str = path.as_ref();
            let mut wtr = csv::Writer::from_file(str_path).unwrap();
            for record in item {
                wtr.encode(self.data[*record].clone());
            }
            i += 1;
        }
    }

    pub fn cross_validation(training_path: &str,
                            n: usize,
                            prefix: &str,
                            segment: bool,
                            training: TrainingMethod) {
        /*******SEGMENTATION******/
        if segment {
            let mut db = Database::<T, U>::from_file(training_path);
            db.segment(n, prefix);
        }
        /*************************/
        let mut precision = 0.0;
        let mut prec_vec = Vec::new();
        for j in 1..n + 1 {
            let mut db = Database::<T, U>::new();
            for i in 1..n + 1 {
                if i != j {
                    db.add_file(format!("../../data/cross-validation/{}-{number:>0width$}",
                                        prefix,
                                        number = i,
                                        width = 2)
                                        .as_ref());
                }
            }
            db.standarize();

            let path = format!("../../data/cross-validation/{}-{number:>0width$}",
                               prefix,
                               number = j,
                               width = 2);
            let mut test_db = Database::<T, U>::from_file(path.as_ref());
            let mut confusion_counts: HashMap<U, HashMap<U, usize>> = HashMap::new();

            let mut n_correct = 0;
            let mut n_incorrect = 0;
            let mut count = 0;

            if let NeuralNetwork = training {
                db.classify();
                let mut values: Vec<(Vec<f64>, Vec<f64>)> = Vec::new();
                for rcrd in db.data.iter() {
                    let mut class = vec![0.0; 9];
                    // println!("Here!: {}", db.get_class_index(rcrd));
                    class[db.get_class_index(rcrd)] = 1.0;
                    // println!("{} vs {}", rcrd.values_f64().len(), T::record_len());
                    values.push((rcrd.values_f64(), class));
                }
                // println!("Exited");

                db.net
                    .train(&values)
                    .halt_condition(HaltCondition::Epochs(100000))
                    .log_interval(Some(1000))
                    .momentum(0.1)
                    .rate(0.3)
                    .go();
            }

            if let SVM = training {
                // let inputs = Matrix::new(T::record_len(), db.data.len(), vec![1.0,3.0,5.0,7.0]);
                // let targets = Vector::new(vec![-1.,-1.,1.,1.]);

                // // Train the model
                // self.svm_mod.train(&inputs, &targets).unwrap();
            }

            for mut record in test_db.data.iter_mut() {
                db.standarize_record(&mut record);
                let class = record.get_class();
                let pred = match training {
                    NearestNeighbors => db.predict_knn(&record, 3),
                    NeuralNetwork => db.predict_nn(&record),
                    SVM => db.predict_knn(&record, 3),
                };

                if class == pred {
                    n_correct += 1;
                } else {
                    n_incorrect += 1;
                }

                match confusion_counts.entry(class) {
                    Vacant(entry) => {
                        let mut class_count = HashMap::new();
                        class_count.insert(pred, 1);
                        entry.insert(class_count);
                    }
                    Occupied(mut entry) => {
                        let counter = entry.get_mut().entry(pred).or_insert(1);
                        *counter += 1;
                    }
                }

                count += 1;
            }

            

            println!("\nTestings for: {}\n\
                      Accuracy: {}%\n\
                      Confusion Matrix===========",
                     path,
                     n_correct as f32 * 100.0 / count as f32);
            for (act_class, counts) in &confusion_counts {
                print!("  {:?} >", act_class);
                for (pred_class, count) in counts {
                    print!("\t{:?}: {}", pred_class, count);
                }
                println!("");
            }
            println!("===========================\n");
            precision += n_correct as f32 * 100.0 / count as f32;
            prec_vec.push(n_correct as f32 * 100.0 / count as f32);
        }
        precision /= n as f32;
        println!("{:?}", prec_vec);
        println!("Avg accuracy: {}%", precision);
    }
}
