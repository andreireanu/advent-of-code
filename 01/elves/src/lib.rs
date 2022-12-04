use std::{path::Path, fs::read_to_string, str::FromStr};
use std::fmt::Debug;

pub fn read_records<T: AsRef<Path>, U: FromStr>(pathname: T)  -> Vec<Vec<U>>
where 
    <U as FromStr>::Err: Debug,
{
    let read_path   =  read_to_string(pathname).expect("Can't open");
    let rez: Vec<Vec<U>> = read_path
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| 
            { 
                s.split('\n')
                    .map(|num| num.parse::<U>().expect("Cannot parse"))
                    .collect::<Vec<U>>()
            })
        .collect();
    rez
}

 