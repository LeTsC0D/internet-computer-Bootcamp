use std::io;

struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: String::from(text),
        }
    }

    fn count_words(&self) -> usize {
        let words: Vec<&str> = self.text.split_whitespace().collect();
        words.len()
    }
}

fn main() {
    // Prompt the user to enter text
    println!("Enter a text:");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read line");

    // Create an instance of WordCounter
    let word_counter = WordCounter::new(&input_text);

    // Call count_words function
    let word_count = word_counter.count_words();

    // Check for empty text
    if word_count == 0 {
        println!("Error: The entered text is empty.");
    } else {
        // Print the word count
        println!("Word count: {}", word_count);
    }
}
