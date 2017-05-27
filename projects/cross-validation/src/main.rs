extern crate distance;
extern crate database;

use database::{AthlDatabase, MpgDatabase};

fn main() {
    /******************** Athletes DB ********************/
    // println!("Loading database, please wait...");
    // let mut db = AthlDatabase::from_file("../../data/athletesTrainingSet.txt");
    // db.standarize();    // println!("{:?}", db);
    // println!("Database ready!\n---------------------------------------------");

    // println!("Pred => {:?}", db.predict(70.0, 170.0));

    /******************** Miles per Gallon DB ********************/
    println!("Loading database, please wait...");
    let mut db = MpgDatabase::from_file("../../data/mpgTrainingSet.txt");
    db.standarize();    println!("{:?}", db);
    println!("Database ready!\n---------------------------------------------");

    // println!("Pred => {:?}", db.predict(8.0, 360.0, 215.0, 4615.0, 14.0));
}
