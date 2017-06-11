extern crate database;

use database::generic::{Database, IrisRecord, MpgRecord};

fn main() {
    Database::<MpgRecord<u32>, u32>::cross_validation("../../data/mpgTrainingSet.txt", 10, "mpgData", false);
    // Database::<IrisRecord<String>, String>::cross_validation("../../data/irisTrainingSet.data", 10, "irisData", false);
    // Database::<PirsonRecord<u32>, u32>::cross_validation("../../data/cbrilpirson.data", 10, "pirsonData");
}
