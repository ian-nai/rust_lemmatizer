# rust_lemmatizer
A lemmatizing package written in Rust.

## How to Use
### On Files
To lemmatize text in a file, run the get_lemmas script using the following arguments, specifying your filename, the path to your csv containing lemmas and their associated forms, your desired file format for the output file, and the "file" command to specify that you're passing in a file rather than string:
```
cargo run --bin get_lemmas [YOUR FILENAME] [PATH TO LEMMA FILE] [FILE OUTPUT - txt or csv] file
```
An example command would be:
```
cargo run --bin get_lemmas src/lemma_example.txt src/lemma_dict.csv csv file
```

### Strings
To lemmatize a string, pass a similar command to the one for files listed above. Just sub in your string for the filename and omit the "file" command:
```
cargo run --bin get_lemmas ["YOUR STRING"] [PATH TO LEMMA FILE] [FILE OUTPUT - txt or csv]
```
For example:
```
cargo run --bin get_lemmas "This is an example string." src/lemma_dict.csv csv 
```

## Lemma Source and Formatting
### Source
The list of lemmas included here was sourced from [this GitHub repo](https://github.com/skywind3000/lemma.en)
The list was created by referencing the British Nation Corpus (BNC), NodeBox Linguistics and Yasumasa Someya's lemma list.
From the original repo:
>This lemma list is provided "as is" and is free to use for any research and/or educational purposes. The list currently contains 186,523 words (tokens) >in 84,487 lemma groups.

### Formatting
To create your own list of lemmas for use with the library, create a csv file formatted like the one included here. Use two columns, the first containing your lemmas and the second containing comma-separated forms of the lemmas. Include a header for each column.


