extern crate csv;
extern crate rustc_serialize;

use csv::Reader;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct PirsonRecord {
    class: u32,
    values: Vec<f64>,
}

fn main() {
    let mut rdr = csv::Reader::from_file("../../data/cross-validation/cbrilpirson.data").unwrap()/*.has_headers(true)*/;
    let mut vec: Vec<PirsonRecord> = Vec::new();

    while !rdr.done() {
        // Vector gen ********************************************
        let mut new_record = PirsonRecord {
            class: 0,
            values: vec![0.0; 1423]
        };

        loop {
            // Class *****************************************
            match rdr.next_bytes() {
                csv::NextField::Data(r) => {
                    let st = String::from_utf8(r.to_vec()).unwrap();
                    new_record.class = u32::from_str_radix(st.as_ref(), 10).unwrap();
                    print!("\n{}->", new_record.class);
                }
                csv::NextField::Error(err) => panic!(err),
                _ => break,
            }

            // Ratings *******************************************
            loop {
                match rdr.next_bytes() {
                    csv::NextField::EndOfCsv => break,
                    csv::NextField::EndOfRecord => break,
                    csv::NextField::Error(err) => panic!(err),
                    csv::NextField::Data(r) => {
                        let string = String::from_utf8(r.to_vec()).unwrap();
                        let rat: Vec<&str> = string.split_terminator(':').collect();
                        let pos = usize::from_str_radix(rat[0], 10).unwrap();
                        new_record.values[pos] = rat[1].parse::<f64>().unwrap();
                        print!("{}:{},", pos, new_record.values[pos]);
                    }
                }
            }
        }
        vec.push(new_record);
        println!("{}", vec.len());
    }

    let mut wtr = csv::Writer::from_file("../../data/cross-validation/pirson.data").unwrap();
    for record in vec {
        wtr.encode(record);
    }

    println!("Hello, world!");
}
