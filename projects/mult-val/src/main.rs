extern crate database;

use database::generic::{Database, IrisRecord, MpgRecord, PirsonRecord, TrainingMethod};

fn main() {
    // Database::<MpgRecord<u32>, u32>::cross_validation("../../data/mpgTrainingSet.txt", 10, "mpgData", false, TrainingMethod::NearestNeighbors);
    // Database::<IrisRecord<String>, String>::cross_validation("../../data/irisTrainingSet.data", 10, "irisData", false, TrainingMethod::NearestNeighbors);
    // Database::<PirsonRecord<u32>, u32>::cross_validation("../../data/pirson.csv", 10, "pirsonData", false, TrainingMethod::NearestNeighbors);
    // Database::<MpgRecord<u32>, u32>::cross_validation("../../data/mpgTrainingSet.txt", 10, "mpgData", true, TrainingMethod::NeuralNetwork);
    // Database::<IrisRecord<String>, String>::cross_validation("../../data/irisTrainingSet.data", 10, "irisData", true, TrainingMethod::NeuralNetwork);
    Database::<PirsonRecord<u32>, u32>::cross_validation("../../data/pirson.csv", 10, "pirsonData", false, TrainingMethod::NeuralNetwork);
}
