use std::collections::HashMap;
use std::os::unix::raw::pid_t;

fn hex_to_byte(input: String) -> Vec<u8> {
    hex::decode(input).expect("invalid input")
}

fn hex_char_to_hex(input: char) -> u8{
 input as u8

}

fn single_char_xor(input: &Vec<u8>, key: &u8) -> Vec<u8> {
    input.iter().map(|x| x^key).collect()
}


    fn scoring(encoded_str: &str, word_combo: &HashMap<char, f64>) ->f64{
    let mut score = 0.0;
        for c in encoded_str.chars() {
            if let Some(&ch) = word_combo.get(&c) {
                score += ch;
            }

        }
        score
    }

fn key_frequency()->HashMap<char,f64>{
let mut map: HashMap<char, f64> = HashMap::new();
    map.insert('A', 8.167);
    map.insert('B', 1.492);
    map.insert('C', 2.782);
    map.insert('D', 4.253);
    map.insert('E', 12.702);
    map.insert('F', 2.228);
    map.insert('G', 2.015);
    map.insert('H', 6.094);
    map.insert('I', 6.966);
    map.insert('J', 0.153);
    map.insert('K', 0.772);
    map.insert('L', 4.025);
    map.insert('M', 2.406);
    map.insert('N', 6.749);
    map.insert('O', 7.507);
    map.insert('P', 1.929);
    map.insert('Q', 0.095);
    map.insert('R', 5.987);
    map.insert('S', 6.327);
    map.insert('T', 9.056);
    map.insert('U', 2.758);
    map.insert('V', 0.978);
    map.insert('W', 2.360);
    map.insert('X', 0.150);
    map.insert('Y', 1.974);
    map.insert('Z', 0.074);
    map.insert(' ', 0.000);
    map.insert('.', 0.000);
    map.insert(',', 0.000);
    map.insert(';', 0.000);
    map.insert(':', 0.000);
    map.insert('\'', 0.000);
    map.insert('\n', 0.000);
    map.insert('_', -10.000);
map
}



fn main() {
    let hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
    let decoded_hex = hex_to_byte(hex_string);
    let key_maps = key_frequency();
    let mut best_score = 0.0;
    let mut best_key = 0;
    let mut best_decryption = Vec::new();

    for key in 0x00..=0xFF{
        let dec_val_key = single_char_xor(&decoded_hex, &key);
        if let Ok(decoded_str) = std::str::from_utf8 (&dec_val_key){
            let score = scoring(decoded_str,&key_maps);
            if score > best_score {
                best_score = score;
                best_key = key;
                best_decryption = dec_val_key;
            }

        }
    }

println!("Key is 0x{:02x}",best_key);
println!("Decrypted msg is:: {}",std::str::from_utf8(&best_decryption).unwrap());



}

