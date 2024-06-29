//dependencies
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use hex::FromHexError;

/// A custom base64 engine to encode and decode
const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

//Convert hexadecimal to plain_text
fn hex_to_plain(input: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(input)
}
//convert plain_to_base64
fn plain_to_base64(input: Vec<u8>) -> String {
    CUSTOM_ENGINE.encode(input)
}

fn main() {
    let inp = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
let output = format!("{}",plain_to_base64(hex_to_plain(inp).unwrap()));
println!("{}",output);







}
