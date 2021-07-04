fn main() {
    let sentence = String::from("This sentence is pig latin");

    for word in sentence.split_whitespace() {
        println!("{}", pig_latinize_word(String::from(word)));
    }
}

// write a function that pig-latinizes words
fn pig_latinize_word(word: String) -> String {
    // vowels are handled differently, so we need an easy way to identify them
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    // split the word into chars so we can do operations on them
    let mut chars = word.chars();

    // we need to do something special to the first character
    match chars.next() {
        Some(letter) => {
            // if first letter is a vowel, append -hay. e.g. apple-hay
            if vowels.contains(&letter) {
                word + "-hay"
            // if first letter is a consonant, take it off and add ay, e.g. pig -> ig-pay
            } else {
                let word: String = chars.collect();
                format!("{}-{}ay", word, letter)
            }
        }
        None => String::from("Unable to ig-pay atinize-lay"),
    }
}
