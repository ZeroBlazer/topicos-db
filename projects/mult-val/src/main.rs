extern crate database;

use database::generic::{Database, IrisRecord, MpgRecord, TrainingMethod};

fn main() {
    Database::<MpgRecord<u32>, u32>::cross_validation("../../data/mpgTrainingSet.txt", 10, "mpgData", true, TrainingMethod::NearestNeighbors);
    Database::<IrisRecord<String>, String>::cross_validation("../../data/irisTrainingSet.data", 10, "irisData", true, TrainingMethod::NearestNeighbors);
    Database::<MpgRecord<u32>, u32>::cross_validation("../../data/mpgTrainingSet.txt", 10, "mpgData", false, TrainingMethod::NeuralNetwork);
    Database::<IrisRecord<String>, String>::cross_validation("../../data/irisTrainingSet.data", 10, "irisData", false, TrainingMethod::NeuralNetwork);
    // Database::<PirsonRecord<u32>, u32>::cross_validation("../../data/cbrilpirson.data", 10, "pirsonData");
}
