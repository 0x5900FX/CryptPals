use std::fs::File;
use std::io::{BufRead, BufReader};
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

/// A custom base64 engine to encode and decode
const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

fn main() {

}


fn hamming_distance(a: &String, b: &String){

}