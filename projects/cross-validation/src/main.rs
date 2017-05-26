extern crate distance;
extern crate database;

use database::AthlDatabase;

fn main() {
    println!("Loading database, please wait...");
    let mut db = AthlDatabase::load("../../data/athletesTrainingSet.txt");
    // println!("{:?}", db);
    db.standarize();
    println!("Database ready!\n---------------------------------------------");
}
