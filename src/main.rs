use std::{
    fs::File,
    io::{self, BufRead},
};

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_english_words() -> Vec<String> {
    let mut english_words = Vec::new();
    if let Ok(lines) = read_lines("english_words.txt") {
        for line in lines.into_iter() {
            english_words.push(line.unwrap());
        }
    }
    english_words
}

fn count_words(text: &String, english_words: &Vec<String>) -> usize {
    let mut word_matches: usize = 1;
    for word in text.to_uppercase().split(' ') {
        if english_words.contains(&word.to_string()) {
            word_matches += 1;
        }
    }
    word_matches
}

fn is_text_english(text: &String, english_words: &Vec<String>) -> bool {
    let word_matches = count_words(&text, english_words);
    let percentage =
        ((word_matches as f64) / (text.split(' ').collect::<Vec<&str>>().len() as f64)) * 100.0;
    // NOTE: if 70% of the words in the text are english words then it is an english text
    percentage >= 70.0
}

fn main() {
    let english_words = get_english_words();
    let plain_text_in_latin = String::from("Crux Sacra Sit Mihi Lux Non Draco Sit Mihi Dux Vade Retro Satana Nunquam Suade Mihi Vana Sunt Mala Quae Libas Ipse Venena Bibas");
    let plain_text_in_english = String::from("May the Holy Cross be my light Let not the dragon be my guide Begone Satan Never tempt me with your vanities All that pours from you is evil Drink your own poison");
    println!(
        "Latin: {}",
        is_text_english(&plain_text_in_latin, &english_words)
    );
    println!(
        "English: {}",
        is_text_english(&plain_text_in_english, &english_words)
    );
}
