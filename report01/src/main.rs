extern crate csv;
extern crate rustc_serialize;

mod distance;

use std::fs::File;
use csv::Reader;
use distance::*;
use std::collections::HashMap;

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

    let mut i = 0;
    let mut ratings: HashMap<u32, HashMap<String, u32>> = HashMap::new();

    for record in rdr.decode() {
        if i > 100 {
            break;
        }
        i += 1;

        let mut rating: Rating = record.unwrap();
        let mut found = false;
        let mut new_user_ratings = HashMap::new();

        match ratings.get_mut(&rating.user_id) {
            Some(mut usr_rtngs) => {
                &usr_rtngs.insert(rating.book_isbn, rating.rating);
                found = true;
            }
            None => {
                new_user_ratings.insert(rating.book_isbn, rating.rating);
            }
        }
        if !found {
            ratings.insert(rating.user_id, new_user_ratings);
        }
    }

    i = 0;

    for record in rdr.decode() {
        if i > 100 {
            break;
        }
        i += 1;

        let mut rating: Rating = record.unwrap();

        print!("{}: ", rating.user_id);

        // let isbn = String::from("0100000X");
        // let rate = 7;

        match ratings.get_mut(&rating.user_id) {
            Some(mut usr_rtngs) => {
                // let variable: () = usr_rtngs;
                // &usr_rtngs.insert(isbn, rate);
                for (book_name, &rating_num) in usr_rtngs.iter() {
                    println!("{} -> {}", book_name, rating_num);
                }
                // usr_rtngs.insert(String::from("AAAAAAA"), 9);
                // println!("Found");
            }
            None => {
                println!("Not found");
            }
        }
    }
}

fn main() {
    load_book_ratings();
    println!("Hello, world!");
}
