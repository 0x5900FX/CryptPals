use std::io;
//dependencies
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

/// A custom base64 engine to encode and decode
const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

//Convert hexadecimal to base64

fn hex_to_base64(input: String) -> String {
    println!("Your user input is {}", input);

    let decoded_hex =     hex::decode(input).unwrap();
    CUSTOM_ENGINE.encode(decoded_hex)
}


fn main() {

    println!("Enter your string \n You're converting from hex to base64 \n");
    let mut user_inp = String::new();
    io::stdin().read_line(&mut user_inp).expect("Failed to read line");

    let output = hex_to_base64(user_inp.trim().to_string());  //removes newlines and extra spaces
    let final_out  = format!("{}",output);
    println!("Your HEx to Base64 is  \n{}",final_out);

}