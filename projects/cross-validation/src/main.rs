extern crate distance;
extern crate database;

use database::AthlDatabase;

fn main() {
    println!("Loading database, please wait...");
    let mut db = AthlDatabase::from_file("../../data/athletesTrainingSet.txt");
    db.standarize();    // println!("{:?}", db);
    println!("Database ready!\n---------------------------------------------");

    println!("Pred => {:?}", db.predict(70.0, 170.0));
}
