fn most_frequent_word(text: &str) -> (String, usize) {
    let text = text.to_lowercase();
    let words = text.split_whitespace();
    //array of tuples, first value word, second is count
    let mut found_words: Vec<(String, usize)> = Vec::new();

    for word in words {
        let mut found = false;
        for tuple in found_words.iter_mut() {
            if tuple.0 == word {
                tuple.1 += 1;
                found = true;
                break;
            }
        }

        if !found {
            found_words.push((word.to_string(), 1));
        }
    }
    
    let mut max_word = String::new();
    let mut max_count = 0;
    
    for (word, count) in found_words.iter() {
        if *count > max_count {
            max_word = word.to_string();
            max_count = *count;
        }
    }
    
    return (max_word, max_count); // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
