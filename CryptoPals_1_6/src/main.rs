use std::f32;
use std::fs::File;
use std::io::{BufRead, BufReader};
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

/// A custom base64 engine to encode and decode
const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

fn main() {
let inp1 = "this is a test".as_bytes();
let inp2 = "wokka wokka!!!".as_bytes();
let key = hamming_distance(inp1, inp2);
    println!("key :: {}", key );
}


fn hamming_distance(a: &[u8], b: &[u8]) ->i32{
    if a.len() != b.len() {
       panic!("Strings must be of equal length");
    }
    let mut distance = 0;

    for (i,j) in a.iter().zip(b.iter()) {
            distance += (i ^ j).count_ones() as i32;
    }
    distance
 }

fn guess_key(text: &[u8] , min_keysize: usize, max_keysize: usize) -> u8 {
    let mut BESTKEY = min_keysize;
    let mut BEST_DIS = f32::MAX;

for key in min_keysize..=max_keysize{

}


}