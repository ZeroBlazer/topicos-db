extern crate database;

use database::generic::{Database, IrisRecord, MpgRecord};

fn main() {
    // MpgDatabase::test("../../data/mpgTrainingSet.txt", "../../data/mpgTestSet.txt", true);
    let db = Database::<IrisRecord>::new("../../data/irisTrainingSet.data");
    // let db = Database::<MpgRecord>::new("../../data/mpgTrainingSet.txt");
}
