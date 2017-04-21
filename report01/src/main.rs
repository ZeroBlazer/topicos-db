extern crate csv;
extern crate rustc_serialize;

mod distance;

use std::fs::File;
use csv::Reader;
use distance::*;

#[derive(RustcDecodable)]
struct User {
    id: u32,
    location: String,
    age: u32,
}

#[derive(RustcDecodable)]
struct Book {
    isbn: String,
    title: String,
    author: String,
    pub_year: u32,
    publisher: String,
}

#[derive(RustcDecodable)]
struct Rating {
    user_id: u32,
    book_isbn: String,
    rating: u32,
}

fn init_db(path: &str) -> Reader<File> {
    let mut rdr = csv::Reader::from_file(path).unwrap();
    rdr
}

// fn get_vector(db: &Value, name: &str) -> Vec<f32> {
//     let mut vec: Vec<f32> = vec![];
//     for elem in db["scores"][name].as_array().unwrap().iter() {
//         vec.push(elem.as_f64().unwrap() as f32);
//     }

//     vec
// }

// fn evaluate_movie_ratings() {
//     let mut rdr = init_db("./data/Movie_Ratings.csv");
// }

fn evaluate_book_ratings() {
    // let mut rdr = init_db("./data/BX-Dump/BX-Book-Ratings.csv");
    let mut rdr = init_db("./data/BX-Dump/BX-Users.csv").has_headers(false);

    let mut i = 0;
    for record in rdr.decode() {
        if i > 10 {
            break;
        }
        let user: User = record.unwrap();
        println!("{}: {} [{}]", user.id, user.location, user.age);
        // let (s1, s2, dist): (u32, String, u32) = record.unwrap();
        // println!("{}: {}: [{}]", s1, s2, dist);
        i += 1;
    }
}

fn main() {
    evaluate_book_ratings();
    println!("Hello, world!");
}
