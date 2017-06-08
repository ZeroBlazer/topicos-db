extern crate quick_csv;
extern crate rustc_serialize;

use self::quick_csv::Csv;

#[derive(Debug, RustcDecodable)]
pub struct MpgRecord {
    class: u32,
    values: (f32, f32, f32, f32, f32),
}

#[derive(Debug, RustcDecodable)]
pub struct IrisRecord {
    values: (f32, f32, f32, f32),
    class: String,
}

#[derive(Debug)]
pub struct Database<T> {
    data: Vec<T>,
}

impl<T> Database<T>
    where T: rustc_serialize::Decodable + ::std::fmt::Debug
{
    pub fn new(path: &str) -> Database<T> {
        let mut rdr = Csv::from_file(path).unwrap().delimiter(b'\t');
        // let first = rdr.unwrap()
        // println!("{}, {:?}", rdr.current_line(), rdr.headers());
        let mut data: Vec<T> = Vec::new();
        for row in rdr.into_iter() {
            if let Ok(cols) = row.unwrap().decode::<T>() {
                data.push(cols);
            } else {
                println!("Couldn't read line");
            }
        }

        Database { data: data }
    }
}
