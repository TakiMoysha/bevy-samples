use byteorder;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// use data::BSAReader;

fn get_workspace_dir() -> PathBuf {
    env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

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
    fn read_bsa_file() -> Result<(), Box<dyn std::error::Error>> {
        let test_bsa_file = get_workspace_dir().join("data/vicn_creature_pack.bsa");
        let data = read_file(test_bsa_file.as_path().to_str().unwrap());
        // let mut reader = BSAReader::new(data);
        Ok(())
    }
}
