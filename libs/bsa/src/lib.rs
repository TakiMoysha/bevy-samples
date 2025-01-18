use byteorder;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};

// use data::BSAReader;

fn read_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let path = env::current_dir()
            .unwrap()
            .join("data/VicnCreaturePack/VicnCreaturePack.bsa");
        let data = read_file(path.as_path().to_str().unwrap());
        // let mut reader = BSAReader::new(data);
    }
}
