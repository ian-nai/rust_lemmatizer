use std::env;

use rust_lemmatizer::string_or_file;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let dict_name = &args[2];
    let file_output = &args[3];
    let input_format = &args[4];
    string_or_file(filename, dict_name, file_output, input_format);
}
