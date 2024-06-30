

//Decoded hex_string into Vec<u8> and return it while asking for user_inp
//Changes hex to byte or decoded hex to byte
fn hex_to_byte(input: String) -> Vec<u8> {
    hex::decode(input).expect("invalid input")
}
//convert vec<u8> into String and return it
/*
fn byte_to_hex(input: &Vec<u8>)->String{
    hex::encode(&input)
}
 */

fn xor_buffer(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter() //iterate over a
        .zip(b.iter()) // combine iterator of a with b returning a tuple| Create a pait of a and b elements
        .map(|(&a , &b)| // closure is defined here for each tuple which is (&a and &b)
       a ^ b //XOR value of a and b
        )
        .collect() //collect them into vec and returns them
}

fn main() {

    let inp1 = "1c0111001f010100061a024b53535009181c".to_string();
    let inp2 = "686974207468652062756c6c277320657965".to_string();



    let out_buff = hex::encode(xor_buffer(&hex_to_byte(inp1), &hex_to_byte(inp2)));


    let exp_buff = "746865206b696420646f6e277420706c6179".to_string();




    assert_eq!(exp_buff, out_buff); //ensuring output is correct


}
