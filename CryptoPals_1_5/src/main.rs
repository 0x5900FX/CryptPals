fn hex_to_byte(input: String) -> Vec<u8> {
    hex::decode(input).expect("invalid input")
}
fn byte_to_hex(input: &Vec<u8>)->String{
    hex::encode(&input)
}

fn repeating_xor(mut key:String, input:&[u8]) ->Vec<u8> {
    let input_len = input.len();
    let key_len = key.len();
    let mut output = Vec::new();



    for i in 0..input_len {
        let x = input[i] ^ key.as_bytes()[i%key_len];
        output.push(x);
    }
    output


}
fn main() {

    let key = "ICE".to_string();
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_string();

    let output = repeating_xor(key, input.as_bytes());
    let hex_out = byte_to_hex(&output);
    println!("{}", hex_out);


}
