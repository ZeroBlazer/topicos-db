extern crate csv;
extern crate rustc_serialize;

mod distance;

use std::fs::File;
use csv::Reader;
use distance::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

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

fn load_book_ratings() {
    /*********************LOAD USERS*********************/
    // let mut rdr = init_db("./data/BX-Dump/BX-Users.csv").has_headers(false);

    // let mut i = 0;
    // for record in rdr.decode() {
    //     if i > 10 {
    //         break;
    //     }
    //     let user: User = record.unwrap();
    //     println!("{}: {} [{}]", user.id, user.location, user.age);

    //     i += 1;
    // }
    /****************************************************/

    let mut rdr = init_db("./data/BX-Dump/BX-Book-Ratings.csv").has_headers(false);
    let mut ratings: HashMap<u32, HashMap<String, u32>> = HashMap::new();

    for record in rdr.decode() {
        let mut rating: Rating = record.unwrap();

        match ratings.entry(rating.user_id) {
            Vacant(entry) => {
                let mut user_ratings = HashMap::new();
                user_ratings.insert(rating.book_isbn, rating.rating);
                entry.insert(user_ratings);
            }
            Occupied(entry) => {
                entry.into_mut().insert(rating.book_isbn, rating.rating);
            }
        }
    }
}

fn main() {
    load_book_ratings();
    println!("Hello, world!");
}
