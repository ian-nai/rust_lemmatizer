use std::env;

use rust_lemmatizer::get_words;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let dict_name = &args[2];
    let file_output = &args[3];
    get_words(filename, dict_name, file_output);
}

