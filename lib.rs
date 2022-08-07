extern crate csv;

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fs;
use std::error::Error;


pub fn collect_args(filename: &str, dict_name: &str, file_output: &str) {
    get_words(filename, dict_name, file_output);
}

pub fn get_words(filename: &str, dict_name: &str, file_output: &str) {
    let reader = BufReader::new(File::open(filename).expect("Cannot open file.txt"));

    let mut word_list: Vec<String> = Vec::new();

    for line in reader.lines() {
        let mut line_words: Vec<String> = Vec::new();
        for word in line.unwrap().split_whitespace() {
            line_words.push(word.to_string());
            }
        word_list.push(line_words.join(" "));
        }
        get_lemma(word_list, dict_name, file_output);
    }

pub fn get_lemma(word_list: Vec<String>, dict_name: &str, file_output: &str) {
    let files = vec![dict_name];

    let mut lemma_map: HashMap<String, String> = HashMap::new();
    for filename in files {
        let mut rdr = csv::Reader::from_path(filename).unwrap();

        for rec in rdr.records() {
            let rr = rec.unwrap();
            let lemma = rr.get(0).unwrap();
            let derivatives = rr.get(1).unwrap();
            lemma_map.insert(lemma.into(), derivatives.into());
        }
    }
    process_data(word_list, lemma_map, file_output)

}

pub fn process_data(word_list: Vec<String>, lemma_map: HashMap<String, String>, file_output: &str){

        let mut lemma_string: Vec<String> = Vec::new();

        for line in &word_list {
            let mut lemma_line: Vec<String> = Vec::new();
            let mut changed_words: Vec<String> = Vec::new();
            for word in line.split_whitespace() {
                for (lemma, derivatives) in &lemma_map{
                    if derivatives.contains(",") {
                        let split = derivatives.split(",");
                        for s in split {
                            if s.trim() == word {
                                lemma_line.push(lemma.to_string());
                                changed_words.push(word.to_string());
                                }
                            }
                        }
                    else if derivatives == word {
                            lemma_line.push(lemma.to_string());
                            changed_words.push(word.to_string());
                        }
                    }
                if changed_words.contains(&word.to_string()) {
                }
                else {
                    lemma_line.push(word.to_string());
                }
}
    lemma_string.push(lemma_line.join(" "));
}
    if file_output == "csv"{
        output_lemmatized_csv(word_list, lemma_string);
    }
    else {
        println!("{}", file_output);
        output_lemmatized_txt(lemma_string);
    }
}

pub fn output_lemmatized_txt(lemma_string: Vec<String>){
    let mut writable_lemmas: Vec<String> = Vec::new();
    for word in lemma_string{
        writable_lemmas.push(word);
    }
    fs::write("lemmatized.txt", writable_lemmas.join("\n")).expect("Cannot write");
}

pub fn output_lemmatized_csv(word_list: Vec<String>, lemma_string: Vec<String>) -> Result<(), Box<dyn Error>> {
    let combined_vector = word_list.into_iter().zip(lemma_string).collect::<Vec<_>>();

    let mut wtr = csv::Writer::from_path("lemmatized.csv")?;
    wtr.write_record(&["Original Text", "Lemmatized Text"])?;
    for (pair, pair2) in combined_vector {
        wtr.write_record(&[pair, pair2])?;
    }

    wtr.flush();
    Ok(())
}
