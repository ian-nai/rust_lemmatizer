# rust_lemmatizer
A lemmatizing package written in Rust.

## How to Use
### On Files
To lemmatize text in a file, run the get_lemmas script using the following arguments. This script saved to a file named "lemmatized.csv" or "lemmatized.txt" depending on user input.

Specify your filename, the path to your csv containing a list of lemmas and their associated forms, and your desired return format (csv, txt, or a Vec<String>).
```
cargo run --bin get_lemmas [YOUR FILENAME] [PATH TO LEMMA FILE] [OUTPUT - txt, csv, or vec]
```
An example command would be:
```
cargo run --bin get_lemmas src/lemma_example.txt src/lemma_dict.csv csv 
```

### On Strings
To lemmatize a string, run the get_lemmas_from_string script. Pass the following command, specifying your string, the file containing the list of lemmas and their forms, and your desired return format (csv, txt, or a Vec<String>).
```
cargo run --bin get_lemmas_from_string ["YOUR STRING"] [PATH TO LEMMA FILE] [OUTPUT - txt, csv, or vec]

```
For example:
```
cargo run --bin get_lemmas_from_string "This is an example string." src/lemma_dict.csv vec
```

### To use in your own project
For lemmatizing a file:
```
use rust_lemmatizer::get_words;

get_words(filename, dict_name, file_output);
```

For lemmatizing on a string:
```
use rust_lemmatizer::get_words_from_string;

get_words_from_string(string_to_analyze, dict_name, return_type);
```


## Lemma List and Formatting
### Source
The list of lemmas included here was sourced from [this GitHub repo](https://github.com/skywind3000/lemma.en) by [Lin Wei.](https://github.com/skywind3000) 

The list was created by referencing the British Nation Corpus (BNC), NodeBox Linguistics and Yasumasa Someya's lemma list.
From the original repo:
>This lemma list is provided "as is" and is free to use for any research and/or educational purposes. The list currently contains 186,523 words (tokens) in 84,487 lemma groups.

### Formatting
To create your own list of lemmas for use with the library, create a csv file formatted like the one included here. Use two columns, the first containing your lemmas and the second containing comma-separated forms of the lemmas. Include a header for each column.


