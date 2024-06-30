fn hex_to_byte(input: String) -> Vec<u8> {
    hex::decode(input).expect("invalid input")
}

fn xor_buffer(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter() //iterate over a
        .zip(b.iter()) // combine iterator of a with b returning a tuple| Create a pait of a and b elements
        .map(|(&a , &b)| // closure is defined here for each tuple which is (&a and &b)
             a ^ b //XOR value of a and b
        )
        .collect() //collect them into vec and returns them
}


fn main() {
let alphabet = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
let hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();

    let decoded = hex_to_byte(hex_string);

    for word in alphabet.chars(){
let char_byte = hex_to_byte(word.to_string());

    }

}
