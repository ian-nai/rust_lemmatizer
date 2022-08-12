extern crate csv;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn string_or_file(filename: &str, dict_name: &str, file_output: &str, input_format: &str) {
    if input_format == "file" {
        get_words(filename, dict_name, file_output);
    } else {
        get_words_from_string(filename, dict_name);
    }
}

pub fn get_words_from_string(filename: &str, dict_name: &str) {
    let mut word_list: Vec<String> = Vec::new();
    for split_word in filename.split(&[' ', '\''][..]) {
        word_list.push(split_word.to_string());
    }
    get_lemma_from_string(word_list, dict_name);
}

pub fn get_lemma_from_string(word_list: Vec<String>, dict_name: &str) {
    let dict_lines = vec![dict_name];

    let mut lemma_map: HashMap<String, String> = HashMap::new();
    for dline in dict_lines {
        let mut rdr = csv::Reader::from_path(dline).unwrap();

        for rec in rdr.records() {
            let rr = rec.unwrap();
            let lemma = rr.get(0).unwrap();
            let derivatives = rr.get(1).unwrap();
            lemma_map.insert(lemma.into(), derivatives.into());
        }
    }
    process_data_from_string(word_list, lemma_map);
}

pub fn process_data_from_string(word_list: Vec<String>, lemma_map: HashMap<String, String>) {
    let mut lemma_string: Vec<String> = Vec::new();

    for line in &word_list {
        let mut lemma_line: Vec<String> = Vec::new();
        let mut changed_words: Vec<String> = Vec::new();
        for word in line.split_whitespace() {
            for (lemma, derivatives) in &lemma_map {
                if derivatives.contains(",") {
                    let split = derivatives.split(",");
                    for s in split {
                        if s.trim() == word {
                            lemma_line.push(lemma.to_string());
                            changed_words.push(word.to_string());
                        }
                    }
                } else if derivatives == word {
                    lemma_line.push(lemma.to_string());
                    changed_words.push(word.to_string());
                }
            }
            if changed_words.contains(&word.to_string()) {
            } else {
                lemma_line.push(word.to_string());
            }
        }
        lemma_string.push(lemma_line.join(" "));
    }
    for string in lemma_string {
        println!("{}", string);
    }
}

pub fn get_words(filename: &str, dict_name: &str, file_output: &str) {
    let reader = BufReader::new(File::open(filename).expect("Cannot open file"));

    let mut word_list: Vec<String> = Vec::new();
    let mut original_text: Vec<String> = Vec::new();

    for line in reader.lines() {
        let mut line_words: Vec<String> = Vec::new();
        let mut unsplit_words: Vec<String> = Vec::new();
        for word in line.unwrap().split_whitespace() {
            unsplit_words.push(word.to_string());
            for split_word in word.split(&[' ', '\''][..]) {
                line_words.push(split_word.to_string());
            }
        }
        word_list.push(line_words.join(" "));
        original_text.push(unsplit_words.join(" "));

    }
    get_lemma(word_list, dict_name, file_output, original_text);
}

pub fn get_lemma(word_list: Vec<String>, dict_name: &str, file_output: &str, original_text: Vec<String>) {
    let dict_lines = vec![dict_name];

    let mut lemma_map: HashMap<String, String> = HashMap::new();
    for dline in dict_lines {
        let mut rdr = csv::Reader::from_path(dline).unwrap();

        for rec in rdr.records() {
            let rr = rec.unwrap();
            let lemma = rr.get(0).unwrap();
            let derivatives = rr.get(1).unwrap();
            lemma_map.insert(lemma.into(), derivatives.into());
        }
    }
    process_data(word_list, lemma_map, file_output, original_text);
}

pub fn process_data(word_list: Vec<String>, lemma_map: HashMap<String, String>, file_output: &str, original_text: Vec<String>) {
    let mut lemma_string: Vec<String> = Vec::new();

    for line in &word_list {
        let mut lemma_line: Vec<String> = Vec::new();
        let mut changed_words: Vec<String> = Vec::new();
        for word in line.split_whitespace() {
            for (lemma, derivatives) in &lemma_map {
                if derivatives.contains(",") {
                    let split = derivatives.split(",");
                    for s in split {
                        if s.trim() == word {
                            lemma_line.push(lemma.to_string());
                            changed_words.push(word.to_string());
                        }
                    }
                } else if derivatives == word {
                    lemma_line.push(lemma.to_string());
                    changed_words.push(word.to_string());
                }
            }
            if changed_words.contains(&word.to_string()) {
            } else {
                lemma_line.push(word.to_string());
            }
        }
        lemma_string.push(lemma_line.join(" "));
    }
    if file_output == "csv" {
        if let Err(e) = output_lemmatized_csv(original_text, lemma_string) {
        eprintln!("{}", e);
        }
    } else {
        output_lemmatized_txt(lemma_string);
    }
}

pub fn output_lemmatized_txt(lemma_string: Vec<String>) {
    let mut writable_lemmas: Vec<String> = Vec::new();
    for word in lemma_string {
        writable_lemmas.push(word);
    }
    fs::write("lemmatized.txt", writable_lemmas.join("\n")).expect("Cannot write");
}

pub fn output_lemmatized_csv(
    original_text: Vec<String>,
    lemma_string: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let combined_vector = original_text.into_iter().zip(lemma_string).collect::<Vec<_>>();

    let mut wtr = csv::Writer::from_path("lemmatized.csv")?;
    wtr.write_record(&["Original Text", "Lemmatized Text"])?;
    for (pair, pair2) in combined_vector {
        wtr.write_record(&[pair, pair2])?;
    }

    wtr.flush()?;
    Ok(())
}
