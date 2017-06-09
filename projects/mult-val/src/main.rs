extern crate database;

use database::generic::{Database, IrisRecord, MpgRecord};

fn main() {
    // MpgDatabase::test("../../data/mpgTrainingSet.txt", "../../data/mpgTestSet.txt", true);
    let mut db = Database::<IrisRecord<String>>::from_file("../../data/irisTrainingSet.data");
    db.standarize();
    let mut db = Database::<MpgRecord<u32>>::from_file("../../data/mpgTrainingSet.txt");
    db.standarize();
}
