use std::io::Read;

fn hex_decoder(input: String) -> Vec<u8> {
    println!("Your user input is {}", input);
    let decoded_hex =     hex::decode(input).unwrap();
    decoded_hex
}


fn main() {
let string1 = "1c0111001f010100061a024b53535009181c".as_bytes();

let string2 = "686974207468652062756c6c277320657965".as_bytes();

    let mut x: Vec<u8> = Vec::new();


}


    println!("{:?}", x.xor);
}
