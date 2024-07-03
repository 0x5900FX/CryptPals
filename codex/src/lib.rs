pub mod level1 {

    pub fn  hex_to_byte(input: String) -> Vec<u8> {
        hex::decode(input).expect("invalid input")
    }









}