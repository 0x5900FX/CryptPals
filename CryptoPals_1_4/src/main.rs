use std::fs::File;

use std::io::{BufRead, BufReader};

fn hex_to_byte(input: String) -> Vec<u8> {
    hex::decode(input).expect("invalid input")
}

fn hex_char_to_hex(input: char) -> u8{
    input as u8

}

fn single_char_xor(input: &Vec<u8>, key: &u8) -> Vec<u8> {
    input.iter().map(|x| x^key).collect()
}


fn main() {
    let file = File::open("src/4.txt").unwrap();
    let  reader = BufReader::new(file);
    for line in reader.lines() {
        let input = line.unwrap();
        for val in 0x00..=0xff {
            let decoded_input = hex_to_byte(input.clone());
            let output = single_char_xor(&decoded_input, &val);


        }
    }
}
