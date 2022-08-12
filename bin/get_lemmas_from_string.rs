use std::env;

use rust_lemmatizer::get_words_from_string;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let string_to_analyze = &args[1];
    let dict_name = &args[2];
    get_words_from_string(string_to_analyze, dict_name);
}
