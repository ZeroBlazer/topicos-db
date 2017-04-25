extern crate csv;
extern crate rustc_serialize;
extern crate time;

mod distance;

use std::fs::File;
use csv::Reader;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use distance::*;
use time::PreciseTime;

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

fn distance(db: &mut HashMap<u32, HashMap<String, u32>>,
            a: u32,
            b: u32,
            func: fn(&Vec<f32>, &Vec<f32>) -> f32)
            -> f32 {
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();

    match db.get(&a) {
        Some(mut a_ratings) => {
            for (isbn_a, &rating_a) in a_ratings.iter() {
                // print!("{} ", rating_a);
                if let Some(b_ratings) = db.get(&b) {
                    if let Some(rating_b) = b_ratings.get(isbn_a) {
                        // print!("{} ", val_b);
                        vec_a.push(rating_a as f32);
                        vec_b.push(*rating_b as f32);
                    } else {
                        vec_a.push(rating_a as f32);
                        vec_b.push(0.0);
                    }
                } else {
                    panic!("b is not found!");
                }
            }
        }
        None => {}
    }

    match db.get(&b) {
        Some(mut b_ratings) => {
            for (isbn_b, &rating_b) in b_ratings.iter() {
                if let Some(a_ratings) = db.get(&a) {
                    if let Some(_) = a_ratings.get(isbn_b) {
                    } else {
                        vec_b.push(rating_b as f32);
                        vec_a.push(0.0);
                    }
                } else {
                    panic!("a is not found!");
                }
            }
        }
        None => {}
    }

    func(&vec_a, &vec_b)
}

fn init_db_reader(path: &str) -> Reader<File> {
    let rdr = csv::Reader::from_file(path).unwrap();
    rdr
}

fn book_ratings_distance() {
    /*********************LOAD RATINGS*********************/
    let mut rdr = init_db_reader("./data/BX-Dump/BX-Book-Ratings.csv").has_headers(false);
    let mut ratings: HashMap<u32, HashMap<String, u32>> = HashMap::new();

    // let mut i = 0;
    for record in rdr.decode() {
        // if i > 54000 {
        //     break;
        // }
        // i += 1;
        let rating: Rating = record.unwrap();

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

    let mut start = PreciseTime::now();
    let mut measure = distance(&mut ratings, 11676, 278418, manhattan_dist);
    let mut end = PreciseTime::now();
    println!("{} seconds for Manhattan_dist: {}", start.to(end), measure);
    start = PreciseTime::now();
    measure = distance(&mut ratings, 11676, 278418, euclidian_dist);
    end = PreciseTime::now();
    println!("{} seconds for Euclidian_dist: {}", start.to(end), measure);
    start = PreciseTime::now();
    measure = distance(&mut ratings, 11676, 278418, cosine_dist);
    end = PreciseTime::now();
    println!("{} seconds for Cosine_dist: {}", start.to(end), measure);
    start = PreciseTime::now();
    measure = distance(&mut ratings, 11676, 278418, pearson_coef);
    end = PreciseTime::now();
    println!("{} seconds for Pearson_dist: {}", start.to(end), measure);

    /*********************LOAD USERS*********************/
    /****************************************************/
}

fn movie_ratings_distance(a: &str, b: &str) {
    /*********************LOAD RATINGS*********************/
    let mut rdr = init_db_reader("./data/Movie_Ratings_transposed.csv").has_headers(true);

    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    let mut catch_a = false;
    let mut catch_b = false;

    while !rdr.done() {
        // skip headers ******************************************
        loop {
            match rdr.next_bytes() {
                csv::NextField::EndOfCsv => break,
                csv::NextField::EndOfRecord => break,
                csv::NextField::Error(err) => panic!(err),
                csv::NextField::Data(_) => {}
            }
        }

        // Vector gen ********************************************
        loop {
            // User name *****************************************
            match rdr.next_bytes() {
                csv::NextField::EndOfCsv => break,
                csv::NextField::EndOfRecord => break,
                csv::NextField::Error(err) => panic!(err),
                csv::NextField::Data(r) => {
                    let st = String::from_utf8(r.to_vec()).unwrap();
                    if st == a.to_string() {
                        catch_a = true;
                    } else if st == b.to_string() {
                        catch_b = true;
                    }
                }
            }

            // Ratings *******************************************
            loop {
                match rdr.next_bytes() {
                    csv::NextField::EndOfCsv => break,
                    csv::NextField::EndOfRecord => break,
                    csv::NextField::Error(err) => panic!(err),
                    csv::NextField::Data(r) => {
                        if catch_a {
                            vec_a.push(String::from_utf8(r.to_vec())
                                           .unwrap()
                                           .parse::<f32>()
                                           .unwrap());
                        } else if catch_b {
                            vec_b.push(String::from_utf8(r.to_vec())
                                           .unwrap()
                                           .parse::<f32>()
                                           .unwrap());
                        }
                        // let val = String::from_utf8(r.to_vec()).unwrap();
                        // let val = String::from_utf8(r.to_vec())
                        //     .unwrap()
                        //     .parse::<f32>()
                        //     .unwrap();
                        // print!("{} - ", val);
                    }
                }
            }
            catch_a = false;
            catch_b = false;
        }

        // while let Some(r) = rdr.next_bytes().into_iter_result() {
        //     let st = String::from_utf8(r.unwrap().to_vec()).unwrap();
        //     print!("{} - ", st);
        // }
    }

    // println!("Manhattan_dist: {}", manhattan_dist(&vec_a, &vec_b));
    // println!("Euclidian_dist: {}", euclidian_dist(&vec_a, &vec_b));
    // println!("Cosine_dist: {}", cosine_dist(&vec_a, &vec_b));
    // println!("Pearson_dist: {}", pearson_coef(&vec_a, &vec_b));

    let mut start = PreciseTime::now();
    let mut measure = manhattan_dist(&vec_a, &vec_b);
    let mut end = PreciseTime::now();
    println!("{} seconds for Manhattan_dist: {}", start.to(end), measure);
    start = PreciseTime::now();
    measure = euclidian_dist(&vec_a, &vec_b);
    end = PreciseTime::now();
    println!("{} seconds for Euclidian_dist: {}", start.to(end), measure);
    start = PreciseTime::now();
    measure = cosine_dist(&vec_a, &vec_b);
    end = PreciseTime::now();
    println!("{} seconds for Cosine_dist: {}", start.to(end), measure);
    start = PreciseTime::now();
    measure = pearson_coef(&vec_a, &vec_b);
    end = PreciseTime::now();
    println!("{} seconds for Pearson_dist: {}", start.to(end), measure);
}

fn main() {
    // book_ratings_distance();
    movie_ratings_distance("Katherine", "Erin");
    println!("Hello, world!");
}
