use std::io;
use crossterm::{execute, terminal};

fn main() {
    println!("Hello, Welcome to Hangman!\n");
    let mut correct_guesses: Vec<char> = Vec::new();
    let mut incorrect_guesses: Vec<char> = Vec::new();
    let random_word = get_random_phrase();
    let correct_letters: Vec<char> = random_word.chars().collect();

    loop {

        println!("correct guess: {:?}", correct_guesses);

        println!("{}", turn_phrase_to_underlines(&random_word, &correct_guesses));

        let mut string_guess = String::new();
        println!("Enter a Letter to Guess\n");

        io::stdin().read_line(&mut string_guess).expect("Failed to read line");

        let letter_guess = string_guess.chars().next();

        match letter_guess {
            Some(guess) if guess.is_alphabetic() => {

                if random_word.contains(guess) {
                    println!("That's Correct! {} is in the word\n", guess);
                    correct_guesses.push(guess);
        
                } else {
                    println!("Unforntunately {} is not in the word\n", guess);
                    incorrect_guesses.push(guess);
                }
            } 
            Some(_) | None => {
                print!("Please Provide a letter\n")
            }        
        }

        if correct_guesses == correct_letters{
            println!("You Win the word is {}", random_word)
        }


        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

    }
}


fn get_random_phrase() -> String {
    "Random Word".to_string()
}

fn turn_phrase_to_underlines(phrase: &String, correct_letters: &Vec<char>) -> String {
    let mut underline_string: Vec<char> = Vec::new();
    let phase_vec: Vec<char> = phrase.chars().collect();
    let space = ' ';

    for char in phase_vec {
        if char == space {
            underline_string.push(space)
        } else if correct_letters.contains(&char) {
            underline_string.push(char)
        } else {
            underline_string.push('_')
        }
    }

    underline_string.into_iter().collect()
}