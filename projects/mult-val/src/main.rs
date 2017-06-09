extern crate database;

use database::generic::{Database, IrisRecord, MpgRecord};

fn main() {
    // let mut db = Database::<IrisRecord<String>, String>::from_file("../../data/irisTrainingSet.data");
    // db.standarize();
    // let mut db = Database::<MpgRecord<u32>, u32>::from_file("../../data/mpgTrainingSet.txt");
    // db.standarize();
    Database::<MpgRecord<u32>, u32>::cross_validation("../../data/mpgTrainingSet.txt", 10, "mpgData");
}
