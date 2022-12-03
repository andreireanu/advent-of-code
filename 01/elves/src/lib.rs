use std::{path::Path, fs::read_to_string};

pub fn read_records<T: AsRef<Path>>(pathname: T)  {
    let rez: String = read_to_string(pathname)
        .expect("Unable to open file")
        .split("\n\n")
        .collect();
    println!("{:?}", rez);
}

// -> Vec<Vec<String>>