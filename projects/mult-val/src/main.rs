extern crate database;

use database::generic::{Database, IrisRecord, MpgRecord};

fn main() {
    Database::<MpgRecord<u32>, u32>::cross_validation("../../data/mpgTrainingSet.txt", 10, "mpgData");
}
