extern crate database;

use database::generic::{Database, IrisRecord, MpgRecord};

fn main() {
    // MpgDatabase::test("../../data/mpgTrainingSet.txt", "../../data/mpgTestSet.txt", true);
    let mut db = Database::<IrisRecord>::from_file("../../data/irisTrainingSet.data");
    db.standarize();
    let mut db = Database::<MpgRecord>::from_file("../../data/mpgTrainingSet.txt");
    db.standarize();
}
