
fn hex_to_byte(input: String) -> Vec<u8> {
    hex::decode(input).expect("invalid input")
}

fn hex_char_to_hex(input: char) -> u8{
 input as u8

}

fn xor_buffer(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter() //iterate over a
        .zip(b.iter()) // combine iterator of a with b returning a tuple| Create a pait of a and b elements
        .map(|(&a , &b)| // closure is defined here for each tuple which is (&a and &b)
             a ^ b //XOR value of a and b
        )
        .collect() //collect them into vec and returns them
}


fn xor_buffer_char(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter() //iterate over a
        .zip(b.iter()) // combine iterator of a with b returning a tuple| Create a pair of a and b elements
        .map(|(&a , &b)| // closure is defined here for each tuple which is (&a and &b)
             a ^ b //XOR value of a and b
        )
        .collect() //collect them into vec and returns them
}




fn scoring_value(input: &[u8], inputb: u8) ->Vec<u8>{
    let mut new_vec = Vec::new();
    for char in input.iter(){
new_vec.push(inputb);
    }
new_vec
}

fn main() {
let alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
let hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
let decoded_hex = hex_to_byte(hex_string);


    for word in alphabet.chars(){
        let word_hex = scoring_value(&decoded_hex, word as u8);
        let xor = xor_buffer(&decoded_hex, &word_hex);
        let decoded_xor = hex::encode(xor);
        println!("{}", decoded_xor);
        println!("{} \n ",word);

    }


}


