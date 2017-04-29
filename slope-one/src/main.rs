use std::collections::HashMap;

struct IndexedDB(Vec<f32>, HashMap<String, usize>, HashMap<String, usize>);

fn load_database(path: &str) -> IndexedDB {
    let mut rdr = csv::Reader::from_file(path).unwrap().has_headers(true);
    let mut headers: HashMap<String, u32> = HashMap::new();
    let mut names: HashMap<String, u32> = HashMap::new();
    let mut records: Vec<f32> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    loop {
        match rdr.next_bytes() {
            csv::NextField::Data(data) => {
                let d_string = String::from_utf8(data.to_vec()).unwrap();
                match j {
                    0 => {
                        /***********HEADERS***********/
                        if i > 0 {
                            headers.insert(d_string, i - 1);
                        }
                    }
                    _ => {
                        /***********RECORDS***********/
                        match i {
                            0 => {
                                /***********NAMES***********/
                                names.insert(d_string, j - 1);
                            }
                            _ => {
                                /***********VALUES***********/
                                records.push(d_string.parse::<f32>().unwrap());
                            }
                        }
                    }
                }
                i += 1;
            }
            csv::NextField::EndOfRecord => {
                j += 1;
                i = 0;
            }
            csv::NextField::EndOfCsv => break,
            csv::NextField::Error(err) => panic!(err),
        }
    }

    IndexedDB(records, headers, names)
}

fn main() {
    println!("Hello, world!");
}
