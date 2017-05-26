extern crate distance;
extern crate database;

use database::AthlDatabase;

fn main() {
    println!("Loading database, please wait...");
    let mut db = AthlDatabase::load("../../data/athletesTrainingSet.txt");
    db.standarize();
    // println!("{:?}", db);
    println!("Database ready!\n---------------------------------------------");
}
